
use store::Position;
use store::StoreViewMsg;

use crate::configuration::Step;

use super::TestCase;
use super::TestCases;

///Empty store test cases
impl TestCases {
    /// Returns test case removing last record
    pub fn remove_last() -> TestCase {
        TestCases::remove_nth(0, 1)
    }

    /// Returns test case removing first record out of two
    pub fn remove_first_of_two() -> TestCase {
        TestCases::remove_nth(0, 2)
    }

    /// Returns test case removing second record out of two
    pub fn remove_second_of_two() -> TestCase {
        TestCases::remove_nth(1, 2)
    }

    /// Returns test case removing first record out of three
    pub fn remove_first_of_three() -> TestCase {
        TestCases::remove_nth(0, 3)
    }

    /// Returns test case removing second record out of three
    pub fn remove_second_of_three() -> TestCase {
        TestCases::remove_nth(1, 3)
    }

    /// Returns test case removing third record out of three
    pub fn remove_third_of_three() -> TestCase {
        TestCases::remove_nth(2, 3)
    }

    /// Creates test case where `nth` record will be removed from store of given `size`
    pub fn remove_nth(nth: usize, size: usize) -> TestCase {
        let TestCase{ mut configuration, data } = TestCases::with_initial_size(size);

        let mut new_data = data.clone();
        new_data.remove(nth);

        configuration.steps.push(Step{
            data: new_data,
            events: vec![
                StoreViewMsg::Remove(Position(nth))
            ]
        });

        TestCase {
            configuration,
            data
        }
    }
}