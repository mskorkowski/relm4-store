mod indexing;

use reexport::gtk;
use reexport::relm4;
use reexport::log;

use std::cmp::min;
use std::cell::RefCell;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::collections::VecDeque;
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


pub trait Ordering<Record: record::Record>: PartialOrd<Record> {}

/// Configuration trait for the InMemoryBackend
pub trait SortedInMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;

    // fn ordering() -> HashMap<Self::OrderBy, >;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct SortedInMemoryBackend<Builder, Allocator = DefaultIdAllocator> 
where 
    Builder: SortedInMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator>,

    /// Order of profiles
    order: RefCell<VecDeque<Id<Builder::Record>>>,

    /// profile storage
    data: RefCell<HashMap<Id<Builder::Record>, Builder::Record>>,

    handlers: RefCell<HashMap<StoreId<Self, Allocator>, Sender<StoreMsg<Builder::Record>>>>,

    sender: Sender<StoreMsg<Builder::Record>>
}

impl<Builder, Allocator> SortedInMemoryBackend<Builder, Allocator> 
where 
    Builder: SortedInMemoryBackendConfiguration + 'static,
    Allocator: TemporaryIdAllocator + 'static
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Rc<RefCell<Self>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let backend = SortedInMemoryBackend{
            id: StoreId::new(),
            order: RefCell::new(VecDeque::new()),
            data: RefCell::new(HashMap::new()),
            handlers: RefCell::new(HashMap::new()),
            sender,
        };

        let shared_backed = Rc::new(RefCell::new(backend));
        let handler_backend = shared_backed.clone();

        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg:StoreMsg<Builder::Record>| {
                if let Ok(backend) = handler_backend.try_borrow() {
                    match msg {
                        StoreMsg::Commit(record) => {
                            let id = record.get_id();
                            {
                                if id.is_new() {
                                    let position = backend.add(record);
                                    backend.fire_handlers(StoreMsg::NewAt(position));
                                }
                                else {
                                    let mut data = backend.data.borrow_mut();
                                    data.insert(id, record);
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

        for record in Builder::initial_data() {
            shared_backed.borrow().send(StoreMsg::Commit(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        shared_backed
    }

    fn fire_handlers(&self, message: StoreMsg<Builder::Record>) {
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

    fn add(&self, record: Builder::Record) -> Position {
        let id = record.get_id();
        {
            self.data.borrow_mut().insert(id, record.clone());
            let mut order = self.order.borrow_mut();
            order.push_back(id);

            Position(order.len() -1)
        }
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
        let iter = order.range(start..(start+length));

        let mut result: Vec<Self::Record> = Vec::new();

        for id in iter {
            let record = {
                self.data.borrow().get(id).unwrap().clone()
            };

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