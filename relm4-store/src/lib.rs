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

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod factory_configuration;
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
pub mod window;

use reexport::relm4;


use std::fmt::Debug;
use std::marker::Sized;
use relm4::Model as ViewModel;

use record::DefaultIdAllocator;
use record::Id;
use record::Identifiable;
use record::Record;
use record::TemporaryIdAllocator;

use crate::math::Range;

pub use factory_configuration::FactoryConfiguration;
pub use factory_configuration::FactoryContainerWidgets;
use handler_wrapper::HandlerWrapper;
pub use pagination::Pagination;
pub use position::Position;
pub use record_with_location::RecordWithLocation;
pub use store_id::StoreId;
pub use store_msg::StoreMsg;
pub use store_size::StoreSize;
pub use store_view_implementation::StoreViewImplementation;
pub use store_view_implementation::StoreViewImplHandler;
pub use store_view_interface::StoreViewInterface;

/// Implementations of this trait are used to send messages between the store and it's views
pub trait Handler<Store: DataStore<Allocator> + ?Sized, Allocator: TemporaryIdAllocator> {
    /// Method called when parent store needs to pass a message to the view
    fn handle(&self, message: StoreMsg<<Store as DataStore<Allocator>>::Record>) -> bool;
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
/// 
/// ## To the writers of the store
/// 
/// If you implement a store to be used by the external applications do yourself and users a favor and 
/// please
/// 
/// 1. Expose StoreId allocator
/// 2. Make sure your store will work with any sufficiently good enough `Id<Record>` without depending
/// on the exact underlying id type. If you require any property to be present for an `Id` just add it
/// to the list of requirements of your store and ask user to give you that.
/// 
/// If there are limitation on that please somewhere at the beginning of the docs note what limitations
/// are around this values. It can easily become a deal breaker for users of your library.
pub trait DataStore<Allocator>: Identifiable<Self, Allocator::Type, Id=StoreId<Self, Allocator>> 
where Allocator: TemporaryIdAllocator
{
    /// Type of records kept in the data store
    type Record: Record + Debug + Clone;

    /// Registers message in the data store
    /// 
    /// Data store might handle it immediately or might do queue it for later. It's up to the store
    /// implementation what to do with a message
    fn inbox(&self, m: StoreMsg<Self::Record>);

    /// Total amount of available records in the store
    fn len(&self) -> usize;

    /// Returns true if store doesn't contain any records yet
    fn is_empty(&self) -> bool;

    /// Returns the record from the store
    /// 
    /// If returns [None] then it means there is no such record in the store
    fn get(&self, id: &Id<Self::Record>) -> Option<Self::Record>;

    /// Returns records which are in the store at the given range
    ///
    /// Returned vector doesn't need to be ordered by position.
    /// If range is out of bounds returned vector will be empty.
    fn get_range(&self, range: &Range) -> Vec<Self::Record>;

    /// Attaches handler to the store
    /// 
    /// Handler is fired whenever there is a change in the store
    fn listen(&self, id: StoreId<Self, Allocator>, h: Box<dyn Handler<Self, Allocator>>);

    /// Removes handler from the store
    /// 
    /// Changes to the store will not be delivered after this handler is removed
    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>);
}

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
pub trait StoreView<Allocator>: DataStore<Allocator>
where
    Allocator: TemporaryIdAllocator,
{
    type Builder: FactoryConfiguration<Allocator>;
    fn window_size(&self) -> usize;
    fn get_window(&self) -> Range;
    fn set_window(&self, range: Range);
    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Record>>;

    /// Returns the position of the record in the view
    /// 
    /// If returns `None` that means record is not in the view
    fn get_position(&self, id: &Id<Self::Record>) -> Option<Position>;

    fn next_page(&self);
    fn prev_page(&self);
    fn first_page(&self);
    fn last_page(&self);

    fn inbox_queue_size(&self) -> usize;
}

pub trait Source<Allocator=DefaultIdAllocator> 
where
    Allocator: TemporaryIdAllocator,
{
    type ParentViewModel : ViewModel;
    type SV: StoreView<Allocator>;

    fn store(parent_model: &Self::ParentViewModel) -> Self::SV;
}
