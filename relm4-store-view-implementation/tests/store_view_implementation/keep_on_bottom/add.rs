use serial_test::serial;
use backend_dummy::test_cases::TestCases;
use store::StoreSize;
use store::StoreView;
use store::math::Range;

use super::ST;

#[test]
#[serial(gtk)]
fn add_one_at_the_end_of_the_view() {
    ST::from(TestCases::add_nth(10, 10, 1))
        .window_size(StoreSize::Items(10))
        .step(&|test_data, store_view,_| {
            assert_eq!(store_view.current_len(), 10);
            assert_eq!(store_view.get_window(), Range::new(1,11));
            let data = store_view.get_view_data();
            assert_eq!(data[0].record, test_data[1]);
            assert_eq!(data[9].record, test_data[10]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_one_in_the_second_half_of_the_view() {
    ST::from(TestCases::add_nth(10, 8, 1))
        .window_size(StoreSize::Items(10))
        .step(&|test_data, store_view,_| {
            assert_eq!(store_view.current_len(), 10);
            assert_eq!(store_view.get_window(), Range::new(1,11));
            let data = store_view.get_view_data();
            assert_eq!(data[0].record, test_data[1]);
            assert_eq!(data[1].record, test_data[2]);
            assert_eq!(data[2].record, test_data[3]);
            assert_eq!(data[3].record, test_data[4]);
            assert_eq!(data[4].record, test_data[5]);
            assert_eq!(data[5].record, test_data[6]);
            assert_eq!(data[6].record, test_data[7]);
            assert_eq!(data[7].record, test_data[10]);
            assert_eq!(data[8].record, test_data[8]);
            assert_eq!(data[9].record, test_data[9]);
        })
        .run();
}

#[test]
#[serial(gtk)]
fn add_five_at_the_end_of_the_view() {
    ST::from(TestCases::add_nth(10, 10, 5))
        .window_size(StoreSize::Items(10))
        .step(&|test_data, store_view,_| {
            assert_eq!(store_view.current_len(), 10);
            assert_eq!(store_view.get_window(), Range::new(5,15));
            let data = store_view.get_view_data();
            assert_eq!(data[0].record, test_data[5]);
            assert_eq!(data[9].record, test_data[14]);
        })
        .run();
}