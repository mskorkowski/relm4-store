
use std::cmp::Eq;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::FromIterator;

use super::Identity;
use super::Record;
use super::TemporaryIdAllocator;

/// Id for models
/// 
/// There are two possible values for it, `New` and `Identifier`.
/// `New` is used for models which were created but not "persisted" yet in
/// store.
/// 
/// `Identifier` is used for models already persisted.
/// 
/// ## Do I need to care?
/// 
/// If you implement a store definitely. If you implement view in most cases it's
/// neutral for you.
/// 
/// From my (authors) experience in most cases editing and creating new record are
/// different enough that you don't need to care about it or it's natural to persist
/// record before showing it to the user.
/// 
/// ## Why ?
/// 
/// This allows to easily implement logic for cases where new records needs to be
/// available to user before it has been persisted. For example if you create a system
/// where users can import values from csv file and would like to allow them to review
/// data inserted, then you can create a new record for each row in csv file and show
/// it with temporary `Id::New`. If user will decide to write records into the store
/// then value will be replaced with `Id::Identifier` in the model by the persistance
/// layer. Now store has an ability to notify all interested parties, that record was
/// saved and it's id was updated from ephemeral `Id::New` to stable one.
/// 
/// It's also useful when you need to keep track of relationships between models which
/// are not persisted yet.
/// 
/// Another use case could be sending messages to remote system, where you can show user
/// a message which was send and when response about persisted message comes back you can
/// update ui to reflect that.
/// 
/// ## Something different then uuid
/// 
/// If you would like to have an id with values which differ from uuid you should
/// implement your own [TemporaryIdAllocator] and set it as the `[Record::Allocator]`
pub enum Id<T> 
where
    T: ?Sized + Record,
{
    /// Id for records which were not committed yet to store
    New{
        /// Value of the id
        value: <T::Allocator as TemporaryIdAllocator>::Type,
    },
    /// Id for records which are persisted already
    /// 
    /// What persisted means depends on the store.
    Permanent {
        /// Value of the id
        value: <T::Allocator as TemporaryIdAllocator>::Type,
    }
}

impl<T> Id<T> 
where
    T: ?Sized + Record,
{
    /// Returns `true` if id has not been committed to store yet
    pub fn is_new(&self) -> bool {
        match self {
            Id::New{..} => true,
            Id::Permanent{..} => false
        }
    }
}

impl<T> Identity<T, <T::Allocator as TemporaryIdAllocator>::Type> for Id<T> 
where
    T: ?Sized + Record,
{
    fn get_value(&self) -> <T::Allocator as TemporaryIdAllocator>::Type {
        match self {
            Id::New{value, ..} => *value,
            Id::Permanent{value, ..} => *value,
        }
    }
}

impl<T> Clone for Id<T> 
where
    T: ?Sized + Record,
{
    fn clone(&self) -> Self {
        match self {
            Id::New{value, ..} => Id::New{
                value: *value,
                // _t: PhantomData,
            },
            Id::Permanent{value, ..} => Id::Permanent{
                value: *value,
            }
        }
    }
}

impl<T> Copy for Id<T> 
where
    T: ?Sized + Record,
{}

impl<T> fmt::Display for Id<T> 
where
    <T::Allocator as TemporaryIdAllocator>::Type: fmt::Display,
    T: ?Sized + Record,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::New{value, ..} => value.fmt(f),
            Id::Permanent{value, ..} => value.fmt(f),
        }
        
    }
}

impl<T> Id<T> 
where
    T: ?Sized + Record,
{
    /// Creates new instance of the Id
    /// 
    /// Returns new temporary id
    #[must_use]
    pub fn new() -> Self {
        Id::New {
            value: T::Allocator::new_id(),
        }
    }

    /// Creates new instance of the Id
    ///
    /// Returns permanent id
    pub fn from(value: <T::Allocator as TemporaryIdAllocator>::Type) -> Self {
        Id::Permanent {
            value,
        }
    }
}

impl<T> Default for Id<T> 
where
    T: ?Sized + Record,
{
    fn default() -> Self {
        Id::new()
    }
}

impl<T> PartialEq for Id<T> 
where
    T: ?Sized + Record,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Id::New{value: lhs, ..}, Id::New{value: rhs, ..}) => lhs.eq(rhs),
            (Id::Permanent{value: lhs, ..}, Id::Permanent{value: rhs, ..}) => lhs.eq(rhs),
            _ => false
        }
    }
}

impl<T> Eq for Id<T> 
where
    T: ?Sized + Record,
{}

impl<T>Hash for Id<T> 
where
    T: ?Sized + Record,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Id::New{value, ..} => value.hash(state),
            Id::Permanent{value, ..} => value.hash(state)
        }
    }
}

impl<T> fmt::Debug for Id<T> 
where
    <T::Allocator as TemporaryIdAllocator>::Type: fmt::Debug,
    T: ?Sized + Record,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::New{value, ..} => 
                f.debug_tuple("Id::New").field(value).finish(),
            Id::Permanent{value, ..} =>
                f.debug_tuple("Id::Permanent").field(value).finish(),
        }

    }
}

impl<T> FromIterator<&'static Id<T>> for HashSet<Id<T>> 
where
    T: 'static + ?Sized + Record,
{
    fn from_iter<II: IntoIterator<Item = &'static Id<T>>>(iter: II) -> Self {
        iter.into_iter().map(|v| v.clone()).collect()
    }
}