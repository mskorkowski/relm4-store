use reexport::gtk;
use reexport::log;
use reexport::relm4;
use reexport::relm4::factory::Factory;

use std::cell::BorrowError;
use std::cell::BorrowMutError;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use gtk::glib;

use relm4::Components;
use relm4::Model as ViewModel;
use relm4::send;
use relm4::Sender;
use relm4::Widgets;

use crate::StoreView;
use crate::StoreViewPrototype;
use crate::FactoryContainerWidgets;
use crate::StoreSize;
use crate::StoreViewInnerComponent;
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
pub struct StoreViewComponent<Configuration> 
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
{
    view: Configuration::StoreView,
    _components: Rc<RefCell<<Configuration::ViewModel as ViewModel>::Components>>,
    _container: Rc<RefCell<<Configuration::ViewModel as ViewModel>::Widgets>>,
    _view_model: Rc<RefCell<Configuration::ViewModel>>,
    root_widget: <<Configuration::ViewModel as ViewModel>::Widgets as relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>>::Root,
    sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>,
    _redraw_sender: Sender<RedrawMessages>,
}

impl<Configuration> std::fmt::Debug for StoreViewComponent<Configuration> 
where 
    Configuration: ?Sized + StoreViewPrototype + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoreViewComponent")
            .field("view", &self.view)
            .finish_non_exhaustive()
    }
}


impl<Configuration> StoreViewComponent<Configuration> 
where 
    Configuration: ?Sized + StoreViewPrototype + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel> + FactoryContainerWidgets<Configuration>,
    <Configuration::ViewModel as ViewModel>::Components: relm4::Components<Configuration::ViewModel> + StoreViewInnerComponent<Configuration::ViewModel>,
{
    /// Creates new instance of the [StoreViewInterface]
    pub fn new(
        parent_view_model: &Configuration::ParentViewModel,
        // parent_widgets: &<Configuration::ParentViewModel as ViewModel>::Widgets,
        store: Configuration::Store, 
        size: StoreSize
    ) -> Self {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let handler_sender = sender.clone();
        let redraw_handler_sender = sender.clone();
        let (redraw_sender, redraw_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let handler_redraw_sender = redraw_sender.clone();

        let view = Configuration::init_store_view(store, size, redraw_sender.clone());
        let redraw_handler_view = view.clone();

        let view_model = Configuration::init_view_model(parent_view_model, &view);
        let mut components = <<Configuration::ViewModel as ViewModel>::Components as relm4::Components<Configuration::ViewModel>>::init_components(&view_model, sender.clone());
        let container = {
            <Configuration::ViewModel as ViewModel>::Widgets::init_view(
                &view_model,
                &components,
                sender.clone(),
            )
        };
        components.connect_parent(&container);

        // container.connect_components(&view_model, &components);
        let shared_components = Rc::new(RefCell::new(components));
        let redraw_handler_components = shared_components.clone();

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
                        Configuration::view(&mut view_model, msg, handler_sender.clone());
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
                log::trace!("Received redraw message!");
                if let Ok(view_model) = redraw_handler_view_model.try_borrow() {
                    if let Ok(mut container) = redraw_handler_container.try_borrow_mut() {
                        log::trace!("Store view queue size: {}", redraw_handler_view.inbox_queue_size());
                        if redraw_handler_view.inbox_queue_size() > 0 { //only redraw if there is an update awaiting
                            log::trace!("Updating the store view");
                            redraw_handler_view.generate(container.container_widget(), redraw_handler_sender.clone());
                        }
                        container.view(&view_model, redraw_handler_sender.clone());
                        if let Ok(mut handler_components) = redraw_handler_components.try_borrow_mut() {
                            handler_components.on_store_update();
                        }
                        else {
                            log::warn!(target: "relm4-store", "Could not borrow the components. Make sure you dropped all references to components after user");    
                        }
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

        Self {
            view,
            _components: shared_components,
            _container: shared_container,
            _view_model: shared_view_model,
            root_widget,
            sender,
            _redraw_sender: redraw_sender,
        }
    }

    /// Returns a sender for this component
    pub fn sender(&self) -> Sender<<Configuration::ViewModel as ViewModel>::Msg> {
        self.sender.clone()
    }

    /// Sends a message to this component
    pub fn send(&self, msg: <Configuration::ViewModel as ViewModel>::Msg) -> Result<(), std::sync::mpsc::SendError<<Configuration::ViewModel as ViewModel>::Msg>> {
        self.sender.send(msg)
    }

    /// Returns root widget for this component, in most cases gtk widget
    pub fn root_widget(&self) -> &<<Configuration::ViewModel as ViewModel>::Widgets as relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>>::Root {
        &self.root_widget
    }
}