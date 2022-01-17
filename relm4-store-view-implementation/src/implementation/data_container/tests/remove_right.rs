use backend_dummy::test_cases::TestRecord;
use record::Record;

use crate::WindowChangeset;
use crate::implementation::data_container::DataContainer;

use super::test_data::TestData;


#[test]
fn remove_right_from_empty_container() {
    let mut dc: DataContainer<TestRecord> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

    let records = vec![];

    dc.remove_right(&mut changeset, 0, 3, records);
    dc.invariants();

    assert_eq!(dc.len(), 0);
}

#[test]
fn remove_right_from_empty_container_outside_of_the_range() {
    let mut dc: DataContainer<TestRecord> = DataContainer::new(10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

    let records = vec![];

    dc.remove_right(&mut changeset, 5, 3, records);
    dc.invariants();

    assert_eq!(dc.len(), 0);
}

#[test]
fn remove_last_element() {
    let TestData{ records, mut container } = TestData::new(1, 10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

    assert_eq!(records.len(), 1);
    assert_eq!(container.len(), 1);

    container.remove_right(&mut changeset, 0, 1, vec![]);

    assert_eq!(container.len(), 0);
    assert!(changeset.remove_contains(&records[0].get_id()));
}


mod records_3{
    use super::Record;
    use super::TestData;
    use super::TestRecord;
    use super::WindowChangeset;

    const RECORDS_CNT: usize = 3;
    const MAX_SIZE: usize = 10;

    #[test]
    fn remove_first_element_insert_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 0, 1, vec![]);

        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 1, 1, vec![]);

        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
    }

    #[test]
    fn remove_last_element_insert_0() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 2, 1, vec![]);

        assert_eq!(container.len(), 2);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
    }

    #[test]
    fn remove_first_element_insert_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1)
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 0, 1, new_data.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1)
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 1, 1, new_data.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
    }

    #[test]
    fn remove_last_element_insert_1() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1)
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 2, 1, new_data.clone());

        assert_eq!(container.len(), 3);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
    }

    #[test]
    fn remove_first_element_insert_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 0, 1, new_data.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.data[&new_data[1].get_id()], new_data[1]);
        assert_eq!(container.order[0], records[1].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert_eq!(container.order[3], new_data[1].get_id());
        assert!(changeset.remove_contains(&records[0].get_id()));
    }

    #[test]
    fn remove_second_element_insert_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 1, 1, new_data.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[2].get_id()], records[2]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.data[&new_data[1].get_id()], new_data[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[2].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert_eq!(container.order[3], new_data[1].get_id());
        assert!(changeset.remove_contains(&records[1].get_id()));
    }

    #[test]
    fn remove_last_element_insert_2() {
        let TestData{ records, mut container } = TestData::new(RECORDS_CNT, MAX_SIZE);
        let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
        let new_data = vec![
            TestRecord::since("Added record 1", 1),
            TestRecord::since("Added record 2", 1),
        ];

        assert_eq!(records.len(), 3);
        assert_eq!(container.len(), 3);

        container.remove_right(&mut changeset, 2, 1, new_data.clone());

        assert_eq!(container.len(), 4);
        assert_eq!(container.data[&records[0].get_id()], records[0]);
        assert_eq!(container.data[&records[1].get_id()], records[1]);
        assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
        assert_eq!(container.data[&new_data[1].get_id()], new_data[1]);
        assert_eq!(container.order[0], records[0].get_id());
        assert_eq!(container.order[1], records[1].get_id());
        assert_eq!(container.order[2], new_data[0].get_id());
        assert_eq!(container.order[3], new_data[1].get_id());
        assert!(changeset.remove_contains(&records[2].get_id()));
    }
}

#[test]
fn remove_last_element_and_insert_too_many_records() {
    let TestData{ records, mut container } = TestData::new(10, 10);
    let mut changeset: WindowChangeset<TestRecord> = WindowChangeset::default();
    let new_data = vec![
        TestRecord::since("Added record 1", 1),
        TestRecord::since("Added record 2", 1),
    ];

    assert_eq!(records.len(), 10);
    assert_eq!(container.len(), 10);

    container.remove_right(&mut changeset, 9, 1, new_data.clone());

    assert_eq!(container.len(), 10);
    assert_eq!(container.data[&new_data[0].get_id()], new_data[0]);
    assert!(!container.data.contains_key(&new_data[1].get_id()));
    assert_eq!(container.order[9], new_data[0].get_id());
    assert!(!container.order.contains(&new_data[1].get_id()));
    assert!(changeset.remove_contains(&records[9].get_id()));
}