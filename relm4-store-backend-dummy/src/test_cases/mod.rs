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
    added: Option<usize>,
    removed: Option<usize>,
}

impl TestRecord {
    /// Creates instance of the test record which is expected to live through out whole test
    pub fn constant(label: &str) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: None,
            removed: None,
        }
    }

    /// Creates instance of the test record which is expected to be added at given step
    /// 
    /// This record is expected to be present in the data till the end of the test
    pub fn since(label: &str, step: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: Some(step),
            removed: None,
        }
    }

    /// Create instance of the test record which is expected to be present until given step (exclusive)
    /// 
    /// This record is expected to be present since initial data to given step
    pub fn until(label: &str, step: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: None,
            removed: Some(step),
        }
    }

    /// Creates instance of the test record which is expected to be in data in steps belonging to a range `[added, removed)`
    pub fn between(label: &str, added: usize, removed: usize) -> Self {
        TestRecord{
            id: Id::new(),
            label: String::from(label),
            added: Some(added),
            removed: Some(removed),
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

/// Describes test case
#[derive(Debug)]
pub struct TestCase {
    /// Configuration for the [DummyBackend]
    pub configuration: C,
    /// Data used by the test case
    pub data: Vec<TestRecord>,
}

///Empty store test cases
impl TestCases {
    /// returns configuration of empty store which contains given number of steps (all empty)
    pub fn empty(num_steps: usize) -> TestCase {
        let mut steps = Vec::with_capacity(num_steps);

        for _ in 0..num_steps {
            steps.push(Step{
                data: vec![],
                events: vec![],
            });
        }

        let configuration = C{
            initial_data: vec![],
            steps,
        };

        TestCase{
            configuration,
            data: vec![],
        }
    }
    
    /// returns configuration which has 0 steps and initial data has `size` of records
    pub fn with_initial_size(size: usize) -> TestCase {
        let mut initial_data = Vec::with_capacity(size);

        for idx in 0..size {
            initial_data.push(TestRecord::constant(&format!("TestRecord {}", idx)));
        }

        let configuration = C{
            initial_data: initial_data.clone(),
            steps: vec![],
        };

        TestCase {
            configuration,
            data: initial_data,
        }
    }

    /// Add single record to the empty store
    /// 
    /// `[] -> [r1]`
    pub fn add_first_record() -> TestCase {
        let r1 = TestRecord::since("r1", 0);
        let configuration = C {
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![r1.clone(),],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1,]
        }
    }

    /// Add second record at the end of the store
    /// 
    /// `[r1] -> [r1, r2]`
    pub fn add_second_record_at_the_end() -> TestCase {
        let r1 = TestRecord::constant("r1");
        let r2 = TestRecord::since("r2", 0);

        let configuration = C{
            initial_data: vec![r1.clone()],
            steps: vec![
                Step{
                    data: vec![r1.clone(), r2.clone()],
                    events: vec![StoreMsg::NewAt(Position(1))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1, r2],
        }
    }

    /// Add second record at the beginning of the store
    /// 
    /// `[r1] -> [r2, r1]`
    pub fn add_second_record_at_the_beginning() -> TestCase {
        let r1 = TestRecord::constant("r1");
        let r2 = TestRecord::since("r2", 0);

        let configuration = C{
            initial_data: vec![r1.clone()],
            steps: vec![
                Step{
                    data: vec![r2.clone(), r1.clone()],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1, r2],
        }
    }

    /// Add third record at the end of the store
    /// 
    /// `[r1, r2] -> [r1, r2, r3]`
    pub fn add_third_record_at_the_end() -> TestCase {
        let r1 = TestRecord::constant("r1");
        let r2 = TestRecord::constant("r2");
        let r3 = TestRecord::since("r3", 0);

        let configuration = C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r1.clone(), r2.clone(), r3.clone()],
                    events: vec![StoreMsg::NewAt(Position(2))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1, r2, r3],
        }
    }

    /// Add third record in the middle of the store
    /// 
    /// `[r1, r2] -> [r1, r3, r2]`
    pub fn add_third_record_in_the_middle() -> TestCase {
        let r1 = TestRecord::constant("r1");
        let r2 = TestRecord::constant("r2");
        let r3 = TestRecord::since("r3", 0);

        let configuration = C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r1.clone(), r3.clone(), r2.clone()],
                    events: vec![StoreMsg::NewAt(Position(1))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1, r2, r3],
        }
    }

    /// Add third record at the beginning of the store
    /// 
    /// `[r1, r2] -> [r3, r1, r2]`
    pub fn add_third_record_at_the_beginning() -> TestCase {
        let r1 = TestRecord::constant("r1");
        let r2 = TestRecord::constant("r2");
        let r3 = TestRecord::since("r3", 0);

        let configuration = C{
            initial_data: vec![r1.clone(), r2.clone()],
            steps: vec![
                Step{
                    data: vec![r3.clone(), r1.clone(), r2.clone()],
                    events: vec![StoreMsg::NewAt(Position(0))]
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![r1, r2, r3],
        }
    }

    /// Reload an empty store
    /// 
    /// `[] ---[ Reload ]---> []`
    pub fn reload_empty_store() -> TestCase {
        let configuration = C{
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![],
                    events: vec![StoreMsg::Reload],
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![],
        }
    }
}