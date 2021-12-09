//! Test cases with initial state holding 2 elements

use store::Position;
use store::StoreMsg;

use crate::configuration::Step;
use crate::test_cases::TestRecord;

use super::C;
use super::TestCase;
use super::TestCases;

impl TestCases {
    /// Returns the [TestCase] with just records being added.
    /// 
    /// In steps records are added in specified order so order [3, 5] is different then [5, 3]. First case
    /// `0, 1, 2, 3, 4, 5, 6 --[3']--> 0, 1, 2, 3', 3, 4, 5, 6 --[5']--> 0, 1, 2, 3', 3, 5', 4, 5, 6` and second
    /// case is
    /// `0, 1, 2, 3, 4, 5, 6 --[5']--> 0, 1, 2, 3, 4, 5', 5, 6 --[3']--> 0, 1, 2, 3', 3, 4, 5', 5, 6`
    /// 
    /// This method is not performing any validation, wherever programmer has any sanity left. If you ask
    /// it to do something stupid in terms of data definition you will receive it. That's by design since
    /// I (author) have a dream of checking behavior of the store view against at least some pathological
    /// inputs
    /// 
    /// PS in case of impossible data this method may panic (ex. add 7th record to 3 element data store)
    pub fn multistep_add_unsafe(initial: usize, add_at: Vec<Vec<usize>>) -> TestCase {
        let mut data = Vec::new();
        let mut initial_data = Vec::with_capacity(initial);
        let mut steps = Vec::new();

        let mut last_step = Vec::new();

        for idx in 0..initial {
            let r = TestRecord::constant(&format!("Initial record {}", idx));

            initial_data.push(r.clone());
            data.push(r.clone());
            last_step.push(r);
        }

        for (step, insertion_points) in add_at.iter().enumerate() {
            let mut events = Vec::new();

            for insertion_point in insertion_points {
                let r = TestRecord::since(&format!("Record inserted at pos `{}` in step `{}`", insertion_point, step), step);
                let event = StoreMsg::NewAt(Position(*insertion_point));

                events.push(event);
                data.push(r.clone());

                if *insertion_point == last_step.len() {
                    last_step.push(r);
                }
                else {
                    last_step.insert(*insertion_point, r)
                }

            }

            let step_data = last_step.clone();

            steps.push(Step{
                data: step_data,
                events,
            });
        }

        let configuration = C {
            steps,
            initial_data,
        };
        
        TestCase{
            configuration,
            data,
        }
    }
}