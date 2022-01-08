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

mod factory_prototype;
pub mod math;
mod pagination;
mod position;
mod record_with_location;
pub mod redraw_messages;
mod store_id;
mod store_msg;
mod store_size;
mod store;
mod store_view_component;
pub mod window;

use reexport::relm4;

use std::fmt::Debug;
use std::marker::Sized;

use relm4::Sender;

use record::Id;
use record::Identifiable;
use record::Record;
use record::TemporaryIdAllocator;

use crate::math::Range;

pub use factory_prototype::StoreViewPrototype;
pub use factory_prototype::FactoryContainerWidgets;
pub use factory_prototype::StoreViewInnerComponent;
pub use pagination::Pagination;
pub use position::Position;
pub use record_with_location::RecordWithLocation;
pub use store::Store;
pub use store_id::StoreId;
pub use store_msg::StoreMsg;
pub use store_size::StoreSize;
pub use store_view_component::StoreViewComponent;
pub use store_view_component::StoreViewInterfaceError;

/// DataStore is a trait describing collections specialized in housekeeping business model data
/// 
/// DataStore is designed with upsert in mind.
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
pub trait DataStore: Identifiable<Self, <Self::Allocator as TemporaryIdAllocator>::Type, Id=StoreId<Self>> 
{
    /// Type of records kept in the data store
    type Record: Record + Debug + Clone + 'static;

    /// Id allocator for this data store
    /// 
    /// ## TL;DR;
    /// 
    /// You should keep it as
    /// 
    /// ```text
    /// type Allocator = DefaultIdAllocator;
    /// ```
    /// 
    /// ## Longer version
    /// 
    /// As long as possible (and little bit longer) you should use `[DefaultIdAllocator]` Overriding it
    /// might be necessary in some super rare cases where you have more then one data store for the same
    /// kind of data or you have dynamic number of data stores of given kind. Both of the cases are in the
    /// "please don't do that" area from design perspective. 
    /// 
    /// If you are reading this section in 99% of cases creating a custom DataStore which will be backed by
    /// more then one backend is the proper solution for your issues.
    type Allocator: TemporaryIdAllocator;

    /// Registers message in the data store
    /// 
    /// Data store might handle it immediately or might do queue it for later. It's up to the store
    /// implementation what to do with a message
    // fn inbox(&self, m: StoreMsg<Self::Record>);

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

    /// Attaches sender to the store
    /// 
    /// Sender is used to send a message whenever there are changes in the store
    fn listen(&self, id: StoreId<Self>, sender: Sender<StoreMsg<Self::Record>>);

    /// Removes handler from the store
    /// 
    /// Changes to the store will not be delivered after this handler is removed
    fn unlisten(&self, handler_ref: StoreId<Self>);

    /// Returns sender for this store
    fn sender(&self) -> Sender<StoreMsg<Self::Record>>;

    /// Sends a message to this store
    fn send(&self, msg: StoreMsg<Self::Record>);
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
pub trait StoreView: DataStore
{
    /// Type describing configuration parts of the store view behavior
    type Configuration: ?Sized + StoreViewPrototype;

    /// How many records should be visible at any point of time
    /// 
    /// If there is less elements in the store/page it's possible that less records will be shown.
    /// StoreView should never show more records then this value
    fn window_size(&self) -> usize;

    /// Returns range in the parent store for data in the current window
    fn get_window(&self) -> Range;

    /// Moves the window to the new range
    fn set_window(&self, range: Range);

    /// Returns vector with list of records in the current view
    /// 
    /// Returned records are **clones** of the actual records
    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Record>>;

    /// Returns current number of elements visible via the store view
    /// 
    /// Always a number between `[0, [StoreView::window_size])`
    fn current_len(&self) -> usize;

    /// Returns the position of the record in the view
    /// 
    /// If returns `None` that means record is not in the view
    fn get_position(&self, id: &Id<Self::Record>) -> Option<Position>;

    /// Advance the store view to the next page (if it exists) of underlying store
    fn next_page(&self);

    /// Goes back the the previous page (if it exists) of underlying store
    fn prev_page(&self);

    /// Goes to the first page of data in the underlying store
    fn first_page(&self);

    /// Goes to the last page of the data in the underlying store
    fn last_page(&self);

    /// Returns current size of unhandled messages in the view
    fn inbox_queue_size(&self) -> usize;
}

/// Structure used by backends to send back information about what should be sent to the views
/// after updates are done on the backend side
#[derive(Debug)]
pub struct Replies<Record> 
where Record: record::Record + Debug + Clone + 'static
{
    /// List of messages to be sent to the store views
    pub replies: Vec<StoreMsg<Record>>,
}

/// This trait should be implemented for backends of the data stores
/// 
/// It's basically `DataStore - Sender`
pub trait Backend {
    /// Type of records kept in the data store
    type Record: Record + Debug + Clone + 'static;

    /// Registers message in the data store
    /// 
    /// Data store might handle it immediately or might do queue it for later. It's up to the store
    /// implementation what to do with a message
    // fn inbox(&self, m: StoreMsg<Self::Record>);

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


    /// Handles messages
    fn inbox(&mut self, msg: StoreMsg<Self::Record>) -> Replies<Self::Record>;
}

/// Default trait describing how the records should be sorted by backend
pub trait Sorter<Record: record::Record>: Copy + Debug {
    /// Compares `lhs` with `rhs`
    /// 
    /// If sorter is being used implementation assumes that `cmp` constitutes a total order
    fn cmp(&self, lhs: &Record, rhs: &Record) -> std::cmp::Ordering;
}


/// Trait implemented by the data store which supports switching of the natural order
/// 
/// If you use [`Store`] then it will use `OrderBy` defined by the backend
pub trait OrderedStore<OrderBy>: DataStore {
    /// sets natural order of the records
    fn set_order(&self, order: OrderBy);
}

/// Trait implemented by the data store backends supporting switching of the natural order
/// 
/// When you define your own implementation of the [`Backend`] in most cases
/// it will look like:
/// 
/// ```text
/// trait MyBackend<OrderBy: Sorted<Record>>{
///    ...
/// }
/// ```
/// 
/// This implementation doesn't limit what `OrderBy` is so you have as much of freedom as
/// possible when you need to do something special
pub trait OrderedBackend<OrderBy>: Backend {
    /// sets natural order of the records
    fn set_order(&mut self, order: OrderBy) -> Replies<Self::Record>;
}