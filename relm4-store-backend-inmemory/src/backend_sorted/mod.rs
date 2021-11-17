mod key;

use reexport::gtk;
use reexport::relm4;
use reexport::log;

use std::cmp::min;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;

use log::warn;

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

use key::Key;

pub trait Sorter<Record: record::Record> {
    fn eq(lhs: &Record, rhs: &Record) -> bool;
    fn cmp(lhs: &Record, rhs: &Record) -> std::cmp::Ordering;
    fn new(record: &Record) -> Self;
}

pub trait OrderedStore<OrderBy> {
    fn set_order(&mut self, order: OrderBy);
}

/// Configuration trait for the InMemoryBackend
pub trait SortedInMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;
    type OrderBy: 'static + Sorter<Self::Record> + Ord + Clone;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;

    fn initial_order() -> Self::OrderBy;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct SortedInMemoryBackend<'me, Config, Allocator = DefaultIdAllocator> 
where 
    Config: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator>,

    data: BTreeMap<Config::OrderBy, Config::Record>,

    handlers: RefCell<HashMap<StoreId<Self, Allocator>, Sender<StoreMsg<Config::Record>>>>,

    sender: Sender<StoreMsg<Config::Record>>,

    order_by: Config::OrderBy,
}

impl<'me, Config, Allocator> SortedInMemoryBackend<'me, Config, Allocator> 
where 
    Config: SortedInMemoryBackendConfiguration + 'static,
    Allocator: TemporaryIdAllocator + 'static
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Rc<RefCell<SortedInMemoryBackend<'static, Config, Allocator>>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let backend = SortedInMemoryBackend{
            id: StoreId::new(),
            data: BTreeMap::new(),
            handlers: RefCell::new(HashMap::new()),
            sender,
            order_by: Config::initial_order(),
        };

        let shared_backed = Rc::new(RefCell::new(backend));
        let handler_backend = shared_backed.clone();

        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg:StoreMsg<Config::Record>| {
                if let Ok(mut backend) = handler_backend.try_borrow_mut() {
                    match msg {
                        StoreMsg::Commit(record) => {
                            let id = record.get_id();
                            let key = Config::OrderBy::new(&record);
                            {
                                if id.is_new() || !backend.data.contains_key(&key){
                                    let position = backend.add(record);
                                    backend.fire_handlers(StoreMsg::NewAt(position));
                                }
                                else {
                                    backend.data.insert(key, record);
                                    backend.fire_handlers(StoreMsg::Update(id))
                                }
                                // let old_record = data.get(&id);
                            }
            
                        },
                        StoreMsg::Reload => {
                            //it's in memory store so nothing to do...
                        }, 
                        _ => {
                        }
                    }
                }
                else {
                    warn!("Can't borrow backend. Remember to release the leases");
                }
                glib::Continue(true)
            });
        }

        for record in Config::initial_data() {
            shared_backed.borrow().send(StoreMsg::Commit(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        shared_backed
    }

    fn fire_handlers(&self, message: StoreMsg<Config::Record>) {
        let handlers = self.handlers.borrow();

        if handlers.is_empty() {
            return;
        }

        // tracks store view id's for removal
        //
        // If handler return `true` from `handle` method it should be removed
        //
        // we borrow the self.handlers in the for loop and to do
        // removal we need to borrow again since, unlisten is
        // internally mutable which would cause UB, since we would 
        // iterate over collection which changes itself
        let mut ids_for_remove: Vec<StoreId<Self, Allocator>> = Vec::new();

        for (key, sender) in handlers.iter() {
            if let Err( _ ) =sender.send(message.clone()) {
                ids_for_remove.push(*key);
            }
        }

        // cleanup all handler which decided to remove itself
        for id in ids_for_remove {
            self.unlisten(id);
        }
    }

    fn add(&mut self, record: Config::Record) -> Position {
        let key = Config::OrderBy::new(&record);
        let id = record.get_id();
        {
            self.data.insert(key.clone(), record.clone());

            Position(key)
        }
    }
}

impl<'me, Config, Allocator> Identifiable<SortedInMemoryBackend<'me, Config, Allocator>, Allocator::Type> for SortedInMemoryBackend<'me, Config, Allocator>
where 
    Config: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Id=StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<'me, Config, Allocator> DataStore<Allocator> for SortedInMemoryBackend<'me, Config, Allocator>
where 
    Config: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Record = Config::Record;

    

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_range(&self, range: &Range) -> Vec<Self::Record> {
        let count = self.len();

        let start = min(*range.start(), count);
        let length = min(*range.end(), count) - start;

        let iter = self.data.range(start..(start+length));

        let mut result: Vec<Self::Record> = Vec::new();

        for id in iter {
            let record = {
                self.data.get(id).unwrap().clone()
            };

            result.push(record);
        }

        result
    }

    fn get(&self, id: &Id<Config::Record>) -> Option<Config::Record> {
        self.data.get(id)
            .map(|r| r.clone())
    }

    fn listen<'b>(&self, handler_ref: StoreId<Self, Allocator>, sender: Sender<StoreMsg<Self::Record>>) {
        self.handlers.borrow_mut().insert(handler_ref, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }

    fn send(&self, msg: StoreMsg<Self::Record>) {
        self.sender.send(msg).expect("Message should be sent, since store exists");
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record>> {
        self.sender.clone()
    }
}

impl<'me, Config, Allocator> OrderedStore<Config::OrderBy> for SortedInMemoryBackend<'me, Config, Allocator> 
where
    Config: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    fn set_order(&mut self, order: Config::OrderBy) {
        self.order_by = order;


    }
}