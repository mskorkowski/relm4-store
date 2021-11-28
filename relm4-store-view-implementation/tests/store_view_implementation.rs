//! This module contains all tests for the store-view

use reexport::gtk;
use reexport::relm4;

use std::marker::PhantomData;

use relm4::Sender;

use backend_dummy::DummyBackend;
use backend_dummy::test_cases::TestRecord;
use store::FactoryConfiguration;
use store::Position;
use store::redraw_messages::RedrawMessages;
use store::window::WindowBehavior;

use relm4_store_view_implementation::StoreViewImplementation;

#[derive(Debug)]
struct TestWidgets {
    root: gtk::Box,
}

struct TestConfig<Window: 'static + WindowBehavior> {
    _window: PhantomData<*const Window>,
}

impl<Window: 'static + WindowBehavior> FactoryConfiguration for TestConfig<Window> {
    type Store = DummyBackend<TestRecord>;
    type StoreView = StoreViewImplementation<Self>;
    type RecordWidgets = TestWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = Window;
    type ViewModel = ();
    type ParentViewModel = ();

    fn init_store_view(store: std::rc::Rc<std::cell::RefCell<Self::Store>>, size: store::StoreSize, redraw_sender: Sender<RedrawMessages>) -> Self::StoreView {
        StoreViewImplementation::new(store, size.items(), redraw_sender)
    }

    fn generate(_record: &<Self::Store as store::DataStore<record::UuidAllocator>>::Record, _position: Position, _sender: Sender<()>) -> Self::RecordWidgets {
        TestWidgets{
            root: gtk::Box::default()
        }
    }

    fn update_record(_model: <Self::Store as store::DataStore<record::UuidAllocator>>::Record, _position: Position, _widgets: &Self::RecordWidgets) {}

    fn update(_view_model: &mut Self::ViewModel, _msg: (), _sender: Sender<()>) {}
    
    fn init_view_model(_parent_view_model: &Self::ParentViewModel, _store_view: std::rc::Rc<std::cell::RefCell<Self::StoreView>>) -> Self::ViewModel {}

    fn position(_model: <Self::Store as store::DataStore<record::UuidAllocator>>::Record, _position: Position) {}

    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root {
        &widgets.root
    }
}

mod store_view {
    mod position_tracking_window {

    }

    mod value_tracking_window {
        use std::cell::RefCell;
        use std::rc::Rc;

        use backend_dummy::test_cases::TestCase;
        use reexport::glib;
        use reexport::gtk;
       
        use backend_dummy::DummyBackend;
        use backend_dummy::test_cases::TestCases;
        use backend_dummy::test_cases::TestRecord;
        use store::DataStore;
        use store::Position;
        use store::StoreView;
        use store::window::ValueTrackingWindow;
    
    
        use relm4_store_view_implementation::StoreViewImplementation;
        use crate::TestConfig;
        type SC = TestConfig<ValueTrackingWindow>;

        #[test]
        fn add_first_record() {
            gtk::init().unwrap();
            let context = glib::MainContext::default();
            let _guard = context.acquire().unwrap();

            let (sender, _receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let (view_sender, _view_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            
            
            let container = gtk::Box::default();

            
            let TestCase{configuration, data:_} = TestCases::add_first_record();
            let data_store: DummyBackend<TestRecord> = DummyBackend::new(configuration);
            let shared_store = Rc::new(RefCell::new(data_store));

            let store_view: StoreViewImplementation<SC> = StoreViewImplementation::new(shared_store.clone(), 10, sender);
            
            // new loads data using `reload` event to self
            // this two calls will handle the case
            context.iteration(true);
            store_view.view(&container, view_sender.clone());

            assert!(shared_store.borrow().listeners_len() == 1);
            assert!(store_view.current_len() == 0);

            {
                shared_store.borrow_mut().advance();
            }

            context.iteration(true);

            println!("Store view inbox size: {}", store_view.inbox_queue_size());

            assert!(store_view.inbox_queue_size() == 1);

            store_view.view(&container, view_sender);
            println!("Length of data in the store: {}", shared_store.borrow().len());
            println!("Length of data in the view: {}", store_view.current_len());
            assert!(store_view.current_len() == 1);
            let data = store_view.get_view_data();
            assert!(data[0].position == Position(0))
        }
    }
}