

mod test_cases {
    use record::Id;
    use record::Record;
    use store::Backend;
    use store::math::Range;

    use crate::DummyBackend;
    use crate::DummyBackendConfiguration;
    use crate::DummyStoreStep;
    use crate::configuration::Step;
    use crate::test_cases::TestCase;
    use crate::test_cases::TestCases;
    use crate::test_cases::TestRecord;

    #[test]
    #[should_panic]
    fn advancing_over_configuration_capacity_should_panic() {
        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration{
            initial_data: vec![],
            steps: vec![],
        };
        let mut be = DummyBackend::<TestRecord>::new(c);

        be.advance();
    }

    #[test]
    fn more_then_one_step_in_store() {
        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration{
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![],
                    events: vec![],
                },
                Step{
                    data: vec![],
                    events: vec![],
                }
            ],
        };

        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(be.step() == DummyStoreStep::Initial);
        be.advance();
        assert!(be.step() == DummyStoreStep::Step(0));
        be.advance();
        assert!(be.step() == DummyStoreStep::Step(1));
    }  

    #[test]
    fn get_record_by_id_from_empty_initial_state() {
        let id = Id::new();

        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![],
            steps: vec![], 
        };

        let be = DummyBackend::<TestRecord>::new(c);

        let result = be.get(&id);
        assert!(result.is_none());
    }

    #[test]
    fn get_first_record_by_id_from_initial_state() {
        let record = TestRecord::constant("Sample record");
        let id = record.get_id();

        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![
                record.clone()
            ],
            steps: vec![], 
        };

        let be = DummyBackend::<TestRecord>::new(c);

        let result = be.get(&id).expect("Record must be present");
        assert!(id == result.get_id());
    }

    #[test]
    fn get_nonfirst_record_by_id_from_initial_state() {
        let first_record = TestRecord::constant("Record to be skipped while searching");
        let record = TestRecord::constant("Sample record");
        let id = record.get_id();

        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![
                first_record,
                record.clone()
            ],
            steps: vec![], 
        };

        let be = DummyBackend::<TestRecord>::new(c);

        let result = be.get(&id).expect("Record must be present");
        assert!(id == result.get_id());
    }

    #[test]
    fn get_record_by_id_from_an_empty_step() {
        let id = Id::new();
        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![],
                    events: vec![],
                }
            ], 
        };

        let mut be = DummyBackend::<TestRecord>::new(c);
        be.advance();

        let result = be.get(&id);
        assert!(result.is_none());
    }

    #[test]
    fn get_first_record_by_id_from_a_nonempty_step() {
        let record = TestRecord::constant("record to be found");
        let id = record.get_id();

        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![record],
                    events: vec![],
                }
            ], 
        };

        let mut be = DummyBackend::<TestRecord>::new(c);
        be.advance();

        let result = be.get(&id).expect("Record should exist");
        assert!(result.get_id() == id);
    }

    #[test]
    fn get_nonfirst_record_by_id_from_a_nonempty_step() {
        let first_record = TestRecord::constant("record to be skipped");
        let record = TestRecord::constant("record to be found");
        let id = record.get_id();

        let c: DummyBackendConfiguration<TestRecord> = DummyBackendConfiguration { 
            initial_data: vec![],
            steps: vec![
                Step{
                    data: vec![first_record, record],
                    events: vec![],
                }
            ], 
        };

        let mut be = DummyBackend::<TestRecord>::new(c);
        be.advance();

        let result = be.get(&id).expect("Record should exist");
        assert!(result.get_id() == id);
    }

    #[test]
    fn get_range_from_empty_data_store_in_an_initial_state() {
        let TestCase{configuration, data: _} = TestCases::empty(0);
        let be = DummyBackend::<TestRecord>::new(configuration);

        let result = be.get_range(&Range::new(10, 20));

        assert!(result.len() == 0);
    }

    #[test]
    fn get_range_in_the_content_range_of_an_initial_state() {
        let TestCase{configuration, data:_} = TestCases::with_initial_size(15);
        let be = DummyBackend::<TestRecord>::new(configuration);

        let range = Range::new(5, 10);
        let result = be.get_range(&range);
        assert!(result.len() == range.len());
    }

    #[test]
    fn get_range_partially_in_the_content_range_of_an_initial_state() {
        let size = 15;
        let TestCase{configuration, data:_} = TestCases::with_initial_size(size);
        let be = DummyBackend::<TestRecord>::new(configuration);

        let range = Range::new(10, 22);
        let result = be.get_range(&range);
        assert!(result.len() == size - *range.start());
    }

    #[test]
    fn get_range_from_empty_data_store_in_not_an_initial_state() {
        let TestCase{configuration, data:_} = TestCases::empty(1);
        let mut be = DummyBackend::<TestRecord>::new(configuration);
        be.advance();
        let result = be.get_range(&Range::new(10, 20));

        assert!(result.len() == 0);
    }
}