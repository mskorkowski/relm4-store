//! Create contains implementation of the store view

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod implementation;
mod widgets;
mod window_changeset;

use reexport::glib;
use reexport::log;
use reexport::relm4::factory::Factory;
use reexport::relm4::factory::FactoryPrototype;
use reexport::relm4::factory::FactoryView;
use store::StoreView;
use store::StoreViewMsg;
use store::math::Range;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use implementation::StoreViewImplementation;
use record::Id;
use record::Identifiable;
use record::TemporaryIdAllocator;
use record::Record;
use reexport::relm4;
use reexport::relm4::Sender;
use reexport::relm4::Model as ViewModel;
use store::DataStore;
use store::StoreId;
use store::StoreSize;
use store::StoreViewPrototype;
use store::redraw_messages::RedrawMessages;
pub use window_changeset::WindowChangeset;
use store::Pagination;

/// StoreView implementation
pub struct View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    id: StoreId<Self>,
    implementation: Rc<RefCell<StoreViewImplementation<Configuration>>>,
    #[allow(clippy::type_complexity)]
    connections: Rc<RefCell<HashMap<StoreId<Self>, Sender<StoreViewMsg<<Configuration::Store as DataStore>::Record>>>>>,
    sender: Sender<StoreViewMsg<<Configuration::Store as DataStore>::Record>>,
    redraw_sender: Sender<RedrawMessages>,
}

impl<Configuration> View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    /// Creates new instance of the View
    pub fn new(store: Configuration::Store, size: StoreSize, redraw_sender: Sender<RedrawMessages>) -> Self {
        let id = StoreId::new();

        let implementation = Rc::new(RefCell::new(
            StoreViewImplementation::new(store.clone(), size.items())
        ));
        let handler_implementation = implementation.clone();
        let handler_redraw_sender = redraw_sender.clone();
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg| {
                if let Ok(implementation) = handler_implementation.try_borrow_mut() {
                    implementation.inbox(msg);
                    log::trace!("StoreView is sending redraw message");
                    handler_redraw_sender.send(RedrawMessages::Redraw).expect("Unexpected failure while sending message via redraw_sender");
                }
                else {
                    log::warn!("Unable to borrow mutably the changes. Please drop all the references to changes!");
                }

                glib::Continue(true)
            });
        }

        store.listen(id.transfer(), sender.clone());

        Self{
            id,
            implementation,
            connections: Rc::new(RefCell::new(HashMap::new())),
            sender,
            redraw_sender,
        }
    }
}

impl<Configuration> Identifiable<Self, <<Configuration::Store as DataStore>::Allocator as TemporaryIdAllocator>::Type> for View<Configuration>
where
    Configuration: 'static + ?Sized + StoreViewPrototype,
{
    type Id = StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Configuration> DataStore for View<Configuration> 
where
    Configuration: ?Sized + StoreViewPrototype + 'static
{
    type Record = <Configuration::Store as DataStore>::Record;
    type Allocator = <Configuration::Store as DataStore>::Allocator;
    type Messages = StoreViewMsg<Self::Record>;

    fn len(&self) -> usize {
        self.implementation.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.implementation.borrow().is_empty()
    }

    fn get(&self, id: &record::Id<Self::Record>) -> Option<Self::Record> {
        self.implementation.borrow().get(id)
    }

    fn get_range(&self, range: &store::math::Range) -> Vec<Self::Record> {
        self.implementation.borrow().get_range(range)
    }

    fn listen(&self, store_id: StoreId<Self>, sender: Sender<StoreViewMsg<Self::Record>>) {
        self.connections.borrow_mut().insert(store_id, sender);
    }

    fn unlisten(&self, store_id: StoreId<Self>) {
        self.connections.borrow_mut().remove(&store_id);
    }

    fn sender(&self) -> Sender<store::StoreViewMsg<Self::Record>> {
        self.sender.clone()
    }

    fn send(&self, msg: store::StoreViewMsg<Self::Record>) {
        self.sender.send(msg).expect("Unable to send message. ???")
    }
}

impl<Configuration> StoreView for View<Configuration> 
where
    Configuration: ?Sized + StoreViewPrototype + 'static
{
    type Configuration = Configuration;

    fn window_size(&self) -> usize {
        self.implementation.borrow().window_size()
    }

    fn get_window(&self) -> store::math::Range {
        self.implementation.borrow().get_window()
    }

    fn set_window(&self, range: store::math::Range) {
        self.implementation.borrow().set_window(range);
        self.send(StoreViewMsg::Reload);
    }

    fn get_view_data(&self) -> Vec<store::RecordWithLocation<Self::Record>> {
        self.implementation.borrow().get_view_data()
    }

    fn current_len(&self) -> usize {
        self.implementation.borrow().current_len()
    }

    fn get_position(&self, id: &record::Id<Self::Record>) -> Option<store::Position> {
        self.implementation.borrow().get_position(id)
    }

    fn next_page(&self) {
        let size = self.window_size();
        let range = self.get_window().to_right(size);
        self.set_window(range);
    }

    fn prev_page(&self) {
        let size = self.window_size();
        let range = self.get_window().to_left(size);
        self.set_window(range);
    }

    fn first_page(&self) {
        let range = self.get_window().slide(0);
        self.set_window(range);
    }

    fn last_page(&self) {
        let total_pages = self.total_pages();
        let size = self.window_size();
        let range = if total_pages > 0 {
            let last_page_start = (total_pages-1)*size;
            Range ::new(last_page_start, last_page_start+size)
        }
        else {
            Range::new(0, size)
        };
        self.set_window(range);
    }

    fn inbox_queue_size(&self) -> usize {
        self.implementation.borrow().inbox_queue_size()
    }
}

impl<Configuration> FactoryPrototype for View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
{
    type Factory = Self;
    type Msg = <Configuration::ViewModel as ViewModel>::Msg;
    type Widgets = Configuration::RecordWidgets;
    type Root = Configuration::Root;
    type View = Configuration::View;

    fn init_view(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>,
    ) -> Self::Widgets {
        let model = self.get(key).expect("Key doesn't point to the model in the store while generating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::init_view(&model, position, sender)
    }

    /// Set the widget position upon creation, useful for [`gtk::Grid`] or similar.
    fn position(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
        let model = self.get(key).expect("Key doesn't point to the model in the store while positioning! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::position(model, position)
    }

    /// Function called when self is modified.
    fn view(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        let model = self.get(key).expect("Key doesn't point to the model in the store while updating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        <Configuration as StoreViewPrototype>::view_record(model, position, widgets)
    }

    /// Get the outermost widget from the widgets.
    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        Configuration::root_widget(widgets)
    }
}

impl<Configuration> Factory<View<Configuration>, Configuration::View> for View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
{
    type Key = Id<<Configuration::Store as DataStore>::Record>;

    fn generate(&self, view: &Configuration::View, sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>) {
        self.implementation.borrow().view(view, sender);
    }
}

/// Implements shallow clone for the View
/// 
/// Derive would require `Configuration` to implement `Clone` which is
/// not required
impl<Configuration> Clone for View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    fn clone(&self) -> Self {
        Self{
            id: self.id,
            implementation: self.implementation.clone(),
            connections: self.connections.clone(),
            sender: self.sender.clone(),
            redraw_sender: self.redraw_sender.clone(),
        }
    }
}


/// Implements debug for the View
/// 
/// Derive would require `Configuration` to implement `std::fmt::Debug` which
/// is not required
impl<Configuration> std::fmt::Debug for View<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("View")
            .field("id", &self.id)
            .field("implementation", &self.implementation)
            .field("connections", &self.connections)
            .field("sender", &self.sender)
            .field("redraw_sender", &self.redraw_sender)
            .finish()
    }
}