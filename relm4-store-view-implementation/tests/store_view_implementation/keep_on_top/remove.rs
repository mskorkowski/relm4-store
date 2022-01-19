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