//! Provides implementation of identifier for Models

use std::cmp::Eq;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use reexport::uuid::Uuid;

pub trait Identity<T: ?Sized> {
    fn get_value(&self) -> Uuid;
}

pub trait Identifiable {
    type Id: Identity<Self> + ?Sized;
    
    fn get_id(&self) -> Self::Id;
}

pub trait Model: Identifiable<Id=Id<Self>> {}

/// Id for models
/// 
/// 
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
pub enum Id<T: ?Sized + Model> {
    /// Id for records which were not committed yet to store
    New{
        value: Uuid,
        phantom: PhantomData<*const T>,
    },
    /// Id for records which are persisted already
    /// 
    /// What persisted means depends on the store.
    Identifier {
        value: Uuid,
        phantom: PhantomData<*const T>,
    }
}

impl<T: ?Sized + Model> Id<T> {
    /// Returns `true` if id has not been committed to store yet
    pub fn is_new(&self) -> bool {
        match self {
            Id::New{..} => true,
            Id::Identifier{..} => false
        }
    }
}

impl<T: ?Sized + Model> Identity<T> for Id<T> {
    fn get_value(&self) -> Uuid {
        match self {
            Id::New{value, ..} => *value,
            Id::Identifier{value, ..} => *value,
        }
    }
}

impl<T: ?Sized + Model> Clone for Id<T> {
    fn clone(&self) -> Self {
        match self {
            Id::New{value, ..} => Id::New{
                value: *value,
                phantom: PhantomData
            },
            Id::Identifier{value, ..} => Id::Identifier{
                value: *value,
                phantom: PhantomData,
            }
        }
    }
}

impl<T: ?Sized + Model> Copy for Id<T> {}

impl<T: ?Sized + Model> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::New{value, ..} => value.fmt(f),
            Id::Identifier{value, ..} => value.fmt(f),
        }
        
    }
}

impl<T: ?Sized + Model> Id<T> {
    #[must_use]
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();

        Id::New {
            value: uuid,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized + Model> Default for Id<T> {
    fn default() -> Self {
        Id::new()
    }
}

impl<T: ?Sized + Model> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Id::New{value: lhs, ..}, Id::New{value: rhs, ..}) => lhs.eq(rhs),
            (Id::Identifier{value: lhs, ..}, Id::Identifier{value: rhs, ..}) => lhs.eq(rhs),
            _ => false
        }
    }
}

impl<T: ?Sized + Model> Eq for Id<T> {}

impl<T: ?Sized + Model>Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Id::New{value, ..} => value.hash(state),
            Id::Identifier{value, ..} => value.hash(state)
        }
    }
}

impl<T: ?Sized + Model> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::New{value, ..} => 
                f.write_str(&format!("Id::New({})", value)),
            Id::Identifier{value, ..} =>
                f.write_str(&format!("Id::Identifier({})", value)),
        }

    }
}