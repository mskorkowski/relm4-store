use std::cmp::min;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

use record::DefaultIdAllocator;
use record::Id;
use record::Identifiable;
use record::Record;
use record::TemporaryIdAllocator;
use store::DataStore;
use store::Handler;
use store::Position;
use store::StoreId;
use store::StoreMsg;
use store::math::Range;

/// Configuration trait for the InMemoryBackend
pub trait InMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct InMemoryBackend<Builder, Allocator = DefaultIdAllocator> 
where 
    Builder: InMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator>,

    /// Order of profiles
    order: RefCell<VecDeque<Id<Builder::Record>>>,

    /// profile storage
    data: RefCell<HashMap<Id<Builder::Record>, Builder::Record>>,

    handlers: RefCell<HashMap<StoreId<Self, Allocator>, Box<dyn Handler<Self, Allocator>>>>,
}

impl<Builder, Allocator> InMemoryBackend<Builder, Allocator> 
where 
    Builder: InMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Self {
        let backend = InMemoryBackend{
            id: StoreId::new(),
            order: RefCell::new(VecDeque::new()),
            data: RefCell::new(HashMap::new()),
            handlers: RefCell::new(HashMap::new()),
        };

        for record in Builder::initial_data() {
            backend.inbox(StoreMsg::Commit(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        backend
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

        for (key, handler) in handlers.iter() {
            let remove = handler.handle(message.clone());

            if remove {
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

impl<Builder, Allocator> Identifiable<InMemoryBackend<Builder, Allocator>, Allocator::Type> for InMemoryBackend<Builder, Allocator>
where 
    Builder: InMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Id=StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Builder, Allocator> DataStore<Allocator> for InMemoryBackend<Builder, Allocator>
where 
    Builder: InMemoryBackendConfiguration,
    Allocator: TemporaryIdAllocator,
{
    type Record = Builder::Record;

    fn inbox(&self, msg: StoreMsg<Builder::Record>) {
        match msg {
            StoreMsg::Commit(record) => {
                let id = record.get_id();
                {
                    
                    if id.is_new() {
                        let position = self.add(record);
                        self.fire_handlers(StoreMsg::NewAt(position));
                    }
                    else {
                        let mut data = self.data.borrow_mut();
                        data.insert(id, record);
                        self.fire_handlers(StoreMsg::Update(id))
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

    fn listen<'b>(&self, handler_ref: StoreId<Self, Allocator>, handler: Box<dyn Handler<Self, Allocator>>) {
        self.handlers.borrow_mut().insert(handler_ref, handler);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }
}