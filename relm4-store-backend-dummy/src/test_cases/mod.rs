//! Contains various configurations for the dummy data store ready for use in your tests

#[cfg(test)]
mod tests;
use reexport::uuid;


use store::Position;
use uuid::Uuid;

use store::StoreMsg;
use record::Id;
use record::Record;

use crate::configuration::Step;
use super::DummyBackendConfiguration;

/// Sample record for test cases
#[derive(Debug, Clone)]
pub struct TestRecord {
    id: Id<Self>,
    label: String,
}

impl TestRecord {
    /// Creates instance of the test record
    pub fn new(label: &str) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
        }
    }
}

impl Record for TestRecord {
    fn get_id(&self) -> record::Id<Self> {
        self.id.clone()
    }

    fn set_permanent_id(&mut self, value: Uuid) -> Result<(), record::IdentityError> {
        if !self.id.is_new() {
            Err(record::IdentityError("TestRecord already has permanent id"))
        }
        else {
            self.id = Id::from(value);
            Ok( () )
        }
    }
}

/// Contains various test case configurations for the DummyStore
#[derive(Debug)]
pub struct TestCases {}

type C = DummyBackendConfiguration<TestRecord>;

///Empty store test cases
impl TestCases {
    /// returns configuration of empty store which contains given number of steps (all empty)
    pub fn empty(num_steps: usize) -> C {
        let mut steps = Vec::with_capacity(num_steps);

        for _ in 0..num_steps {
            steps.push(Step{
                data: vec![],
                events: vec![],
            });
        }

        C{
            initial_data: vec![],
            steps,
        }
    }
    
    /// returns configuration which has 0 steps and initial data has `size` of records
    pub fn with_initial_size(size: usize) -> C {
        let mut initial_data = Vec::with_capacity(size);

        for idx in 0..size {
            initial_data.push(TestRecord::new(&format!("TestRecord {}", idx)));
        }

        C{
            initial_data,
            steps: vec![],
        }
    }

    /// Add single record to the empty store
    /// 
    /// `[] -> [r1]`
    pub fn add_first_record() -> C {
        let r1 = TestRecord::new("r1");
        C {
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![r1,],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        }
    }

    /// Add second record at the end of the store
    /// 
    /// `[r1] -> [r1, r2]`
    pub fn add_second_record_at_the_end() -> C {
        let r1 = TestRecord::new("r1");
        let r2 = TestRecord::new("r2");

        C{
            initial_data: vec![r1.clone()],
            steps: vec![
                Step{
                    data: vec![r1, r2],
                    events: vec![StoreMsg::NewAt(Position(1))]
                }
            ]
        }
    }

    /// Add second record at the beginning of the store
    /// 
    /// `[r1] -> [r2, r1]`
    pub fn add_second_record_at_the_beginning() -> C {
        let r1 = TestRecord::new("r1");
        let r2 = TestRecord::new("r2");

        C{
            initial_data: vec![r1.clone()],
            steps: vec![
                Step{
                    data: vec![r2, r1],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        }
    }

    /// Add third record at the end of the store
    /// 
    /// `[r1, r2] -> [r1, r2, r3]`
    pub fn add_third_record_at_the_end() -> C {
        let r1 = TestRecord::new("r1");
        let r2 = TestRecord::new("r2");
        let r3 = TestRecord::new("r3");

        C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r1, r2, r3],
                    events: vec![StoreMsg::NewAt(Position(2))]
                }
            ]
        }
    }

    /// Add third record in the middle of the store
    /// 
    /// `[r1, r2] -> [r1, r3, r2]`
    pub fn add_third_record_in_the_middle() -> C {
        let r1 = TestRecord::new("r1");
        let r2 = TestRecord::new("r2");
        let r3 = TestRecord::new("r3");

        C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r1, r3, r2],
                    events: vec![StoreMsg::NewAt(Position(1))]
                }
            ]
        }
    }

    /// Add third record at the beginning of the store
    /// 
    /// `[r1, r2] -> [r3, r1, r2]`
    pub fn add_third_record_at_the_beginning() -> C {
        let r1 = TestRecord::new("r1");
        let r2 = TestRecord::new("r2");
        let r3 = TestRecord::new("r3");

        C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r3, r1, r2],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        }
    }

    /// Reload an empty store
    /// 
    /// `[] ---[ Reload ]---> []`
    pub fn reload_empty_store() -> C {
        C{
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![],
                    events: vec![StoreMsg::Reload],
                }
            ]
        }
    }
}