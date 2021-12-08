use backend_dummy::test_cases::TestRecord;
use record::Record;
use record::DefaultIdAllocator;

use crate::WindowChangeset;
use crate::implementation::data_container::DataContainer;


#[cfg(test)]
pub(crate) struct TestData {
    pub(crate) records: Vec<TestRecord>,
    pub(crate) container: DataContainer<TestRecord, DefaultIdAllocator>,
}

#[cfg(test)]
impl TestData {
    pub(crate) fn new(records_cnt: usize, max_size: usize) -> TestData {
        let mut dc: DataContainer<TestRecord, DefaultIdAllocator> = DataContainer::new(max_size);
        let mut records: Vec<TestRecord> = Vec::with_capacity(records_cnt);
        let mut changeset: WindowChangeset<TestRecord, DefaultIdAllocator> = WindowChangeset::default();

        for idx in 0..records_cnt {
            let r = TestRecord::constant(&format!("Record #{}", idx+1));
            records.push(r);
        }

        dc.insert_right(&mut changeset, 0, records.clone());

        assert_eq!(dc.len(), records_cnt);
        for idx in 0..records_cnt {
            let r = &records[idx];
            assert_eq!(dc.data[&r.get_id()], records[idx], "Data at idx `{}` should be equal to record at the same position", idx);
            assert_eq!(dc.order[idx], r.get_id());
        }

        TestData{
            records: records.clone(), 
            container: dc,
        }
    }
}