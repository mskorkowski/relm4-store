//! Provides implementation of identifier for records in the store
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod id;
mod uuid_allocator;

use std::fmt::Debug;
use std::hash::Hash;

pub use id::Id;
pub use uuid_allocator::UuidAllocator;

/// Default allocator used for id
pub type DefaultIdAllocator = UuidAllocator;

/// Error returned when there is an issue with an identifier
#[derive(Debug)]
pub struct IdentityError(pub &'static str);

/// Trait which describes identifier
/// 
/// - **T** type for which this is an identifier. This makes the id's for different objects to be distinguishable
/// - **Type** Type of the id
pub trait Identity<T: ?Sized, Type> {
    /// Returns value of the identifier
    fn get_value(&self) -> Type;
}

/// Trait for values which need identification but are not records so they don't need the New/Permanent logic
pub trait Identifiable<T: ?Sized, Type> {
    /// Type of identifier
    type Id: Identity<Self, Type> + ?Sized;
    /// Returns the id of this object
    fn get_id(&self) -> Self::Id;
}

/// Definition of the record in the data store
/// 
/// By default it's using uuid as identifiers so it's almost impossible to generate the collision for two records
pub trait Record: Clone {
    /// This defines how this record will get it's id's.
    /// 
    /// If you don't care about id's (do not store your records in the database) then [`DefaultIdAllocator`] will do the job.
    /// If you store your records in the MySql, Postgresql or anything else you should create appropriate id allocator
    type Allocator: TemporaryIdAllocator;

    /// Returns the id of this object
    fn get_id(&self) -> Id<Self>;

    /// Updates record to use permanent
    fn set_permanent_id(&mut self, value: <Self::Allocator as TemporaryIdAllocator>::Type) -> Result<(), IdentityError>;
}

/// Provides a way to create temporary id's
pub trait TemporaryIdAllocator: Clone + Debug {
    /// Type of values on which `Id` is based of
    /// 
    /// This type defines memory representation of the id for the record
    type Type: Copy + PartialEq + Hash + Eq + std::fmt::Debug;
    /// Returns value of new **temporary** id
    /// 
    /// Every call must return new different value otherwise it's possible to have a conflict which could end up
    /// with data loss
    fn new_id() -> Self::Type;
}

