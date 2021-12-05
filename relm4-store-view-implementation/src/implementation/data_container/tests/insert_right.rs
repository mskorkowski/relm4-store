//! Contains tests for `[DataContainer::insert_right]`

use backend_dummy::test_cases::TestRecord;
use record::Record;
use record::DefaultIdAllocator;

use crate::WindowChangeset;
use crate::implementation::data_container::DataContainer;


#[test]
fn insert_right_first_record() {
    let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
    let record: TestRecord = TestRecord::constant("First record to add");
    let records: Vec<TestRecord> = vec![record.clone()];
    let position: usize = 0;

    dc.insert_right(&mut changeset, position, records);

    dc.invariants();
    assert_eq!(dc.data[&record.get_id()], record);
    assert_eq!(dc.order[0], record.get_id());
}