
use store::Position;
use store::StoreViewMsg;

use crate::configuration::Step;
use crate::test_cases::TestRecord;

use super::C;
use super::TestCase;
use super::TestCases;



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

    /// returns configuration which adds 1 new record at nth position
    pub fn add_nth(size: usize, nth: usize, count: usize) -> TestCase {
        let mut tc = TestCases::with_initial_size(size);

        let mut result_data = tc.data.clone();
        let mut events: Vec<StoreViewMsg<TestRecord>> = vec![];

        for idx in 0..count {
            let record = TestRecord::since(&format!("Added test record {}", idx+1), 1);
            if nth+idx == result_data.len() {
                result_data.push(record.clone());
            }
            else {
                result_data.insert(nth+idx, record.clone());
            }
            tc.data.push(record);
            events.push(StoreViewMsg::NewAt(Position(nth+idx)));
        }

        

        tc.configuration.steps.push(Step{
            data: result_data,
            events,
        });

        tc
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
                    events: vec![StoreViewMsg::NewAt(Position(0))]
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
                    events: vec![StoreViewMsg::NewAt(Position(1))]
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
                    events: vec![StoreViewMsg::NewAt(Position(0))]
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
                    events: vec![StoreViewMsg::NewAt(Position(2))]
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
                    events: vec![StoreViewMsg::NewAt(Position(1))]
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
                    events: vec![StoreViewMsg::NewAt(Position(0))]
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
                    events: vec![StoreViewMsg::Reload],
                }
            ]
        };

        TestCase{
            configuration,
            data: vec![],
        }
    }
}