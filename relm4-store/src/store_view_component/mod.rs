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

use crate::store_view_implementation::StoreViewImplHandler;

use super::DataStore;
use super::FactoryConfiguration;
use super::FactoryContainerWidgets;
use super::StoreSize;
use super::StoreView;
use super::StoreViewImplementation;

use crate::redraw_messages::RedrawMessages;

/// Enum with possible errors returned by the [StoreViewInterface]
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

/// Specialized kind of component to handle store view 
/// 
/// 
#[derive(Debug)]
pub struct StoreViewComponent<Widgets, Configuration, Allocator= DefaultIdAllocator> 
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    view: Rc<RefCell<StoreViewImplementation<Widgets, Configuration, Allocator>>>,
    _components: Configuration::Components,
    _container: Rc<RefCell<Widgets>>,
    _view_model: Rc<RefCell<Configuration>>,
    root_widget: <Widgets as FactoryContainerWidgets<Configuration, Allocator>>::Root,
    sender: Sender<Configuration::Msg>,
    redraw_sender: Sender<RedrawMessages>,
}

impl<Widgets, Configuration, Allocator> StoreViewComponent<Widgets, Configuration, Allocator> 
where 
    Widgets: FactoryContainerWidgets<Configuration, Allocator> + 'static,
    Configuration: FactoryConfiguration<Widgets, Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    /// Creates new instance of the [StoreViewInterface]
    pub fn new(
        parent_view_model: &Configuration::ParentViewModel, 
        store: Rc<RefCell<Configuration::Store>>, 
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
        let redraw_handler_view = view.clone();

        {
            let s: RefMut<'_, Configuration::Store> = store.borrow_mut();
            s.listen(
                view_id.transfer(),
                Box::new(
                    StoreViewImplHandler::new(weak_view, redraw_sender.clone()),
                )
            );
        }

        let view_model = Configuration::init_view_model(parent_view_model, view.clone());
        let container = {
            Widgets::init_view(
                &view_model,
                sender.clone(),
            )
        };

        let components = <Configuration::Components as Components<Configuration>>::init_components(&view_model, &container, sender.clone());
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
                if let Ok(mut view_model) = handler_view_model.try_borrow_mut() {
                    if let Ok(mut container) = handler_container.try_borrow_mut() {
                        // update store view if there are any unhandled messages in the inbox of the store-view
                        container.view(&view_model, handler_sender.clone());
                        // update the view using fully resolved store-view
                        Configuration::update(&mut view_model, msg, handler_sender.clone());
                        // in case some messages were added to the store view re run the update for the new messages
                        // this way `update` has seen fully resolved store-view and in case some messages were sent
                        // they are resolved also
                        container.view(&view_model, handler_sender.clone());
                        send!(handler_redraw_sender, RedrawMessages::Redraw);
                    }
                    else {
                        log::warn!(target: "relm4-store", "Could not borrow the container. Make sure you dropped all references to container after user");
                    }
                }
                else {
                    log::warn!(target: "relm4-store", "Could not borrow the view model. Make sure you dropped all references to view model after use");
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
                            container.view(&view_model, redraw_handler_sender.clone());
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

    /// Returns a sender for this component
    pub fn sender(&self) -> Sender<Configuration::Msg> {
        self.sender.clone()
    }

    /// Sends a message to this component
    pub fn send(&self, msg: Configuration::Msg) -> Result<(), std::sync::mpsc::SendError<Configuration::Msg>> {
        self.sender.send(msg)
    }

    /// Returns root widget for this component, in most cases gtk widget
    pub fn root_widget(&self) -> &<Widgets as FactoryContainerWidgets<Configuration, Allocator>>::Root {
        &self.root_widget
    }
}

impl<Widgets, Configuration, Allocator> FactoryPrototype for StoreViewComponent<Widgets, Configuration, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Factory = Self;
    type Msg = Configuration::Msg;
    type Widgets = Configuration::RecordWidgets;
    type Root = Configuration::Root;
    type View = Configuration::View;

    fn generate(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        sender: Sender<Configuration::Msg>,
    ) -> Self::Widgets {
        let view = self.view.borrow();
        let model = view.get(key).expect("Key doesn't point to the model in the store while generating! WTF?");
        let position = view.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::generate(&model, position, sender)
    }

    /// Set the widget position upon creation, useful for [`gtk::Grid`] or similar.
    fn position(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
        let view = self.view.borrow();
        let model = view.get(key).expect("Key doesn't point to the model in the store while positioning! WTF?");
        let position = view.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::position(model, position)
    }

    /// Function called when self is modified.
    fn update(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        let view = self.view.borrow();
        let model = view.get(key).expect("Key doesn't point to the model in the store while updating! WTF?");
        let position = view.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        <Configuration as FactoryConfiguration<Widgets, Allocator>>::update_record(model, position, widgets)
    }

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root {
        Configuration::get_root(widgets)
    }
}

impl<Widgets, Builder, Allocator> Factory<StoreViewComponent<Widgets, Builder, Allocator>, Builder::View> for StoreViewComponent<Widgets, Builder, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Builder, Allocator>,
    Builder: FactoryConfiguration<Widgets, Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Key = Id<<Builder::Store as DataStore<Allocator>>::Record>;

    fn generate(&self, view: &Builder::View, sender: Sender<Builder::Msg>) {
        let view_impl = self.view.borrow();
        view_impl.generate(view, sender);
    }
}