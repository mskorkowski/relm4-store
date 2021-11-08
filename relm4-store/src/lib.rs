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

mod factory_builder;
mod handler_wrapper;
pub mod math;
mod pagination;
mod position;
mod record_with_location;
mod redraw_messages;
mod store_id;
mod store_msg;
mod store_size;
mod store_view_implementation;
mod store_view_interface;
pub mod widgets;
pub mod window;
mod window_changeset;

use reexport::relm4;

use std::fmt::Debug;
use std::marker::Sized;
use relm4::Model as ViewModel;

use model::Identifiable;
use model::Model;

use crate::math::Range;

pub use factory_builder::FactoryBuilder;
pub use factory_builder::FactoryContainerWidgets;
use handler_wrapper::HandlerWrapper;
pub use pagination::Pagination;
pub use position::Position;
pub use record_with_location::RecordWithLocation;
pub use store_id::StoreId;
pub use store_msg::StoreMsg;
pub use store_size::StoreSize;
pub use store_view_implementation::StoreViewImplementation;
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
    fn get(&self, id: &<Self::Model as Identifiable>::Id) -> Option<Self::Model>;

    /// Returns records which are in the store at the given range
    ///
    /// Returned vector doesn't need to be ordered by position.
    /// If range is out of bounds returned vector will be empty.
    fn get_range(&self, range: &Range) -> Vec<Self::Model>;
}

pub trait DataStoreListenable: IdentifiableStore + DataStoreBase {
    fn listen(&self, id: StoreId<Self>, h: Box<dyn Handler<Self>>);
    fn unlisten(&self, handler_ref: StoreId<Self>);
}

/// DataStore is a trait describing collections specialized in housekeeping business model data
/// 
/// ## DataStore implementations are collections
/// 
/// That means they own (in terms of borrow checker) some instances of your business model data.
/// Also it must give you ability to query your data so you can see what's inside.
/// 
/// ## DataStore implementations are housekeeping your data
/// 
/// DataStore is a place which knows how to mange CRUD operations on your business data.
/// Different implementations of this trait might use different strategies to keep your data. 
/// Some may keep it in the files on the local file system. Some others might store it in database.
/// Wherever your data are being kept data store needs to track when they have been changed and 
/// sometimes how so they can store them. 
/// 
/// ## When can I use it?
/// 
/// You can use data store if it can own the data safely. This boils down to three requirements
/// 
/// - no internal mutation<br>
///   Data store needs to be certain about the data which it holds
/// - they are serializable<br>
///   Data store needs to be able to create a copies of the data
/// - identifiable
///   In presence of copies data store needs to be able to tell which data is just a different 
///   representation of another instance of data
/// 
/// ### No internal mutation
/// 
/// If you create a model which has internal mutability then it would need know to which data store
/// it belongs to so it can notify it about the changes. But data store is collection so it needs to
/// know the data which are kept inside. So we have a perfect circle of dependencies which would make
/// it insane in terms of implementation.
/// 
/// Another reason is that store implementation in really more like simple database or database proxy.
/// As such it must own the "truth" of what the current state of the records is. If your records are
/// internally mutable, you might introduce side effects which are not recorded by the database.
/// 
/// ### They are serializable
/// 
/// DataStore implementations needs to be able to save a data for example in a file and read from it.
/// Also since there is no internal mutation allowed that means data store needs to be able to give you
/// copies of data inside of it so you can mutate them. If you disallow internal mutation rust's [Clone]
/// is good enough for generic case. Specific implementations might have extra serialization requirements. 
/// 
/// ### They are identifiable
/// 
/// DataStore gives you the copy of a data so you can mutate it. Now you would like to update the data
/// in the store. But which data you are talking about? Equality operation is not going to work since
/// you just mutated them. That's way your model needs to provide an identifier which is good enough
/// so the data store can depend on it to identify the instances of the data.
pub trait DataStore: IdentifiableStore + DataStoreBase + DataStoreListenable{}

/// StoreView allows you to access part of the data in the data store
/// 
/// StoreView is a special kind of data store which tracks subset of the data in the other data store.
/// This is useful in various scenarios
/// 
/// - showing data to the user<br>
///   Showing all ten billion of records at the same time is time consuming and will overwhelm user
/// - editing the data<br>
///   Your business model has two data sets `A` and `B` and there is `1-*` relationship between the data.
///   There are valid scenarios when you would like to edit item in `A` and give the ability to modify
///   related items in `B` at the same time. 
pub trait StoreView: DataStore
{
    type Builder: FactoryBuilder;
    fn window_size(&self) -> usize;
    fn get_window(&self) -> Range;
    fn set_window(&self, range: Range);
    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Model>>;

    /// Returns the position of the record in the view
    /// 
    /// If returns `None` that means record is not in the view
    fn get_position(&self, id: &<Self::Model as Identifiable>::Id) -> Option<Position>;

    fn next_page(&self);
    fn prev_page(&self);
    fn first_page(&self);
    fn last_page(&self);

    fn inbox_queue_size(&self) -> usize;
}



pub trait Source {
    type ParentViewModel : ViewModel;
    type SV: StoreView;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV;
}
