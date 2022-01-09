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

use store::Backend;
use store::Store;
use store::Replies;
use std::fmt::Debug;
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
pub struct DummyBackend<Record> 
where
    Record: 'static + record::Record + Debug + Clone,
{
    configuration: DummyBackendConfiguration<Record>,
    index: usize,
    initiated: bool,
}

impl<Record> DummyBackend<Record> 
where
    Record: 'static + record::Record + Debug + Clone,
{
    /// Creates new instance of this structure
    pub fn new(configuration: DummyBackendConfiguration<Record>) -> Self {
        Self{
            configuration,
            index: 0,
            initiated: false,
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
}

impl<Record> Backend for DummyBackend<Record>
where 
    Record: 'static + record::Record + Debug + Clone,
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

    fn get(&self, id: &record::Id<Self::Record>) -> Option<Self::Record> {
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

        for record in v.iter().take(last_idx).skip(*range.start()) {
            result.push(record.clone());
        }

        result
    }

    fn inbox(&mut self, _msg: StoreMsg<Self::Record>) -> store::Replies<Self::Record> {
        Replies{
            replies: vec!()
        }
    }

}

/// Trait providing advance method
/// 
/// It makes using `DummyBackend` easier since you can implement this trait for store used
/// in tests, in a way which will call the `advance` method for the [DummyBackend]
pub trait StepByStepStore {
    /// Advances dummy backend to the next step
    fn advance(&mut self);
}

impl<Record> StepByStepStore for Store<DummyBackend<Record>> 
where
    Record: record::Record + std::fmt::Debug
{
    fn advance(&mut self) {
        let be = self.backend();
        let mut backend = be.borrow_mut();
        backend.advance();

        self.fire_handlers(&backend.configuration[backend.index].events);
    }
}