use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use reexport::gtk;
use reexport::relm4;
use reexport::log;

use std::cell::BorrowError;
use std::cell::BorrowMutError;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use gtk::glib;

use relm4::Components;
use relm4::send;
use relm4::Sender;
use relm4::factory::Factory;
use relm4::factory::FactoryPrototype;
use relm4::factory::FactoryView;

use record::Id;
use record::Identifiable;
use record::Record;

use super::DataStore;
use super::Handler;
use super::HandlerWrapper;
use super::FactoryBuilder;
use super::FactoryContainerWidgets;
use super::math;
use super::Position;
use super::RecordWithLocation;
use super::StoreId;
use super::StoreMsg;
use super::StoreSize;
use super::StoreView;
use super::StoreViewImplementation;
use super::StoreViewImplHandler;

use crate::redraw_messages::RedrawMessages;

pub enum StoreViewInterfaceError {
    /// Error returned if borrow failed
    Borrow(BorrowError),
    /// Error returned if borrowing mutably failed
    BorrowMut(BorrowMutError),
}

/// Helper to convert values of [`std::cell::BorrowError`] into [`StoreViewInterfaceError`]
impl From<BorrowError> for StoreViewInterfaceError {
    fn from(err: BorrowError) -> Self {
        StoreViewInterfaceError::Borrow(err)
    }
}

/// Helper to convert values of [`std::cell::BorrowMutError`] into [`StoreViewInterfaceError`]
impl From<BorrowMutError> for StoreViewInterfaceError {
    fn from(err: BorrowMutError) -> Self {
        StoreViewInterfaceError::BorrowMut(err)
    }
}

/// Formats [`StoreViewInterfaceError`] for empty format `{}`
/// 
/// This allows you to print errors without doing `matching` or `if let` statements
impl Display for StoreViewInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreViewInterfaceError::Borrow(err) => {
                f.write_fmt(format_args!("{}", err))
            },
            StoreViewInterfaceError::BorrowMut(err) => {
                f.write_fmt(format_args!("{}", err))
            }
        }
    }
}

/// Formats [`StoreViewInterfaceError`] for debug format `{:?}`
/// 
/// This allows you to print errors without doing `matching` or `if let` statements
impl Debug for StoreViewInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreViewInterfaceError::Borrow(err) => {
                f.debug_tuple("StoreViewInterfaceError::Borrow")
                    .field(err)
                    .finish()
            },
            StoreViewInterfaceError::BorrowMut(err) => {
                f.debug_tuple("StoreViewInterfaceError::BorrowMut")
                    .field(err)
                    .finish()
            }
        }
    }
}

pub struct StoreViewInterface<Builder, Allocator= DefaultIdAllocator> 
where
    Builder: FactoryBuilder<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    view: Rc<RefCell<StoreViewImplementation<Builder, Allocator>>>,
    _components: Builder::Components,
    _container: Rc<RefCell<Builder::ContainerWidgets>>,
    _view_model: Rc<RefCell<Builder>>,
    root_widget: <Builder::ContainerWidgets as FactoryContainerWidgets<Builder, Allocator>>::Root,
    sender: Sender<Builder::Msg>,
    redraw_sender: Sender<RedrawMessages>,
}

impl<Builder, Allocator> StoreViewInterface<Builder, Allocator> 
where 
    Builder: FactoryBuilder<Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    pub fn new(
        parent_view_model: &Builder::ParentViewModel, 
        store: Rc<RefCell<Builder::Store>>, 
        size: StoreSize
    ) -> Self {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let handler_sender = sender.clone();
        let redraw_handler_sender = sender.clone();
        let (redraw_sender, redraw_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let handler_redraw_sender = redraw_sender.clone();

        let view = Rc::new(
            RefCell::new(
                StoreViewImplementation::new(store.clone(), size.items())
            )
        );
        let view_id = view.borrow().get_id();
        let weak_view = Rc::downgrade(&view);
        let handler_view = view.clone();
        let redraw_handler_view = view.clone();

        {
            let s: RefMut<'_, Builder::Store> = store.borrow_mut();
            s.listen(
                view_id.transfer(),
                Box::new(
                    StoreViewImplHandler::new(weak_view, redraw_sender.clone()),
                )
            );
        }

        let view_model = Builder::init_view_model(parent_view_model, view.clone());
        let container = {
            let v: &StoreViewImplementation<Builder, Allocator> = &view.borrow();
            Builder::ContainerWidgets::init_view(
                &view_model,
                v,
                sender.clone(),
            )
        };

        let components = <Builder::Components as Components<Builder>>::init_components(&view_model, &container, sender.clone());
        container.connect_components(&view_model, &components);

        let shared_view_model = Rc::new(RefCell::new(view_model));
        let handler_view_model = shared_view_model.clone();
        let redraw_handler_view_model = shared_view_model.clone();
        

        let root_widget = container.root_widget();
        let shared_container = Rc::new(RefCell::new(container));
        let handler_container = shared_container.clone();
        let redraw_handler_container = shared_container.clone();
        
        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg| {

                if let Ok(store_view) = handler_view.try_borrow() {
                    if let Ok(mut view_model) = handler_view_model.try_borrow_mut() {
                        if let Ok(mut container) = handler_container.try_borrow_mut() {
                            Builder::update(&mut view_model, msg, handler_sender.clone());
                            container.view(&view_model, &store_view, handler_sender.clone());
                            send!(handler_redraw_sender, RedrawMessages::Redraw);
                        }
                        else {
                            log::warn!(target: "relm4-store", "Could not borrow the container. Make sure you dropped all references to container after user");
                        }
                    }
                    else {
                        log::warn!(target: "relm4-store", "Could not borrow the view model. Make sure you dropped all references to view model after use");
                    }
                }
                else {
                    log::warn!(target: "relm4-store", "Could not borrow the store view. Make sure you dropped all references to store view after use");
                }

                glib::Continue(true)
            });
        }

        {
            let context = glib::MainContext::default();
            redraw_receiver.attach(Some(&context), move |_| {
                if let Ok(store_view) = redraw_handler_view.try_borrow() {
                    if let Ok(view_model) = redraw_handler_view_model.try_borrow() {
                        if let Ok(mut container) = redraw_handler_container.try_borrow_mut() {
                            container.view(&view_model,&store_view, redraw_handler_sender.clone());
                            if store_view.inbox_queue_size() > 0 { //only redraw if there is an update awaiting
                                store_view.generate(container.container_widget(), redraw_handler_sender.clone());
                            }
                        }
                        else {
                            log::warn!(target: "relm4-store", "Could not borrow the container. Make sure you dropped all references to container after user");
                        }
                    }
                    else {
                        log::warn!(target: "relm4-store", "Could not borrow the view model. Make sure you dropped all references to view model after use");
                    }
                }
                else {
                    log::warn!(target: "relm4-store", "Could not borrow the store view. Make sure you dropped all references to store view after use");
                }

                glib::Continue(true)
            });
        }

        Self {
            view,
            _components: components,
            _container: shared_container,
            _view_model: shared_view_model,
            root_widget,
            sender,
            redraw_sender,
        }
    }

    pub fn sender(&self) -> Sender<Builder::Msg> {
        self.sender.clone()
    }

    pub fn send(&self, msg: Builder::Msg) -> Result<(), std::sync::mpsc::SendError<Builder::Msg>> {
        self.sender.send(msg)
    }

    pub fn root_widget(&self) -> &<Builder::ContainerWidgets as FactoryContainerWidgets<Builder, Allocator>>::Root {
        &self.root_widget
    }
}

impl<Builder, Allocator> Identifiable<Self, Allocator::Type> for StoreViewInterface<Builder, Allocator>
where
    Builder: 'static + FactoryBuilder<Allocator>,
    Allocator: TemporaryIdAllocator + 'static,

{
    type Id = StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.view.borrow().get_id().transfer()
    }
}

impl<Builder, Allocator> DataStore<Allocator> for StoreViewInterface<Builder, Allocator>
where
    Builder: 'static + FactoryBuilder<Allocator>,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Record = <Builder::Store as DataStore<Allocator>>::Record;

    fn inbox(&self, msg: StoreMsg<<Self as DataStore<Allocator>>::Record>) {
        println!("[StoreViewInterface::inbox] StoreViewInterface received message");
        self.view.borrow().inbox(msg);
        let redraw_sender = self.redraw_sender.clone();
        send!(redraw_sender, RedrawMessages::Redraw);
    }

    fn len(&self) -> usize { 
        self.view.borrow().len()
    }

    fn is_empty(&self) -> bool { 
        self.view.borrow().is_empty()
    }

    fn get(&self, id: &Id<Self::Record>) -> Option<Self::Record> { 
        self.view.borrow().get(id)
     }

    fn get_range(&self, range: &math::Range) -> std::vec::Vec<Self::Record> {
        self.view.borrow().get_range(range)
    }

    fn listen(&self, handler_ref: StoreId<Self, Allocator>,  handler: std::boxed::Box<(dyn Handler<Self, Allocator> + 'static)>) { 
        self.view.borrow_mut().listen(
            handler_ref.transfer(),
            HandlerWrapper::from(handler)
        )
     }

    fn unlisten(&self, id: StoreId<Self, Allocator>) { 
        self.view.borrow_mut().unlisten(id.transfer())
    }
}

impl<Builder, Allocator> StoreView<Allocator> for StoreViewInterface<Builder, Allocator>
where
    Builder: 'static + FactoryBuilder<Allocator>,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Builder = Builder;

    fn window_size(&self) -> usize {
        self.view.borrow().window_size()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Record>> {
        self.view.borrow().get_view_data()
    }

    fn first_page(&self) {
        self.view.borrow().first_page();
    }

    fn prev_page(&self) {
        self.view.borrow().prev_page();
    }

    fn next_page(&self) {
        self.view.borrow().next_page();
    }

    fn last_page(&self) {
        self.view.borrow().last_page();
    }

    fn get_window(&self) -> math::Range {
        self.view.borrow().get_window()
    }

    fn get_position(&self, id: &Id<Self::Record>) -> Option<Position> {
        self.view.borrow().get_position(id)
    }

    fn set_window(&self, range: math::Range) {
        self.view.borrow().set_window(range);
    }

    fn inbox_queue_size(&self) -> usize {
        self.view.borrow().inbox_queue_size()
    }
}

impl<Builder, Allocator> FactoryPrototype for StoreViewInterface<Builder, Allocator>
where
    Builder: FactoryBuilder<Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Factory = Self;
    type Msg = Builder::Msg;
    type Widgets = Builder::RecordWidgets;
    type Root = Builder::Root;
    type View = Builder::View;

    fn generate(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        sender: Sender<Builder::Msg>,
    ) -> Self::Widgets {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while generating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Builder::generate(&model, position, sender)
    }

    /// Set the widget position upon creation, useful for [`gtk::Grid`] or similar.
    fn position(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while positioning! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Builder::position(model, position)
    }

    /// Function called when self is modified.
    fn update(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while updating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        <Builder as FactoryBuilder<Allocator>>::update_record(model, position, widgets)
    }

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root {
        Builder::get_root(widgets)
    }
}

impl<Builder, Allocator> Factory<StoreViewInterface<Builder, Allocator>, Builder::View> for StoreViewInterface<Builder, Allocator>
where
    Builder: FactoryBuilder<Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Key = Id<<Builder::Store as DataStore<Allocator>>::Record>;

    fn generate(&self, view: &Builder::View, sender: Sender<Builder::Msg>) {
        let view_impl = self.view.borrow();
        view_impl.generate(view, sender);
    }
}