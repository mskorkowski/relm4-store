use reexport::gtk;
use reexport::relm4;
use reexport::log;

use std::cmp::min;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use gtk::glib;

use relm4::Sender;

use record::DefaultIdAllocator;
use record::Id;
use record::Identifiable;
use record::Record;
use record::TemporaryIdAllocator;
use store::DataStore;
use store::Position;
use store::StoreId;
use store::StoreMsg;
use store::math::Range;

pub trait Sorter<Record: record::Record>: Copy {
    fn cmp(&self, lhs: &Record, rhs: &Record) -> std::cmp::Ordering;
}

pub trait OrderedStore<OrderBy> {
    fn set_order(&mut self, order: OrderBy);
}

/// Configuration trait for the SortedInMemoryBackend
pub trait SortedInMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;
    type OrderBy: 'static + Sorter<Self::Record> + Copy;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;

    fn initial_order() -> Self::OrderBy;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct SortedInMemoryBackend<Config, Allocator = DefaultIdAllocator> 
where 
    Config: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator>,

    /// Order of profiles
    order: RefCell<Vec<Id<Config::Record>>>,

    /// profile storage
    data: RefCell<HashMap<Id<Config::Record>, Config::Record>>,

    senders: RefCell<HashMap<StoreId<Self, Allocator>, Sender<StoreMsg<Config::Record>>>>,

    sender: Sender<StoreMsg<Config::Record>>,

    ordering: Config::OrderBy
}

impl<Config, Allocator> SortedInMemoryBackend<Config, Allocator> 
where 
    Config: SortedInMemoryBackendConfiguration + 'static,
    Allocator: TemporaryIdAllocator + 'static
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Rc<RefCell<Self>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let backend = SortedInMemoryBackend{
            id: StoreId::new(),
            order: RefCell::new(Vec::new()),
            data: RefCell::new(HashMap::new()),
            senders: RefCell::new(HashMap::new()),
            sender,
            ordering: Config::initial_order(),
        };

        
        let shared_backed = Rc::new(RefCell::new(backend));
        let handler_backend = shared_backed.clone();
        
        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg:StoreMsg<Config::Record>| {
                log::info!("Message received in receiver: {:?}", &msg);
                if let Ok(backend) = handler_backend.try_borrow() {
                    log::info!("Pushing message via inbox!");
                    backend.inbox(msg);
                }
                else {
                    log::warn!("Can't borrow backend. Remember to release the leases");
                }
                glib::Continue(true)
            });
        }
        
        
        let order = Config::initial_order();
        let mut initial_data = Config::initial_data();
        initial_data.sort_by(|lhs, rhs| {
            order.cmp(lhs, rhs)
        });

        for record in initial_data {
            shared_backed.borrow().inbox(StoreMsg::Commit(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        shared_backed
    }

    fn inbox(&self, msg: StoreMsg<Config::Record>) {
        log::info!("Received message: {:?}", &msg);
        match msg {
            StoreMsg::Commit(record) => {
                let id = record.get_id();
                {
                    if id.is_new() || !self.data.borrow().contains_key(&id) {
                        let position = self.insert(record);
                        self.fire_handlers(StoreMsg::NewAt(position));
                    }
                    else {
                        let message = self.update(record);
                        self.fire_handlers(message)
                    }
                }

            },
            StoreMsg::Reload => {
                //it's in memory store so nothing to do...
            }, 
            _ => {
            }
        }
    }

    fn fire_handlers(&self, message: StoreMsg<Config::Record>) {
        let senders = self.senders.borrow();

        if senders.is_empty() {
            log::info!("Senders are empty. Exiting");
            return;
        }

        log::info!("Senders contain {} items", senders.len());

        // tracks store view id's for removal
        //
        // If handler return `true` from `handle` method it should be removed
        //
        // we borrow the self.handlers in the for loop and to do
        // removal we need to borrow again since, unlisten is
        // internally mutable which would cause UB, since we would 
        // iterate over collection which changes itself
        let mut ids_for_remove: Vec<StoreId<Self, Allocator>> = Vec::new();

        for (key, sender) in senders.iter() {
            if let Err( _ ) =sender.send(message.clone()) {
                log::warn!("Receiver was cleaned up before dropping sender instance. Dropping sender for {:?}", &key);
                ids_for_remove.push(*key);
            }
            else {
                log::info!("Sent message to {:?}", &key);
            }
        }

        // cleanup all handler which decided to remove itself
        for id in ids_for_remove {
            self.unlisten(id);
        }
    }

    /// Calls to this method are allowed only if you add nonexisting record to the store
    /// 
    /// Running it for existing record is undefined and might happily destroy your data. 
    fn insert(&self, record: Config::Record) -> Position {
        let id = record.get_id();
        let mut data = self.data.borrow_mut();
        let mut order = self.order.borrow_mut();
        data.insert(id, record.clone());

        log::info!("Order collection before insert has length: {}", order.len());
        log::info!("Order collection before insert");
        for i in order.iter() {
            log::info!("\t {:?}", data.get(i).unwrap());
        }

        let position = {
            let ordering = self.ordering;
            let r = record.clone();
            order.binary_search_by(|other_id| {
                let other = data.get(other_id).unwrap();
                ordering.cmp(&r, other)
            })
        };

        let pos = match position {
            // if two elements are equal it doesn't matter which one is first
            Ok(p) => {
                order.insert(p, id);
                Position(p)
            },
            Err(p) => {
                order.insert(p, id);
                Position(p)
            }
        };

        log::info!("Order collection after insert has length: {}", order.len());
        log::info!("Order collection after insert");
        for i in order.iter() {
            log::info!("\t {:?}", data.get(i).unwrap());
        }

        pos
    }

    /// Calls to this method are allowed only if record is already in the store
    /// 
    /// Running if you run it for nonexisting record this method is undefined. It might end up with panic or
    /// killing your cat, or flooding, or erupting volcanos, or whatever other disaster you might think of. 
    fn update(&self, record: Config::Record) -> StoreMsg<Config::Record> {
        let id = record.get_id();
        // record is already in store => it's safe to unwrap
        let old_record = self.data.borrow_mut().insert(id, record.clone()).unwrap();

        let mut order = self.order.borrow_mut();
        let old_position: usize = {
            let data = self.data.borrow();
            let ordering = self.ordering;
            let position = order.binary_search_by(|other_id| {
                let other = data.get(other_id).unwrap();
                ordering.cmp(&old_record, other)
            });

            match position {
                Ok(p) => p,
                Err(_) => panic!("Record doesn't exist in order while it's in the data! Performing seppuku!")
            }
        };


        let position = {
            let data = self.data.borrow();
            let ordering = self.ordering;
            let r = record.clone();
            order.binary_search_by(|other_id| {
                let other = data.get(other_id).unwrap();
                ordering.cmp(&r, other)
            })
        };

        let (to, mut from, p) = match position {
            Ok(p) => {
                // if two elements are equal it doesn't matter which one is first
                if p == old_position {
                    (p, old_position, StoreMsg::Update(id))
                }
                else if p < old_position {
                    (old_position, p, StoreMsg::Move{from: Position(old_position), to: Position(p)} )
                }
                else {
                    (p, old_position, StoreMsg::Move{from: Position(old_position), to: Position(p)} )
                }
            },
            Err(p) => {
                if p == old_position {
                    (p, old_position, StoreMsg::Update(id))
                }
                else if p < old_position {
                    (old_position, p, StoreMsg::Move{from: Position(old_position), to: Position(p)} )
                }
                else {
                    (p, old_position, StoreMsg::Move{from: Position(old_position), to: Position(p)} )
                }
            }
        };

        if from != to {
            // perform minimum reorder of view for the case when records where updated
            while to >= from {
                order[from] = order[from-1];
                from -= 1;
            }
            order[to] = id;
        }

        p
    }
}

impl<Builder, Allocator> Identifiable<SortedInMemoryBackend<Builder, Allocator>, Allocator::Type> for SortedInMemoryBackend<Builder, Allocator>
where 
    Builder: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Id=StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Builder, Allocator> DataStore<Allocator> for SortedInMemoryBackend<Builder, Allocator>
where 
    Builder: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Record = Builder::Record;
 
    fn len(&self) -> usize {
        self.data.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_range(&self, range: &Range) -> Vec<Self::Record> {
        let count = self.len();

        let start = min(*range.start(), count);
        let length = min(*range.end(), count) - start;

        let order = self.order.borrow();
        let data = self.data.borrow();

        let mut result: Vec<Self::Record> = Vec::new();

        for idx in start..(start+length) {
            let id = &order[idx];
            let record = data.get(id).unwrap().clone();
            result.push(record);
        }

        result
    }

    fn get(&self, id: &Id<Builder::Record>) -> Option<Builder::Record> {
        let data = self.data.borrow();
        data.get(id)
            .map(|r| r.clone())
    }

    fn listen<'b>(&self, handler_ref: StoreId<Self, Allocator>, sender: Sender<StoreMsg<Self::Record>>) {
        self.senders.borrow_mut().insert(handler_ref, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>) {
        self.senders.borrow_mut().remove(&handler_ref);
    }

    fn send(&self, msg: StoreMsg<Self::Record>) {
        log::info!("Sending message via sender: {:?}", msg);
        self.sender.send(msg).expect("Message should be sent, since store exists");
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record>> {
        self.sender.clone()
    }
}