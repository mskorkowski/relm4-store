use super::ST;

mod last_page {
    use serial_test::serial;
    use backend_dummy::test_cases::TestCases;
    use store::Position;
    use store::StoreView;

    use super::ST;

    #[test]
    #[serial(gtk)]
    fn remove_last() {
        ST::from(TestCases::remove_last())
            .step(&|_, store_view, _|{
                assert_eq!(store_view.current_len(), 0, "store must be empty");
                let data = store_view.get_view_data();
                assert_eq!(data.len(), 0, "View must be empty");
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_first_of_two() {
        ST::from(TestCases::remove_first_of_two())
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 1, "store must have one element");
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0), "View must be empty");
                assert_eq!(data[0].record, test_data[1], "View must be empty");
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_second_of_two() {
        ST::from(TestCases::remove_second_of_two())
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 1, "store must have one element");
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0), "View must be empty");
                assert_eq!(data[0].record, test_data[0], "View must be empty");
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_first_of_three() {
        ST::from(TestCases::remove_first_of_three())
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 2, "store must have one element");
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0), "View must be empty");
                assert_eq!(data[1].position, Position(1), "View must be empty");
                assert_eq!(data[0].record, test_data[1], "View must be empty");
                assert_eq!(data[1].record, test_data[2], "View must be empty");
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_second_of_three() {
        ST::from(TestCases::remove_second_of_three())
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 2, "store must have one element");
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0), "View must be empty");
                assert_eq!(data[1].position, Position(1), "View must be empty");
                assert_eq!(data[0].record, test_data[0], "View must be empty");
                assert_eq!(data[1].record, test_data[2], "View must be empty");
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_third_of_three() {
        ST::from(TestCases::remove_third_of_three())
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 2, "store must have one element");
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0), "View must be empty");
                assert_eq!(data[1].position, Position(1), "View must be empty");
                assert_eq!(data[0].record, test_data[0], "View must be empty");
                assert_eq!(data[1].record, test_data[1], "View must be empty");
            })
            .run();
    }
}

mod first_page {
    use serial_test::serial;
    use backend_dummy::test_cases::TestCases;
    use store::Position;
    use store::StoreSize;
    use store::StoreView;
    use store::math::Range;

    use super::ST;

    #[test]
    #[serial(gtk)]
    fn remove_element_from_second_half() {
        ST::from(TestCases::remove_nth(5, 25))
            .window_size(StoreSize::Items(10))
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 10, "Store view must be full");
                let data = store_view.get_view_data();
                assert_eq!(data[9].position, Position(9));
                assert_eq!(data[9].record, test_data[10]);
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_element_from_first_half() {
        ST::from(TestCases::remove_nth(4, 25))
            .window_size(StoreSize::Items(10))
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 10, "Store view must be full");
                let range = store_view.get_window();
                assert_eq!(range, Range::new(0, 10));
                let data = store_view.get_view_data();
                // println!("Data in the view: {:#?}", data);
                assert_eq!(data[9].position, Position(9));
                assert_eq!(data[9].record, test_data[10]);
            })
            .run();
    }
}

mod somwhere_in_the_middle {
    use serial_test::serial;
    use backend_dummy::test_cases::TestCases; 
    use store::Position;
    use store::StoreSize;
    use store::StoreView;
    use store::math::Range;

    use super::ST;

    #[test]
    #[serial(gtk)]
    fn remove_record_in_the_first_half_of_view() {
        ST::from(TestCases::remove_nth(14, 25))
            .window_size(StoreSize::Items(10))
            .prepare(&|view|{
                view.next_page();
                true
            })
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 10, "Store view must be full");
                let range = store_view.get_window();
                assert_eq!(range, Range::new(9, 19));
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(9));
                assert_eq!(data[0].record, test_data[9]);
                assert_eq!(data[9].position, Position(18));
                assert_eq!(data[9].record, test_data[19]);
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn remove_record_to_the_left_of_view() {
        ST::from(TestCases::remove_nth(5, 25))
            .window_size(StoreSize::Items(10))
            .prepare(&|view|{
                view.next_page();
                true
            })
            .step(&|test_data, store_view, _|{
                assert_eq!(store_view.current_len(), 10, "Store view must be full");
                let range = store_view.get_window();
                assert_eq!(range, Range::new(9, 19));
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(9));
                assert_eq!(data[0].record, test_data[9]);
                assert_eq!(data[9].position, Position(18));
                assert_eq!(data[9].record, test_data[19]);
            })
            .run();
    }
}