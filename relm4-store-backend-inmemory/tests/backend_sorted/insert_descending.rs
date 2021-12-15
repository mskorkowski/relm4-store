
mod empty {
    use record::Record;
    use reexport::glib;
    use reexport::gtk;

    use serial_test::serial;

    use dummy::test_cases::TestRecord;
    use store::DataStore;
    use store::Store;
    use store::StoreMsg;
    use store::math::Range;

    use crate::common::TestRecordsBase;
    use crate::common::TestRecordsConfigDescEmpty;

    type TestRecords = TestRecordsBase<TestRecordsConfigDescEmpty>;

    #[test]
    #[serial(gtk)]
    fn insert_first() {
        gtk::init().unwrap();
        let context = glib::MainContext::default();
        let _guard = context.acquire().unwrap();

        let backend: TestRecords = TestRecords::new();
        let store: Store<TestRecords> = Store::new(backend);
        let new_record = TestRecord::since("First record to be added", 0).permanent();
        store.send(StoreMsg::Commit(new_record.clone()));
        
        assert_eq!(store.len(), 0, "Store should be empty at the beginning");
        context.iteration(true);
        assert_eq!(store.len(), 1, "Now store should have one element");

        let record_from_store = store.get(&new_record.get_id());
        assert_eq!(record_from_store, Some(new_record.clone()));

        let data = vec![new_record];
        let range_from_store = store.get_range(&Range::new(0, 10));
        assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
    }

    #[test]
    #[serial(gtk)]
    fn insert_second_asc() {
        gtk::init().unwrap();
        let context = glib::MainContext::default();
        let _guard = context.acquire().unwrap();

        let backend: TestRecords = TestRecords::new();
        let store: Store<TestRecords> = Store::new(backend);
        let new_record1 = TestRecord::since("A - First record to be added", 0).permanent();
        let new_record2 = TestRecord::since("B - Second record to be added", 0).permanent();
        store.send(StoreMsg::Commit(new_record1.clone()));
        store.send(StoreMsg::Commit(new_record2.clone()));
        
        assert_eq!(store.len(), 0, "Store should be empty at the beginning");
        context.iteration(true);
        assert_eq!(store.len(), 2, "Now store should have two elements");

        let record1_from_store = store.get(&new_record1.get_id());
        let record2_from_store = store.get(&new_record2.get_id());
        assert_eq!(record1_from_store, Some(new_record1.clone()));
        assert_eq!(record2_from_store, Some(new_record2.clone()));

        let data = vec![new_record2, new_record1];
        let range_from_store = store.get_range(&Range::new(0, 10));
        assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
    }


    #[test]
    #[serial(gtk)]
    fn insert_second_desc() {
        gtk::init().unwrap();
        let context = glib::MainContext::default();
        let _guard = context.acquire().unwrap();

        let backend: TestRecords = TestRecords::new();
        let store: Store<TestRecords> = Store::new(backend);
        let new_record1 = TestRecord::since("A - First record to be added", 0).permanent();
        let new_record2 = TestRecord::since("B - Second record to be added", 0).permanent();
        store.send(StoreMsg::Commit(new_record2.clone()));
        store.send(StoreMsg::Commit(new_record1.clone()));
        
        assert_eq!(store.len(), 0, "Store should be empty at the beginning");
        context.iteration(true);
        assert_eq!(store.len(), 2, "Now store should have two elements");

        let record1_from_store = store.get(&new_record1.get_id());
        let record2_from_store = store.get(&new_record2.get_id());
        assert_eq!(record1_from_store, Some(new_record1.clone()));
        assert_eq!(record2_from_store, Some(new_record2.clone()));

        let data = vec![new_record2, new_record1];
        let range_from_store = store.get_range(&Range::new(0, 10));
        assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
    }

    mod single_step {

        use record::Record;
        use reexport::glib;
        use reexport::gtk;

        use serial_test::serial;

        use dummy::test_cases::TestRecord;
        use store::DataStore;
        use store::Store;
        use store::StoreMsg;
        use store::math::Range;

        use super::TestRecords;

        #[test]
        #[serial(gtk)]
        fn insert_third_beginning() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("E - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            store.send(StoreMsg::Commit(new_record3.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record3, new_record2, new_record1];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }

        #[test]
        #[serial(gtk)]
        fn insert_third_middle() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("C - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            store.send(StoreMsg::Commit(new_record3.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record2, new_record3, new_record1];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }

        #[test]
        #[serial(gtk)]
        fn insert_third_end() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("A - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            store.send(StoreMsg::Commit(new_record3.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record2, new_record1, new_record3];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }
    }

    mod two_steps {

        use record::Record;
        use reexport::glib;
        use reexport::gtk;

        use serial_test::serial;

        use dummy::test_cases::TestRecord;
        use store::DataStore;
        use store::Store;
        use store::StoreMsg;
        use store::math::Range;

        use super::TestRecords;

        #[test]
        #[serial(gtk)]
        fn insert_third_beginning() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("E - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            store.send(StoreMsg::Commit(new_record3.clone()));
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record3, new_record2, new_record1];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }

        #[test]
        #[serial(gtk)]
        fn insert_third_middle() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("C - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            store.send(StoreMsg::Commit(new_record3.clone()));
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record2, new_record3, new_record1];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }

        #[test]
        #[serial(gtk)]
        fn insert_third_end() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let backend: TestRecords = TestRecords::new();
            let store: Store<TestRecords> = Store::new(backend);
            let new_record1 = TestRecord::since("B - First record to be added", 0).permanent();
            let new_record2 = TestRecord::since("D - Second record to be added", 0).permanent();
            let new_record3 = TestRecord::since("A - Third records to be added", 0).permanent();
            store.send(StoreMsg::Commit(new_record1.clone()));
            store.send(StoreMsg::Commit(new_record2.clone()));
            
            assert_eq!(store.len(), 0, "Store should be empty at the beginning");
            context.iteration(true);
            store.send(StoreMsg::Commit(new_record3.clone()));
            context.iteration(true);
            assert_eq!(store.len(), 3, "Now store should have two elements");

            let record1_from_store = store.get(&new_record1.get_id());
            let record2_from_store = store.get(&new_record2.get_id());
            let record3_from_store = store.get(&new_record3.get_id());
            assert_eq!(record1_from_store, Some(new_record1.clone()));
            assert_eq!(record2_from_store, Some(new_record2.clone()));
            assert_eq!(record3_from_store, Some(new_record3.clone()));

            let data = vec![new_record2, new_record1, new_record3];
            let range_from_store = store.get_range(&Range::new(0, 10));
            assert_eq!(range_from_store, data, "Value returned from store and expected data must match");
        }
    }
}

