use reexport::uuid;

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Result as FmtResult;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

use uuid::Uuid;

use model::Identity;

use super::DataStore;
use super::IdentifiableStore;

pub struct StoreId<Store>
where
    Store: ?Sized + IdentifiableStore<Id=StoreId<Store>>,
{
    value: Uuid,
    _store: PhantomData<*const Store>,
}

impl<Store> StoreId<Store> 
where
    Store: IdentifiableStore<Id=StoreId<Store>> + ?Sized
{
    pub fn new() -> Self {
        StoreId {
            value: Uuid::new_v4(),
            _store: PhantomData,
        }
    }

    pub fn transfer<OtherStore>(&self) -> StoreId<OtherStore> 
    where
        OtherStore: IdentifiableStore<Id=StoreId<OtherStore>> + ?Sized
    {
        StoreId{
            value: self.value,
            _store: PhantomData,
        }
    }
}

impl<Store> Default for StoreId<Store> 
where
    Store: IdentifiableStore<Id=StoreId<Store>> + ?Sized
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Store> Clone for StoreId<Store> 
where
    Store: ?Sized + IdentifiableStore
{
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _store: PhantomData
        }
    }
}

impl<Store> Hash for StoreId<Store> 
where
    Store: DataStore + ?Sized,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<Store> Copy for StoreId<Store>
where
    Store: ?Sized + DataStore{}

impl<Store> PartialEq for StoreId<Store> 
where
    Store: DataStore + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}


impl<Store> Identity<Store> for StoreId<Store>
where
    Store: ?Sized + DataStore<Id=StoreId<Store>>,
{
    fn get_value(&self) -> Uuid {
        self.value
    }
}

impl<Store: ?Sized + DataStore> Eq for StoreId<Store> {}

impl<Store: ?Sized + DataStore> Debug for StoreId<Store> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("StoreId").
            field("value", &self.value).
            finish()
    }
}