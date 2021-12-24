use reexport::glib;
use reexport::gtk;
use reexport::relm4;

use std::fmt::Debug;
use std::marker::PhantomData;

use relm4::Sender;

use backend_dummy::DummyBackend;
use backend_dummy::StepByStepStore;
use backend_dummy::test_cases::TestCase;
use backend_dummy::test_cases::TestRecord;
use store::DataStore;
use store::StoreViewPrototype;
use store::Position;
use store::Store;
use store::StoreSize;
use store::math::Range;
use store::redraw_messages::RedrawMessages;
use store::window::WindowBehavior;

use relm4_store_view_implementation::StoreViewImplementation;
use relm4_store_view_implementation::View;

#[derive(Debug)]
pub struct TestWidgets {
    root: gtk::Box,
}

#[derive(Debug)]
pub struct TestConfig<Window: 'static + WindowBehavior> {
    _window: PhantomData<*const Window>,
}

impl<Window: 'static + WindowBehavior + Debug> StoreViewPrototype for TestConfig<Window> {
    type Store = Store<DummyBackend<TestRecord>>;
    type StoreView = View<Self>;
    type RecordWidgets = TestWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = Window;
    type ViewModel = ();
    type ParentViewModel = ();

    fn init_store_view(store: Self::Store, size: store::StoreSize, redraw_sender: Sender<RedrawMessages>) -> Self::StoreView {
        View::new(store, size, redraw_sender)
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
    Window: 'static + WindowBehavior + Debug
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
    Window: 'static + WindowBehavior + Debug,
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

        let (view_sender, _view_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let container = gtk::Box::default();

        let mut data_store: Store<DummyBackend<TestRecord>> = Store::new(DummyBackend::new(self.test_case.configuration.clone()));

        let store_view: StoreViewImplementation<TestConfig<Window>> = StoreViewImplementation::new(
            data_store.clone(), 
            self.window_size.items(),
        );

        // StoreView is using `Reload` event to populate itself
        context.iteration(true);
        store_view.view(&container, view_sender.clone());

        if let Some(p) = self.prepare {
            let block = p(&store_view);
            context.iteration(block);
            store_view.view(&container, view_sender.clone());
        }
        

        let data_store_len = data_store.len();
        let ia = self.initial_assertion;
        ia(&self.test_case.data.clone(), &store_view, &data_store.get_range(&Range::new(0, data_store_len)));



        for assertion in &self.asserts {
            {
                data_store.advance();
            }
            context.iteration(true);
            store_view.view(&container, view_sender.clone());
            let data_store_len = data_store.len();
            assertion(&self.test_case.data, &store_view, &data_store.get_range(&Range::new(0, data_store_len)));
        }

        
    }
}
