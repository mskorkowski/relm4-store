use dummy::test_cases::TestRecord;
use record::DefaultIdAllocator;

use relm4_store_backend_inmemory::SortedInMemoryBackend;
use relm4_store_backend_inmemory::SortedInMemoryBackendConfiguration;
use relm4_store_backend_inmemory::Sorter;

pub type TestRecordsBase<Config> = SortedInMemoryBackend<Config, DefaultIdAllocator, DefaultIdAllocator>;

#[derive(Clone, Copy, Debug)]
pub enum OrderTestRecordsBy {
    Name{ascending: bool},
}

impl Sorter<TestRecord, DefaultIdAllocator> for OrderTestRecordsBy {
    fn cmp(&self, lhs: &TestRecord, rhs: &TestRecord) -> std::cmp::Ordering {
        match self {
            OrderTestRecordsBy::Name{ascending} => {
                if *ascending {
                    lhs.label.cmp(&rhs.label)
                }
                else {
                    lhs.label.cmp(&rhs.label).reverse()
                }
            },
        }
    }
}

pub struct TestRecordsConfigDescEmpty {}
impl SortedInMemoryBackendConfiguration<DefaultIdAllocator> for TestRecordsConfigDescEmpty
{
    type Record = TestRecord;
    type OrderBy = OrderTestRecordsBy;

    fn initial_data() -> Vec<Self::Record> {
        vec![]
    }

    fn initial_order() -> Self::OrderBy {
        OrderTestRecordsBy::Name{ascending: false}
    }
}

pub struct TestRecordsConfigDesc8 {}
impl SortedInMemoryBackendConfiguration<DefaultIdAllocator> for TestRecordsConfigDesc8
{
    type Record = TestRecord;
    type OrderBy = OrderTestRecordsBy;

    fn initial_data() -> Vec<Self::Record> {
        vec![
            TestRecord::constant("r"),
            TestRecord::constant("f"),
            TestRecord::constant("i"),
            TestRecord::constant("c"),
            TestRecord::constant("o"),
            TestRecord::constant("y"),
            TestRecord::constant("l"),
            TestRecord::constant("u"),
        ]
    }

    fn initial_order() -> Self::OrderBy {
        OrderTestRecordsBy::Name{ascending: false}
    }
}

pub struct TestRecordsConfigAscEmpty {}
impl SortedInMemoryBackendConfiguration<DefaultIdAllocator> for TestRecordsConfigAscEmpty
{
    type Record = TestRecord;
    type OrderBy = OrderTestRecordsBy;

    fn initial_data() -> Vec<Self::Record> {
        vec![]
    }

    fn initial_order() -> Self::OrderBy {
        OrderTestRecordsBy::Name{ascending: true}
    }
}

pub struct TestRecordsConfigAsc8 {}
impl SortedInMemoryBackendConfiguration<DefaultIdAllocator> for TestRecordsConfigAsc8
{
    type Record = TestRecord;
    type OrderBy = OrderTestRecordsBy;

    fn initial_data() -> Vec<Self::Record> {
        vec![
            TestRecord::constant("r"),
            TestRecord::constant("f"),
            TestRecord::constant("i"),
            TestRecord::constant("c"),
            TestRecord::constant("o"),
            TestRecord::constant("y"),
            TestRecord::constant("l"),
            TestRecord::constant("u"),
        ]
    }

    fn initial_order() -> Self::OrderBy {
        OrderTestRecordsBy::Name{ascending: true}
    }
}

#[cfg(test)]
mod tests {

    mod order_test_record_by {
        /// OrderTestRecordBy must behave in accordance to the "convention of" [Ordering::cmp]
        mod ordering {
            use std::cmp::Ordering;

            use dummy::test_cases::TestRecord;

            use relm4_store_backend_inmemory::Sorter;
            use super::super::super::OrderTestRecordsBy;

            #[test]
            fn less_then() {
                let l = TestRecord::constant("A");
                let r = TestRecord::constant("B");

                let ordering = OrderTestRecordsBy::Name{ascending: true};
                assert_eq!(ordering.cmp(&l, &r), Ordering::Less)
            }

            #[test]
            fn equal() {
                let l = TestRecord::constant("A");
                let r = TestRecord::constant("A");

                let ordering = OrderTestRecordsBy::Name{ascending: true};
                assert_eq!(ordering.cmp(&l, &r), Ordering::Equal)
            }

            #[test]
            fn greater_then() {
                let l = TestRecord::constant("B");
                let r = TestRecord::constant("A");

                let ordering = OrderTestRecordsBy::Name{ascending: true};
                assert_eq!(ordering.cmp(&l, &r), Ordering::Greater)
            }
        }
    }
}