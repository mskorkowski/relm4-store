use serial_test::serial;
use backend_dummy::test_cases::TestCases;
use store::Position;
use store::window::ValueTrackingWindow;
use crate::common::StoreViewTest;

type ST = StoreViewTest<ValueTrackingWindow>;

#[test]
#[serial(gtk)]
fn add_first_record() {
    ST::from(TestCases::add_first_record())
        .initial(&|_, store_view, _|{
            assert_eq!(store_view.current_len(), 0, "store must be empty at the beginning");
        })
        .step(&|test_data, store_view, _|{
            assert_eq!(store_view.current_len(), 1, "store must contain one element");
            let data = store_view.get_view_data();
            assert_eq!(data[0].position, Position(0), "0th position don't match");
            assert_eq!(data[0].record, test_data[0], "0th record don't match");
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_second_record_at_the_beginning() {
    ST::from(TestCases::add_second_record_at_the_beginning())
        .initial(&|_, store_view, _|{
            assert_eq!(store_view.current_len(), 1);
        })
        .step(&|test_data, store_view, _|{
            assert_eq!(store_view.current_len(), 2, "current_len == 2");
            let data = store_view.get_view_data();

            assert_eq!(data[0].position, Position(0), "data[0].position == Position(0)");
            assert_eq!(data[1].position, Position(1), "data[1].position == Position(1)");
            assert_eq!(data[0].record, test_data[1]);
            assert_eq!(data[1].record, test_data[0]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_second_record_at_the_end() {
    ST::from(TestCases::add_second_record_at_the_end())
        .initial(&|_, store_view, _|{
            assert!(store_view.current_len() == 1);
        })
        .step(&|test_data, store_view, _|{
            assert_eq!(store_view.current_len(), 2);
            let data = store_view.get_view_data();
            assert_eq!(data[0].position, Position(0));
            assert_eq!(data[1].position, Position(1));
            assert_eq!(data[0].record, test_data[0]);
            assert_eq!(data[1].record, test_data[1]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_third_record_at_the_beginning() {
    ST::from(TestCases::add_third_record_at_the_beginning())
        .initial(&|_, store_view, _|{
            assert!(store_view.current_len() == 2);
        })
        .step(&|test_data, store_view, _|{
            assert!(store_view.current_len() == 3);
            let data = store_view.get_view_data();
            assert!(data[0].position == Position(0));
            assert!(data[1].position == Position(1));
            assert!(data[2].position == Position(2));
            assert!(data[0].record == test_data[2]);
            assert!(data[1].record == test_data[0]);
            assert!(data[2].record == test_data[1]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_third_record_in_the_middle() {
    ST::from(TestCases::add_third_record_in_the_middle())
        .initial(&|_, store_view, _|{
            assert!(store_view.current_len() == 2);
        })
        .step(&|test_data, store_view,_|{
            assert!(store_view.current_len() == 3);
            let data = store_view.get_view_data();
            assert!(data[0].position == Position(0));
            assert!(data[1].position == Position(1));
            assert!(data[2].position == Position(2));
            assert!(data[0].record == test_data[0]);
            assert!(data[1].record == test_data[2]);
            assert!(data[2].record == test_data[1]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_third_record_at_the_end() {
    ST::from(TestCases::add_third_record_at_the_end())
        .initial(&|_, store_view,_|{
            assert!(store_view.current_len() == 2);
        })
        .step(&|test_data, store_view,_|{
            assert!(store_view.current_len() == 3);
            let data = store_view.get_view_data();
            assert!(data[0].position == Position(0));
            assert!(data[1].position == Position(1));
            assert!(data[2].position == Position(2));
            assert!(data[0].record == test_data[0]);
            assert!(data[1].record == test_data[1]);
            assert!(data[2].record == test_data[2]);
        })
        .run();
}


mod window_size_2 {
    use serial_test::serial;
    use backend_dummy::test_cases::TestCases;
    use store::Position;
    use store::StoreSize;
    use store::math::Range;

    use super::ST;

    /// `|[ 0, 1 ]| -> |2, [ 0, 1 ]|`
    #[test]
    #[serial(gtk)]
    fn add_third_record_at_the_beginning() {
        ST::from(TestCases::add_third_record_at_the_beginning())
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1));
                assert!(data[1].position == Position(2));
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[0, 1]| -> |[0, 2], 1|`
    #[test]
    #[serial(gtk)]
    fn add_third_record_in_the_middle() {
        ST::from(TestCases::add_third_record_in_the_middle())
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0));
                assert!(data[1].position == Position(1));
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .run();
    }


    /// `|[0, 1]| -> |[0, 1], 2|
    #[test]
    #[serial(gtk)]
    fn add_third_record_at_the_end() {
        ST::from(TestCases::add_third_record_at_the_end())
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0));
                assert!(data[1].position == Position(1));
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |2, [ 0, 1 ]| -> |3, 2, [0, 1]|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_0_at_0() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(0),
                vec!(0),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 0th step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 0th step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(2), "At 1st step expected position 2 got {:?}", data[0].position);
                assert!(data[1].position == Position(3), "At 1st step expected position 3 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |2, [ 0, 1 ]| -> |2, 3, [0, 1]|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_0_at_1() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(0),
                vec!(1),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 0th step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 0th step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(2), "At 1st step expected position 2 got {:?}", data[0].position);
                assert!(data[1].position == Position(3), "At 1st step expected position 3 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |2, [ 0, 1 ]| -> |2, [0, 3], 1|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_0_at_2() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(0),
                vec!(2),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 0th step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 0th step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[3]);
            })
            .run();
    }

    /// `| 0, [ 1, 2 ]| -> | 0, [1, 3], 2|`
    #[test]
    #[serial(gtk)]
    fn simplify_add_multistep_to_initial_2_at_0_at_2() {
        let tc = TestCases::multistep_add_unsafe(
            3,
            vec!(
                vec!(2),
            )
        );

        ST::from(tc)
            .window_size(StoreSize::Items(2))
            .prepare(&|store_view|{
                store_view.set_window(Range::new(1, 3));
                false
            })
            .step(&|test_data, store_view,_|{
                assert_eq!(store_view.current_len(), 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 0th step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 0th step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[1]);
                assert!(data[1].record == test_data[3]);
            })
            .run();
            
    }

   /// `|[ 0, 1 ]| -> |2, [ 0, 1 ]| -> |2, [0, 1], 3|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_0_at_3() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(0),
                vec!(3),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 0th step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 0th step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |[ 0, 2 ], 1| -> |3, [ 0, 2 ], 1|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_1_at_0() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(1),
                vec!(0),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |[ 0, 2 ], 1| -> |[ 0, 3 ], 2, 1|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_1_at_1() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(1),
                vec!(1),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[3]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |[ 0, 2 ], 1| -> |[ 0, 2 ], 3, 1|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_1_at_2() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(1),
                vec!(2),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .run();
    }


    /// `|[ 0, 1 ]| -> |[ 0, 2 ], 1| -> |[ 0, 2 ], 1, 3|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_1_at_3() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(1),
                vec!(3),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[2]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |[ 0, 1], 2| -> |3, [ 0, 1 ], 2|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_2_at_0() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(2),
                vec!(0),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(1), "At 1st step expected position 1 got {:?}", data[0].position);
                assert!(data[1].position == Position(2), "At 1st step expected position 2 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }

    /// `|[ 0, 1 ]| -> |[ 0, 1 ], 2| -> |[ 0, 3], 2, 1|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_2_at_1() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(2),
                vec!(1),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[3]);
            })
            .run();
    }

   /// `|[ 0, 1 ]| -> |[ 0, 1 ], 2| -> |[ 0, 1 ], 3, 2|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_2_at_2() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(2),
                vec!(2),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }


    /// `|[ 0, 1 ]| -> |[ 0, 1 ], 2| -> |[ 0, 1 ], 2, 3|`
    #[test]
    #[serial(gtk)]
    fn add_multistep_to_initial_2_at_2_at_3() {
        ST::from(TestCases::multistep_add_unsafe(
            2,
            vec!(
                vec!(2),
                vec!(3),
            )
        ))
            .window_size(StoreSize::Items(2))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 0th step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 0th step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .step(&|test_data, store_view,_|{
                assert!(store_view.current_len() == 2);
                let data = store_view.get_view_data();
                assert!(data[0].position == Position(0), "At 1st step expected position 0 got {:?}", data[0].position);
                assert!(data[1].position == Position(1), "At 1st step expected position 1 got {:?}", data[1].position);
                assert!(data[0].record == test_data[0]);
                assert!(data[1].record == test_data[1]);
            })
            .run();
    }
}

mod window_size_4 {
    use serial_test::serial;
    use backend_dummy::test_cases::TestCases;
    use store::Position;
    use store::StoreSize;

    use super::ST;

    const WINDOW_SIZE: usize = 4;

    /// `|[ 0, 1 ]| -> |[2,  0, 1 ]|`
    #[test]
    #[serial(gtk)]
    fn add_third_record_at_the_beginning() {
        ST::from(TestCases::add_third_record_at_the_beginning())
            .window_size(StoreSize::Items(WINDOW_SIZE))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 2);
            })
            .step(&|test_data, store_view,_|{
                assert_eq!(store_view.current_len(), 3);
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(0));
                assert_eq!(data[1].position, Position(1));
                assert_eq!(data[2].position, Position(2));
                assert_eq!(data[0].record, test_data[2]);
                assert_eq!(data[1].record, test_data[0]);
                assert_eq!(data[2].record, test_data[1]);
            })
            .run();
    }

    /// |[0, 1, 2, 3]| -> |[4, 1, 2, 3]|
    #[test]
    #[serial(gtk)]
    fn add_fourth_record_at_1 () {
        let tc = TestCases::multistep_add_unsafe(
            4,
            vec![
                vec![1],
            ]
        );

        ST::from(tc)
            .window_size(StoreSize::Items(WINDOW_SIZE))
            .initial(&|_, store_view,_|{
                assert!(store_view.current_len() == 4);
            })
            .step(&|test_data, store_view,_|{
                assert_eq!(store_view.current_len(), 4);
                let data = store_view.get_view_data();
                assert_eq!(data[0].position, Position(1));
                assert_eq!(data[1].position, Position(2));
                assert_eq!(data[2].position, Position(3));
                assert_eq!(data[3].position, Position(4));
                assert_eq!(data[0].record, test_data[4]);
                assert_eq!(data[1].record, test_data[1]);
                assert_eq!(data[2].record, test_data[2]);
                assert_eq!(data[3].record, test_data[3]);
            })
            .run();
    }
}
