use reexport::gtk;
use reexport::relm4;
use reexport::log;

use std::cmp::min;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
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

/// Configuration trait for the InMemoryBackend
pub trait InMemoryBackendConfiguration<Allocator: TemporaryIdAllocator> {
    /// Type of data in the in memory store
    type Record: 'static + Record<Allocator> + Debug + Clone;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;

    // fn ordering() -> HashMap<Self::OrderBy, >;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct InMemoryBackend<Configuration, Allocator, StoreIdAllocator> 
where 
    Configuration: InMemoryBackendConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator, StoreIdAllocator>,

    /// Order of profiles
    order: RefCell<VecDeque<Id<Configuration::Record, Allocator>>>,

    /// profile storage
    data: RefCell<HashMap<Id<Configuration::Record, Allocator>, Configuration::Record>>,

    senders: RefCell<HashMap<StoreId<Self, Allocator, StoreIdAllocator>, Sender<StoreMsg<Configuration::Record, Allocator>>>>,

    sender: Sender<StoreMsg<Configuration::Record, Allocator>>,
}

impl<Configuration, Allocator, StoreIdAllocator> InMemoryBackend<Configuration, Allocator, StoreIdAllocator> 
where 
    Configuration: InMemoryBackendConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
    StoreIdAllocator: TemporaryIdAllocator + 'static,
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Rc<RefCell<Self>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let backend: InMemoryBackend<Configuration, Allocator, StoreIdAllocator> = InMemoryBackend{
            id: StoreId::new(),
            order: RefCell::new(VecDeque::new()),
            data: RefCell::new(HashMap::new()),
            senders: RefCell::new(HashMap::new()),
            sender,
        };

        let shared_backed = Rc::new(RefCell::new(backend));
        let handler_backend = shared_backed.clone();

        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg:StoreMsg<Configuration::Record, Allocator>| {
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

        

        for record in Configuration::initial_data() {
            shared_backed.borrow().inbox(StoreMsg::Commit(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        shared_backed
    }

    fn inbox(&self, msg: StoreMsg<Configuration::Record, Allocator>) {
        log::info!("Received message: {:?}", &msg);
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
                }

            },
            StoreMsg::Reload => {
                //it's in memory store so nothing to do...
            }, 
            _ => {
            }
        }
    }

    fn fire_handlers(&self, message: StoreMsg<Configuration::Record, Allocator>) {
        let mut ids_for_remove: Vec<StoreId<Self, Allocator, StoreIdAllocator>> = Vec::new();

        {
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

            for (key, sender) in senders.iter() {
                if let Err( _ ) =sender.send(message.clone()) {
                    log::warn!("Receiver was cleaned up before dropping sender instance. Dropping sender for {:?}", &key);
                    ids_for_remove.push(*key);
                }
                else {
                    log::info!("Sent message to {:?}", &key);
                }
            }
        } // end of self.senders.borrow(). This way self.unlisten can borrow mutably senders

        // cleanup all handler which decided to remove itself
        for id in ids_for_remove {
            self.unlisten(id);
        }
    }

    fn add(&self, record: Configuration::Record) -> Position {
        let id = record.get_id();
        {
            self.data.borrow_mut().insert(id, record.clone());
            let mut order = self.order.borrow_mut();
            order.push_back(id);

            Position(order.len() -1)
        }
    }
}

impl<Configuration, Allocator, StoreIdAllocator> Identifiable<InMemoryBackend<Configuration, Allocator, StoreIdAllocator>, StoreIdAllocator::Type> for InMemoryBackend<Configuration, Allocator, StoreIdAllocator>
where 
    Configuration: InMemoryBackendConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Id=StoreId<Self, Allocator, StoreIdAllocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Configuration, Allocator, StoreIdAllocator> DataStore<Allocator, StoreIdAllocator> for InMemoryBackend<Configuration, Allocator, StoreIdAllocator>
where 
    Configuration: InMemoryBackendConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Record = Configuration::Record;

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

    fn get(&self, id: &Id<Configuration::Record, Allocator>) -> Option<Configuration::Record> {
        let data = self.data.borrow();
        data.get(id)
            .map(|r| r.clone())
    }

    fn listen<'b>(&self, handler_ref: StoreId<Self, Allocator, StoreIdAllocator>, sender: Sender<StoreMsg<Self::Record, Allocator>>) {
        self.senders.borrow_mut().insert(handler_ref, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator, StoreIdAllocator>) {
        self.senders.borrow_mut().remove(&handler_ref);
    }

    fn send(&self, msg: StoreMsg<Self::Record, Allocator>) {
        log::info!("Sending message via sender: {:?}", msg);
        self.sender.send(msg).expect("Message should be sent, since store exists");
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record, Allocator>> {
        self.sender.clone()
    }
}