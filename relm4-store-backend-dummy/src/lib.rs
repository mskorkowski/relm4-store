//! Really simple data store
//! 
//! It has been created with the idea of testing the code using [store::DataStore] and [store::StoreView]

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod configuration;
pub mod test_cases;

#[cfg(test)]
mod tests;

use reexport::gtk;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use gtk::glib;

use record::Identifiable;
use record::TemporaryIdAllocator;
use reexport::relm4::Sender;
use store::DataStore;
use store::StoreId;
use store::StoreMsg;


pub use configuration::DummyBackendConfiguration;

/// Allows to tell where at which step dummy store is
#[derive(Debug, PartialEq, Eq)]
pub enum DummyStoreStep {
    /// Store is in initial state and was never advanced
    Initial,
    /// Store is at given step
    Step(usize),
}

/// Dummy store
#[derive(Debug)]
pub struct DummyBackend<Record, Allocator, StoreIdAllocator> 
where
    Record: 'static + record::Record<Allocator> + Debug + Clone, 
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator
{
    configuration: DummyBackendConfiguration<Record, Allocator>,
    index: usize,
    initiated: bool,
    id: StoreId<Self, Allocator, StoreIdAllocator>,
    senders: RefCell<HashMap<StoreId<Self, Allocator, StoreIdAllocator>, Sender<StoreMsg<Record, Allocator>>>>,
    sender: Sender<StoreMsg<Record, Allocator>>,
    _allocator: PhantomData<Allocator>,
}

impl<Record, Allocator, StoreIdAllocator> DummyBackend<Record, Allocator, StoreIdAllocator> 
where
    Record: 'static + record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Creates new instance of this structure
    pub fn new(configuration: DummyBackendConfiguration<Record, Allocator>) -> Self {
        let (sender, _receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        Self{
            configuration,
            index: 0,
            initiated: false,
            id: StoreId::new(),
            senders: RefCell::new(HashMap::new()),
            sender,
            _allocator: PhantomData,
        }
    }

    /// Advances state of the store
    pub fn advance(&mut self) {
        if self.initiated {
            self.index += 1;
        }
        else {
            self.initiated = true;
            self.index = 0;
        }

        if self.index >= self.configuration.len() {
            panic!("Trying to advance above the configuration");
        }

        let mut ids_for_remove: Vec<StoreId<Self, Allocator, StoreIdAllocator>> = Vec::new();

        {
            let senders = self.senders.borrow();

            if senders.is_empty() {
                return;
            }


            for message in &self.configuration[self.index].events {
                for (key, sender) in senders.iter() {
                    if let Err( _ ) = sender.send(message.clone()) {
                        ids_for_remove.push(*key);
                    }
                }
            }
        } // end of senders borrow

        for id in ids_for_remove {
            self.unlisten(id);
        }
    }

    /// returns information at which step this store is
    pub fn step(&self) -> DummyStoreStep {
        if self.initiated {
            DummyStoreStep::Step(self.index)
        }
        else {
            DummyStoreStep::Initial
        }
    }

    /// Returns number of registered listeners
    pub fn listeners_len(&self) -> usize {
        self.senders.borrow().len()
    }
}

impl<Record, Allocator, StoreIdAllocator> Identifiable<Self, StoreIdAllocator::Type> for DummyBackend<Record, Allocator, StoreIdAllocator> 
where
    Record: 'static + record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Id = StoreId<Self, Allocator, StoreIdAllocator>;

    #[cfg(not(tarpaulin_include))]
    fn get_id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl<Record, Allocator, StoreIdAllocator> DataStore<Allocator, StoreIdAllocator> for DummyBackend<Record, Allocator, StoreIdAllocator>
where 
    Record: 'static + record::Record<Allocator> + Debug + Clone,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Record = Record;

    fn len(&self) -> usize {
        if !self.initiated {
            self.configuration.initial_data.len()
        }
        else {
            self.configuration[self.index].data.len()
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, id: &record::Id<Self::Record, Allocator>) -> Option<Self::Record> {
        if !self.initiated {
            self.configuration.initial_data.iter().find_map(|r|{
                if r.get_id() == *id {
                    Some(r.clone())
                }
                else {
                    None
                }
            })
        }
        else {
            self.configuration[self.index].data.iter().find_map(|r|{
                if r.get_id() == *id {
                    Some(r.clone())
                }
                else {
                    None
                }
            })
        }
    }

    fn get_range(&self, range: &store::math::Range) -> Vec<Self::Record> {
        let v = if !self.initiated {
            &self.configuration.initial_data
        }
        else {
            &self.configuration[self.index].data
        };

        let mut result = if range.len() >= v.len() {
            //This protects against unlimited store view size
            Vec::with_capacity(v.len())
        }
        else {
            Vec::with_capacity(range.len())
        };

        
        if *range.start() >= v.len() {
            return vec![]
        }

        let last_idx = std::cmp::min(*range.end(), v.len());

        for idx in *range.start()..last_idx {
            result.push(v[idx].clone());
        }

        result
    }

    fn listen(&self, id: StoreId<Self, Allocator, StoreIdAllocator>, sender: reexport::relm4::Sender<StoreMsg<Self::Record, Allocator>>) {
        self.senders.borrow_mut().insert(id, sender);
    }

    fn unlisten(&self, id: StoreId<Self, Allocator, StoreIdAllocator>) {
        self.senders.borrow_mut().remove(&id);
    }

    /// Receiver of this sender is not attached to anything, sending messages via this sender is not going
    /// to trigger any events
    /// 
    /// This makes the state of this store easier to predict, since you trigger advancement of the state by hand
    fn sender(&self) -> Sender<StoreMsg<Self::Record, Allocator>> {
        // coverage of this code is meaningless, this sender is dumb
        self.sender.clone()
    }

    /// This method is sending messages via sender whose receiver is not connected, so it won't do a lot
    fn send(&self, msg: StoreMsg<Self::Record, Allocator>) {
        // coverage of this code is meaningless, this sender is dumb
        self.sender.send(msg).expect("Message should be sent, since store exists");
    }
}

