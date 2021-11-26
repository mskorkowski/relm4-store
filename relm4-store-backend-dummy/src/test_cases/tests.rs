
mod test_record {
    use reexport::uuid::Uuid;

    use record::Id;
    use record::Record;

    use crate::test_cases::TestRecord;


    #[test]
    fn get_id_is_stable() {
        let record = TestRecord::new("Sample record");

        let id1 = record.get_id();
        assert!(id1.is_new());

        let id2 = record.get_id();
        assert!(id1 == id2);
    }

    #[test]
    fn set_permanent_id() {
        let mut record = TestRecord::new("Sample record");

        let new_id = Uuid::new_v4();
        let permanent_id = Id::<TestRecord>::from(new_id);

        record.set_permanent_id(new_id).expect("Setting permanent id for the first time should work");

        assert!(record.get_id() == permanent_id);
    }

    #[test]
    fn reset_permanent_id_should_fail() {
        let mut record = TestRecord::new("Sample record");

        let new_id = Uuid::new_v4();
        let permanent_id = Id::<TestRecord>::from(new_id);

        record.set_permanent_id(new_id).expect("Setting permanent id for the first time should work");

        assert!(record.get_id() == permanent_id);

        let another_id = Uuid::new_v4();
        
        record.set_permanent_id(another_id).expect_err("This one should fail. You can't set permanent id again");

        assert!(record.get_id() == permanent_id);
    }
}

mod test_cases {
    use store::DataStore;

    use crate::DummyBackend;
    use crate::test_cases::TestCases;
    use crate::test_cases::TestRecord;

    mod empty {
        use store::DataStore;

        use crate::DummyBackend;
        use crate::test_cases::TestCases;
        use crate::test_cases::TestRecord;

        #[test]
        fn empty_0_steps() {
            let c = TestCases::empty(0);
            assert!(c.len() == 0);

            let be = DummyBackend::<TestRecord>::new(c);
            assert!(be.len() ==  0);
        }

        #[test]
        fn empty_2_steps() {
            let c = TestCases::empty(2);
            assert!(c.len() == 2);

            let mut be = DummyBackend::<TestRecord>::new(c);
            assert!(be.len() ==  0);
            be.advance();
            assert!(be.len() == 0);
            be.advance();
            assert!(be.len() == 0);
        }

    }

    #[test]
    fn add_first_record() {
        let c = TestCases::add_first_record();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);
        
        assert!(be.is_empty());
        assert!(be.len() == 0);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 1);
    }

    #[test]
    fn add_second_record_at_the_beginning() {
        let c = TestCases::add_second_record_at_the_beginning();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(!be.is_empty());
        assert!(be.len() == 1);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 2);
    }

    #[test]
    fn add_second_record_at_the_end() {
        let c = TestCases::add_second_record_at_the_end();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(!be.is_empty());
        assert!(be.len() == 1);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 2);
    }

    #[test]
    fn add_third_record_at_the_beginning() {
        let c = TestCases::add_third_record_at_the_beginning();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(!be.is_empty());
        assert!(be.len() == 2);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 3);
    }

    #[test]
    fn add_third_record_in_the_middle() {
        let c = TestCases::add_third_record_in_the_middle();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(!be.is_empty());
        assert!(be.len() == 2);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 3);
    }

    #[test]
    fn add_third_record_at_the_end() {
        let c = TestCases::add_third_record_at_the_end();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(!be.is_empty());
        assert!(be.len() == 2);
        be.advance();
        assert!(!be.is_empty());
        assert!(be.len() == 3);
    }

    #[test]
    fn reload_an_empty_store() {
        let c = TestCases::reload_empty_store();
        assert!(c.len() == 1);
        let mut be = DummyBackend::<TestRecord>::new(c);

        assert!(be.is_empty());
        assert!(be.len() == 0);
        be.advance();
        assert!(be.is_empty());
        assert!(be.len() == 0);
    }
}