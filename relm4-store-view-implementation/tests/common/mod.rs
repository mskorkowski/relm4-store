use reexport::glib;
use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use relm4::Sender;

use backend_dummy::DummyBackend;
use backend_dummy::test_cases::TestCase;
use backend_dummy::test_cases::TestRecord;
use store::DataStore;
use store::FactoryConfiguration;
use store::Position;
use store::StoreSize;
use store::math::Range;
use store::redraw_messages::RedrawMessages;
use store::window::WindowBehavior;

use relm4_store_view_implementation::StoreViewImplementation;

#[derive(Debug)]
pub struct TestWidgets {
    root: gtk::Box,
}

pub struct TestConfig<Window: 'static + WindowBehavior> {
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

    fn generate(_record: &<Self::Store as store::DataStore>::Record, _position: Position, _sender: Sender<()>) -> Self::RecordWidgets {
        TestWidgets{
            root: gtk::Box::default()
        }
    }

    fn update_record(_model: <Self::Store as store::DataStore>::Record, _position: Position, _widgets: &Self::RecordWidgets) {}

    fn update(_view_model: &mut Self::ViewModel, _msg: (), _sender: Sender<()>) {}
    
    fn init_view_model(_parent_view_model: &Self::ParentViewModel, _store_view: std::rc::Rc<std::cell::RefCell<Self::StoreView>>) -> Self::ViewModel {}

    fn position(_model: <Self::Store as store::DataStore>::Record, _position: Position) {}

    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root {
        &widgets.root
    }
}

pub type Assertion<Window> = &'static dyn Fn(&Vec<TestRecord>, &StoreViewImplementation<TestConfig<Window>>, &Vec<TestRecord>) -> ();
pub type Prepare<Window> = &'static dyn Fn(&StoreViewImplementation<TestConfig<Window>>) -> bool;

pub struct StoreViewTest<Window>
where
    Window: 'static + WindowBehavior
{
    asserts: Vec<Assertion<Window>>,
    initial_assertion: Assertion<Window>,
    prepare: Option<Prepare<Window>>,
    test_case: TestCase,
    window_size: StoreSize,
    _window: PhantomData<*const Window>,
}

impl<Window> StoreViewTest<Window>
where
    Window: 'static + WindowBehavior,
{
    pub fn from(config: TestCase) -> StoreViewTest<Window> {
        StoreViewTest{
            asserts: vec![],
            initial_assertion: &|_,_,_|{},
            prepare: None,
            test_case: config,
            window_size: StoreSize::Unlimited,
            _window: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn skip_step(&mut self) -> &mut Self {
        self.step(&|_, _, _|{});
        self
    }

    pub fn step(&mut self, f: Assertion<Window>) -> &mut Self {
        self.asserts.push(f);
        self
    }

    pub fn initial(&mut self, f: Assertion<Window>) -> &mut Self {
        self.initial_assertion = f;
        self
    }

    pub fn window_size(&mut self, size: StoreSize) -> &mut Self {
        self.window_size = size;
        self
    }

    pub fn prepare(&mut self, f: Prepare<Window>) -> &mut Self {
        self.prepare = Some(f);
        self
    }

    pub fn run(&self) {
        gtk::init().unwrap();

        let context = glib::MainContext::default();
        let _guard = context.acquire().unwrap();

        let (sender, _receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let (view_sender, _view_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let container = gtk::Box::default();

        let data_store: DummyBackend<TestRecord> = DummyBackend::new(self.test_case.configuration.clone());
        let shared_store = Rc::new(RefCell::new(data_store));

        let store_view: StoreViewImplementation<TestConfig<Window>> = StoreViewImplementation::new(
            shared_store.clone(), 
            self.window_size.items(), 
            sender
        );
        // StoreView is using `Reload` event to populate itself
        context.iteration(true);
        store_view.view(&container, view_sender.clone());

        if let Some(p) = self.prepare {
            let block = p(&store_view);
            context.iteration(block);
            store_view.view(&container, view_sender.clone());
        }
        

        let data_store_len = shared_store.borrow().len();
        let ia = self.initial_assertion;
        ia(&self.test_case.data.clone(), &store_view, &shared_store.borrow().get_range(&Range::new(0, data_store_len)));



        for assertion in &self.asserts {
            {
                shared_store.borrow_mut().advance();
            }
            context.iteration(true);
            store_view.view(&container, view_sender.clone());
            let data_store_len = shared_store.borrow().len();
            assertion(&self.test_case.data, &store_view, &shared_store.borrow().get_range(&Range::new(0, data_store_len)));
        }

        
    }
}
