use store::Position;
use store::StoreViewMsg;

use crate::DummyBackendConfiguration;
use crate::configuration::Step;

use super::TestCase;
use super::TestRecord;


/// TestCaseBuilder
#[derive(Debug)]
pub struct TestCaseBuilder {
    test_case: TestCase
}

impl TestCaseBuilder {
    /// Creates "size amount of initial data"
    pub fn initial_size(mut self, size: usize) -> Self {
        let mut initial_data = Vec::with_capacity(size);

        for idx in 0..size {
            initial_data.push(TestRecord::constant(&format!("Initial test record {}", idx)))
        }

        self.test_case.data = initial_data.clone();
        self.test_case.configuration.initial_data = initial_data;

        self
    }

    /// Adds a step
    pub fn add_step(mut self) -> Self {
        let last_step_data = {
            let steps = &self.test_case.configuration.steps;
            if steps.len() > 0 {
                steps[steps.len() - 1].data.clone()
            }
            else {
                self.test_case.configuration.initial_data.clone()
            }
        };


        self.test_case.configuration.steps.push(Step{
            data: last_step_data,
            events: vec![],
        });

        self
    }

    /// Slides a given step to the position
    /// 
    /// You must add a step before running this method
    pub fn slide(mut self, position: usize) -> Self {
        let step = self.test_case.configuration.steps.len() - 1;

        self.test_case.configuration.steps[step].events.push(StoreViewMsg::SlideTo(Position(position)));

        self
    }

    /// Returns constructed test case
    pub fn build(self) -> TestCase {
        self.test_case
    }
}

impl Default for TestCaseBuilder{
    fn default() -> Self {
        TestCaseBuilder{
            test_case: TestCase{
                configuration: DummyBackendConfiguration{
                    steps: vec![],
                    initial_data: vec![],
                },
                data: vec![],
            }
        }
    }
}