use backend_dummy::test_cases::TestRecord;
use record::Record;

use crate::WindowChangeset;
use crate::data_container::DataContainer;

use super::test_data::TestData;

#[test]
fn remove_left_from_empty_container() {
    let mut dc: DataContainer<TestRecord> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

    let left_records = vec![];
    let right_records = vec![];

    dc.remove_left(&mut changeset, 3, 1, left_records, right_records);

    assert_eq!(dc.len(), 0);
}

#[test]
fn remove_last_element() {
    let TestData{ records, mut container } = TestData::new(1, 10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

    let left_records = vec![];
    let right_records = vec![];

    assert_eq!(records.len(), 1);
    assert_eq!(container.len(), 1);

    container.remove_left(&mut changeset, 1, 1, left_records, right_records);

    assert_eq!(container.len(), 0);
    assert!(changeset.remove_contains(&records[0].get_id()));
}

mod records_3 {
    use super::Record;
    use super::TestData;
    use super::TestRecord;
    use super::WindowChangeset;

    const RECORDS_CNT: usize = 3;
    const MAX_SIZE: usize = 10;

    #[test]
    fn remove_first_element_insert_0_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records, right_records);

        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
    }

    
    
    #[test]
    fn remove_second_element_insert_0_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        
        let left_records = vec![];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);
        
        container.remove_left(&mut changeset, 2, 1, left_records, right_records);
        
        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
    }
    
    #[test]
    fn remove_last_element_insert_0_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records, right_records);

        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
    }

    ///---[5]----

    #[test]
    fn remove_first_element_insert_1_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1)
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records);

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.remove_contains(&records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_1_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1)
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records);

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
    }

    #[test]
    fn remove_last_element_insert_1_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1)
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records);
        
        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
    }

    #[test]
    fn remove_first_element_insert_2_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records);

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert!(!container.data.contains_key(&left_records[0].get_id()));
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
    }

    #[test]
    fn remove_second_element_insert_2_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records);

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
    }

/// ---[ 10 ]----

    #[test]
    fn remove_last_element_insert_2_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];
        let right_records = vec![];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records);

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
    }

    #[test]
    fn remove_first_element_insert_0_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_0_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_last_element_insert_0_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_first_element_insert_1_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

/// ---[ 15 ]----

    #[test]
    fn remove_second_element_insert_1_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_last_element_insert_1_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_first_element_insert_2_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_2_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

    #[test]
    fn remove_last_element_insert_2_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
    }

/// ---[ 20 ]------

    #[test]
    fn remove_first_element_insert_0_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert_eq!(container.order[3], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_second_element_insert_0_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert_eq!(container.order[3], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_last_element_insert_0_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], right_records[0].get_id());
        assert_eq!(container.order[3], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_first_element_insert_1_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_second_element_insert_1_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

/// ---[ 25 ]----

    #[test]
    fn remove_last_element_insert_1_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1)
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[0].get_id()], left_records[0]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[0].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_first_element_insert_2_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 1, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_second_element_insert_2_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 2, 1, left_records.clone(), right_records.clone());

        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[2].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    /// ----------------------------------
    /// ----------------------------------
    /// ----------------------------------
    /// ----------------------------------
    /// ----------------------------------
    /// ----------------------------------
    /// ----------------------------------

    #[test]
    fn remove_last_element_insert_2_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let left_records = vec![
            TestRecord::since("Added left record 1", 1),
            TestRecord::since("Added left record 2", 1),
        ];
        let right_records = vec![
            TestRecord::since("Added right record 1", 1),
            TestRecord::since("Added right record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_left(&mut changeset, 3, 1, left_records.clone(), right_records.clone());
        assert_eq!(container.len(), 5);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&left_records[1].get_id()], left_records[1]);
        assert_eq!(container.data[&right_records[0].get_id()], right_records[0]);
        assert_eq!(container.data[&right_records[1].get_id()], right_records[1]);
        assert_eq!(container.order[0], left_records[1].get_id());
        assert_eq!(container.order[1], records[0].get_id());
        assert_eq!(container.order[2], records[1].get_id());
        assert_eq!(container.order[3], right_records[0].get_id());
        assert_eq!(container.order[4], right_records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
        assert!(changeset.add_contains(&left_records[1].get_id()));
        assert!(changeset.add_contains(&right_records[0].get_id()));
        assert!(changeset.add_contains(&right_records[1].get_id()));
    }

    #[test]
    fn remove_all_non_full() {
        let TestData{ records: _, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![];

        container.remove_left(&mut changeset, RECORDS_CNT, RECORDS_CNT, left_records, right_records);

        assert_eq!(container.len(), 0);
    }

    #[test]
    fn remove_all_full() {
        let TestData{ records: _, mut container } = TestData::new(MAX_SIZE, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        let left_records = vec![];
        let right_records = vec![];

        container.remove_left(&mut changeset, MAX_SIZE, MAX_SIZE, left_records, right_records);

        assert_eq!(container.len(), 0);
    }
}