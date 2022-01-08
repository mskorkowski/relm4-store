use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Result as FmtResult;
use std::hash::Hash;
use std::hash::Hasher;

use record::Identity;
use record::TemporaryIdAllocator;

use super::DataStore;

/// Identifier used by store
/// 
/// Each store is required to have only one id. Value identifies the store which issued id, 
/// type identifies the store which holds the id. 
/// 
/// You should treat this identifier as stable only during lifetime of an application. You should
/// never depend on this id as a way to describe relationships between the data in your business model.
/// 
/// This class only provides [StoreId::default] and argumentless [StoreId::new]. To prevent you to depend
/// on store id in persistent way. Of course there are ways around it, but it's painful enough that I hope
/// you will drop any notions of storing instances of this struct.
/// 
/// If you need to pass id's between stores you can use [StoreId::transfer] method to create
/// a copy of an id with new holder association as long as other store is using same kind
/// of allocator.
/// 
/// Allocator must provide globally unique identifiers.
/// 
/// ## To writers of the crates with stores
/// 
/// If you write something which could be used by others, you would help your users if
/// you expose the allocator to be set. You can always add `Allocator=DefaultIdAllocator` as part of
/// definition for users who play by the rules. If you have a reason why id must be of the given type
/// please let users know about it with big red box at the beginning o your documentation. This could
/// easily be a deal breaker for them if they need to change allocator in their app or can't adjust
/// the allocator for the store from your library.
/// 
/// ## To ones who wish to change the allocator
/// 
/// You shouldn't change allocator used by the store. UUID's are prefect since they guarantee
/// uniqueness. Once again don't change the allocator. Stability of this id is required only
/// during application runtime.
/// 
/// Since you must play with id allocators make sure you know what you are doing and prepare
/// good backing for your allocator so you always get the unique id. You've been warned.
/// 
/// Before you change the allocator type make sure that you know your store interactions.
/// In some cases store-store interaction might be required and it will make your life hard.
/// 
/// In most cases it will be painless but in more advanced use cases you might get a wall of
/// errors after the change related to the different kind of allocator used by different stores.
/// In such a case you basically need to figure out a way to either have two kinds of id's for
/// your store and live with it or create some kind of translation layer. Both solutions have
/// a lot of issues by themselves. 
/// 
/// PS. If you change the default allocator for just part of stores and do something more 
/// interesting then just store to store-view I wish you good luck. Sincerely yours, author. 
pub struct StoreId<Store>
where
    Store: ?Sized + DataStore,
{
    value: <Store::Allocator as TemporaryIdAllocator>::Type,
}

impl<Store> StoreId<Store> 
where
    Store: ?Sized + DataStore,
{
    /// Creates new instance of [StoreId]
    pub fn new() -> Self {
        StoreId {
            value: <Store::Allocator as TemporaryIdAllocator>::new_id(),
        }
    }

    /// Moves holder assignment for given StoreId
    /// 
    /// This is possible if and only if other store's allocator and current store's allocator internal 
    /// id data type is the same
    pub fn transfer<OtherStore>(&self) -> StoreId<OtherStore> 
    where
        OtherStore: ?Sized + DataStore,
        OtherStore::Allocator: TemporaryIdAllocator<Type=<Store::Allocator as TemporaryIdAllocator>::Type>
    {
        StoreId{
            value: self.value,
        }
    }
}

impl<Store> Default for StoreId<Store> 
where
    Store: ?Sized + DataStore,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Store> Clone for StoreId<Store> 
where
    Store: ?Sized + DataStore,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value,
        }
    }
}

impl<Store> Hash for StoreId<Store> 
where
    Store: ?Sized + DataStore,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<Store> Copy for StoreId<Store>
where
    Store: ?Sized + DataStore,
{}


impl<Store> PartialEq for StoreId<Store> 
where
    Store: ?Sized + DataStore,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}


impl<Store> Identity<Store, <Store::Allocator as TemporaryIdAllocator>::Type> for StoreId<Store>
where
    Store: ?Sized + DataStore<Id=StoreId<Store>>,
{
    fn get_value(&self) -> <Store::Allocator as TemporaryIdAllocator>::Type {
        self.value
    }
}

impl<Store> Eq for StoreId<Store> 
where
    Store: ?Sized + DataStore,
{}

impl<Store> Debug for StoreId<Store> 
where
    Store: ?Sized + DataStore, 
    <Store::Allocator as TemporaryIdAllocator>::Type: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("StoreId").
            field("value", &self.value).
            finish()
    }
}