//! Contains definition of the Store
//! 
//! ## What it is?
//! 
//! 1. [DataStore] is dynamic list of values. 
//! 2. Storage of values is not defined for [DataStore]. It might be in memory, or SQL, or csv file, or whatever else
//! 3. [DataStore] supports views. View tracks subset of the data kept in the store.
//! 
//! ## Why?
//! 
//! Relm4 is notorious in mixing business logic and view. Stores allows to give a strict separation between 
//! business model and what and how it's shown to the user.
//! 
//! As side effect it simplifies relm4 factories usage.

mod handler_wrapper;
pub mod math;
mod pagination;
mod position;
mod record_with_location;
mod store_id;
mod store_msg;
mod store_size;
mod store_view_implementation;
mod store_view_interface;
pub mod widgets;
mod window_changeset;

use reexport::gtk;
use reexport::relm4;

use std::fmt::Debug;
use std::marker::Sized;
use gtk::prelude::WidgetExt;
use gtk::glib::Sender;
use relm4::Model as ViewModel;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use model::Identifiable;
use model::Model;

use crate::math::Range;
use crate::math::Window as DataWindow;

use handler_wrapper::HandlerWrapper;
pub use pagination::Pagination;
pub use position::Position;
pub use record_with_location::RecordWithLocation;
pub use store_id::StoreId;
pub use store_msg::StoreMsg;
pub use store_size::StoreSize;
use store_view_implementation::StoreViewImpl;
use store_view_implementation::StoreViewImplHandler;
use window_changeset::WindowChangeset;
pub use store_view_interface::StoreViewInterface;

/// Implementations of this trait are used to send messages between the store and it's views
pub trait Handler<Store: DataStore + ?Sized> {
    /// Method called when parent store needs to pass a message to the view
    fn handle(&self, message: StoreMsg<<Store as DataStoreBase>::Model>) -> bool;
}

pub trait IdentifiableStore: Identifiable<Id=StoreId<Self>> {}

pub trait DataStoreBase: IdentifiableStore {
    type Model: Model + Debug + Clone;

    fn inbox(&self, m: StoreMsg<Self::Model>);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, id: &<Self::Model as Identifiable>::Id) -> Option<(Position, Self::Model)>;

    /// Returns records which are in the store at the given range
    ///
    /// Returned vector doesn't need to be ordered by position.
    /// If range is out of bounds returned vector will be empty.
    fn get_range(&self, range: &Range) -> Vec<RecordWithLocation<Self::Model>>;
}

pub trait DataStoreListenable: IdentifiableStore + DataStoreBase {
    fn listen(&self, id: StoreId<Self>, h: Box<dyn Handler<Self>>);
    fn unlisten(&self, handler_ref: StoreId<Self>);
}

pub trait DataStore: IdentifiableStore + DataStoreBase + DataStoreListenable{}

pub trait StoreView: DataStore
{
    type Builder: FactoryBuilder;
    fn window_size(&self) -> usize;
    fn get_window(&self) -> Range;
    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Model>>;

    fn next_page(&self);
    fn prev_page(&self);
    fn first_page(&self);
    fn last_page(&self);
}

pub trait FactoryBuilder {
    type Store: DataStore;
    type Widgets: Debug;
    type Root: WidgetExt;
    type View: FactoryView<Self::Root> + FactoryListView<Self::Root>;
    type Window: DataWindow;
    type Msg;

    fn generate(
        record: &<Self::Store as DataStoreBase>::Model,
        position: Position,
        sender: Sender<Self::Msg>,
    ) -> Self::Widgets;

    /// Function called when self is modified.
    fn update(
        model: <Self::Store as DataStoreBase>::Model,
        position: Position,
        widgets: &Self::Widgets,
    );

    fn position(
        model: <Self::Store as DataStoreBase>::Model, 
        position: Position,
    ) -> <Self::View as FactoryView<Self::Root>>::Position;

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root;
}

pub trait Source {
    type ParentViewModel : ViewModel;
    type SV: StoreView;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV;
}
