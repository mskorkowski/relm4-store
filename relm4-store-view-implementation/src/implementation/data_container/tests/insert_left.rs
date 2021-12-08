//! Contains tests for `[DataContainer::insert_left]`

use backend_dummy::test_cases::TestRecord;
use record::Record;
use record::DefaultIdAllocator;

use crate::WindowChangeset;
use crate::implementation::data_container::DataContainer;

use super::test_data::TestData;

#[test]
fn insert_left_first_record_at_0() {
    let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
    let record: TestRecord = TestRecord::constant("First record to add");
    let records: Vec<TestRecord> = vec![record.clone()];
    let position: usize = 0;

    dc.insert_left(&mut changeset, position, records);

    dc.invariants();
    assert_eq!(dc.len(), 0);
    assert_eq!(dc.data.len(), 0);
    assert_eq!(dc.order.len(), 0);
}

#[test]
fn insert_left_first_record_at_1() {
    let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
    let record: TestRecord = TestRecord::constant("First record to add");
    let records: Vec<TestRecord> = vec![record.clone()];
    let position: usize = 1;

    dc.insert_left(&mut changeset, position, records);

    dc.invariants();
    assert_eq!(dc.len(), 1);
    assert_eq!(dc.data[&record.get_id()], record);
    assert_eq!(dc.order[0], record.get_id());
}

#[test]
fn insert_left_second_two_records() {
    let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
    let record1: TestRecord = TestRecord::constant("First record to add");
    let record2: TestRecord = TestRecord::constant("Second record to add");
    let records: Vec<TestRecord> = vec![record1.clone(), record2.clone()];
    let position: usize = 2;

    dc.insert_left(&mut changeset, position, records);

    dc.invariants();
    assert_eq!(dc.len(), 2);
    assert_eq!(dc.data[&record1.get_id()], record1);
    assert_eq!(dc.data[&record2.get_id()], record2);
    assert_eq!(dc.order[0], record1.get_id());
    assert_eq!(dc.order[1], record2.get_id());
}

#[test]
fn insert_left_three_records() {
    let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
    let record1: TestRecord = TestRecord::constant("First record to add");
    let record2: TestRecord = TestRecord::constant("Second record to add");
    let record3: TestRecord = TestRecord::constant("Third record to add");
    let records: Vec<TestRecord> = vec![record1.clone(), record2.clone(), record3.clone()];
    let position: usize = 3;

    dc.insert_left(&mut changeset, position, records);

    dc.invariants();
    assert_eq!(dc.len(), 3);
    assert_eq!(dc.data[&record1.get_id()], record1);
    assert_eq!(dc.data[&record2.get_id()], record2);
    assert_eq!(dc.data[&record3.get_id()], record3);
    assert_eq!(dc.order[0], record1.get_id());
    assert_eq!(dc.order[1], record2.get_id());
    assert_eq!(dc.order[2], record3.get_id());
}

mod max_size_3 {
    //! Tests various inserts into data container with 3 elements

    use backend_dummy::test_cases::TestRecord;
    use record::Record;
    use record::DefaultIdAllocator;
    
    use crate::WindowChangeset;

    use super::TestData;

    const RECORDS_CNT: usize = 3;
    const MAX_SIZE: usize = 3;
    
    mod at_at_0 {
        
        use super::DefaultIdAllocator;
        use super::Record;
        use super::TestRecord;
        use super::TestData;
        use super::WindowChangeset;

        use super::RECORDS_CNT;
        use super::MAX_SIZE;

        #[test]
        fn add_at_max_elements_0() {
            let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_records = vec![];
            let position = records.len();

            container.insert_left(&mut changeset, position, new_records);

            assert_eq!(container.len(), 3);

            assert_eq!(container.data[&records[0].get_id()], records[0]);
            assert_eq!(container.data[&records[1].get_id()], records[1]);
            assert_eq!(container.data[&records[2].get_id()], records[2]);

            assert_eq!(container.order[0], records[0].get_id());
            assert_eq!(container.order[1], records[1].get_id());
            assert_eq!(container.order[2], records[2].get_id());
        }


        /// `|1, 2, 3| -> |2, 3, 4|`
        #[test]
        fn add_at_max_elements_1() {
            let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_record = TestRecord::since("Record added at pos 0", 1);
            let new_records = vec![new_record.clone()];
            let position = records.len();

            container.insert_left(&mut changeset, position, new_records);

            assert_eq!(container.len(), 3);

            assert_eq!(container.data[&new_record.get_id()], new_record);
            assert_eq!(container.data[&records[1].get_id()], records[1]);
            assert_eq!(container.data[&records[2].get_id()], records[2]);

            assert_eq!(container.order[0], records[1].get_id());
            assert_eq!(container.order[1], records[2].get_id());
            assert_eq!(container.order[2], new_record.get_id());
        }

        /// `|1, 2, 3| -> |3, 4, 5|
        #[test]
        fn add_at_max_elements_2() {
            let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_record1 = TestRecord::since("Record added at pos 0 - 0", 1);
            let new_record2 = TestRecord::since("Record added at pos 0 - 1", 1);
            let new_records = vec![new_record1.clone(), new_record2.clone()];
            let position = records.len();

            
            container.insert_left(&mut changeset, position, new_records);
            
            assert_eq!(container.len(), 3);
            
            assert_eq!(container.data[&new_record1.get_id()], new_record1);
            assert_eq!(container.data[&new_record2.get_id()], new_record2);
            assert_eq!(container.data[&records[2].get_id()], records[2]);
            
            assert_eq!(container.order[0], records[2].get_id());
            assert_eq!(container.order[1], new_record1.get_id());
            assert_eq!(container.order[2], new_record2.get_id());
        }
        
        #[test]
        fn add_at_max_elements_3() {
            let TestData{ mut container, records } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_record1 = TestRecord::since("Record added at pos 0 - 0", 1);
            let new_record2 = TestRecord::since("Record added at pos 0 - 1", 1);
            let new_record3 = TestRecord::since("Record added at pos 0 - 2", 1);
            let new_records = vec![new_record1.clone(), new_record2.clone(), new_record3.clone()];
            let position = records.len();
            
            container.insert_left(&mut changeset, position, new_records);
            
            assert_eq!(container.len(), 3);
            
            assert_eq!(container.data[&new_record1.get_id()], new_record1);
            assert_eq!(container.data[&new_record2.get_id()], new_record2);
            assert_eq!(container.data[&new_record3.get_id()], new_record3);
            
            assert_eq!(container.order[0], new_record1.get_id());
            assert_eq!(container.order[1], new_record2.get_id());
            assert_eq!(container.order[2], new_record3.get_id());
        }
        
        #[test]
        fn add_at_max_elements_4() {
            let TestData{ mut container, records } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_record1 = TestRecord::since("Record added at pos 0 - 0", 1);
            let new_record2 = TestRecord::since("Record added at pos 0 - 1", 1);
            let new_record3 = TestRecord::since("Record added at pos 0 - 2", 1);
            let new_record4 = TestRecord::since("Record outside of the view - 1", 1);
            let new_records = vec![
                new_record1.clone(), 
                new_record2.clone(), 
                new_record3.clone(), 
                new_record4.clone()
            ];
            let position = records.len();

            container.insert_left(&mut changeset, position, new_records);
            
            assert_eq!(container.len(), 3);
            
            assert_eq!(container.data[&new_record2.get_id()], new_record2);
            assert_eq!(container.data[&new_record3.get_id()], new_record3);
            assert_eq!(container.data[&new_record4.get_id()], new_record4);
            
            assert_eq!(container.order[0], new_record2.get_id());
            assert_eq!(container.order[1], new_record3.get_id());
            assert_eq!(container.order[2], new_record4.get_id());
        }

        #[test]
        fn add_at_max_elements_5() {
            let TestData{ mut container, records } = TestData::new(RECORDS_CNT, MAX_SIZE);
            let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
            let new_record1 = TestRecord::since("Record added at pos 0 - 0", 1);
            let new_record2 = TestRecord::since("Record added at pos 0 - 1", 1);
            let new_record3 = TestRecord::since("Record added at pos 0 - 2", 1);
            let new_record4 = TestRecord::since("Record outside of the view - 1", 1);
            let new_record5 = TestRecord::since("Record outside of the view - 2", 1);
            let new_records = vec![
                new_record1.clone(), 
                new_record2.clone(), 
                new_record3.clone(), 
                new_record4.clone(),
                new_record5.clone(),
            ];
            let position = records.len();

            container.insert_left(&mut changeset, position, new_records);

            assert_eq!(container.len(), 3);

            assert_eq!(container.data[&new_record3.get_id()], new_record3);
            assert_eq!(container.data[&new_record4.get_id()], new_record4);
            assert_eq!(container.data[&new_record5.get_id()], new_record5);

            assert_eq!(container.order[0], new_record3.get_id());
            assert_eq!(container.order[1], new_record4.get_id());
            assert_eq!(container.order[2], new_record5.get_id());
        }
    }

    #[test]
    fn add_at_1() {
        
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
        let new_record = TestRecord::since("Record added at pos 0", 1);
        let new_records = vec![new_record.clone()];
        let position = 1;

        container.insert_left(&mut changeset, position, new_records);

        assert_eq!(container.len(), 3);

        assert_eq!(container.data[&new_record.get_id()], new_record);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);

        assert_eq!(container.order[0], new_record.get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
    }

    #[test]
    fn add_at_2() {   
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();
        let new_record = TestRecord::since("Record added at pos 1", 1);
        let new_records = vec![new_record.clone()];
        let position = 2;

        container.insert_left(&mut changeset, position, new_records);

        assert_eq!(container.len(), 3);

        assert_eq!(container.data[&new_record.get_id()], new_record);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);

        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], new_record.get_id());
        assert_eq!(container.order[2], records[2].get_id());
    }
}