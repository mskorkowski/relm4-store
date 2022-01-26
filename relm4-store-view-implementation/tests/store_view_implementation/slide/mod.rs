use store::window::PositionTrackingWindow;
use crate::common::StoreViewTest;

// sliding is independent form window behavior so which behavior we use doesn't matter
type ST = StoreViewTest<PositionTrackingWindow>;

mod last_page {
    use backend_dummy::test_cases::TestCaseBuilder;
    use serial_test::serial;
    use store::Position;
    use store::StoreSize;
    use store::StoreView;

    use super::ST;

    #[test]
    #[serial(gtk)]
    fn slide_left_by_1_of_10() {
        let slide1 = 15;
        let slide2 = 14;
        let page_size1 = 10;

        let tc = TestCaseBuilder::default()
            .initial_size(30)
            .add_step()
            .slide(slide1)
            .add_step()
            .slide(slide2)
            .build();


        ST::from(tc)
            .window_size(StoreSize::Items(page_size1))
            .step(&|test_data, store_view, _|{
                let data = store_view.get_view_data();

                for idx in 0..10 {
                    assert_eq!(data[idx].record, test_data[idx+15]);
                    assert_eq!(data[idx].position, Position(idx+15));
                }
            })
            .step(&|test_data, store_view, _|{
                let data = store_view.get_view_data();

                for idx in 0..10 {
                    assert_eq!(data[idx].record, test_data[idx+14]);
                    assert_eq!(data[idx].position, Position(idx+14));
                }
            })
            .run();
    }

    #[test]
    #[serial(gtk)]
    fn slide_left_by_5_of_10() {
        let slide1 = 15;
        let slide2 = 10;
        let page_size1 = 10;

        let tc = TestCaseBuilder::default()
            .initial_size(30)
            .add_step()
            .slide(slide1)
            .add_step()
            .slide(slide2)
            .build();


        ST::from(tc)
            .window_size(StoreSize::Items(page_size1))
            .step(&|test_data, store_view, _|{
                let data = store_view.get_view_data();

                for idx in 0..10 {
                    assert_eq!(data[idx].record, test_data[idx+15]);
                    assert_eq!(data[idx].position, Position(idx+15));
                }
            })
            .step(&|test_data, store_view, _|{
                let data = store_view.get_view_data();

                for idx in 0..10 {
                    assert_eq!(data[idx].record, test_data[idx+10]);
                    assert_eq!(data[idx].position, Position(idx+10));
                }
            })
            .run();
    }
}