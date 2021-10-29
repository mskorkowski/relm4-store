use std::marker::PhantomData;

use super::DataStore;
use super::DataStoreBase;
use super::Handler;
use super::StoreMsg;

pub struct HandlerWrapper<Store, OtherStore>
where
    OtherStore: DataStore + ?Sized,
    Store: DataStore<Model=<OtherStore as DataStoreBase>::Model> + ?Sized,
{
    parent: Box<dyn Handler<Store>>,
    _other: PhantomData<*const OtherStore>,
}

impl<Store, OtherStore> HandlerWrapper<Store, OtherStore>
where
    OtherStore: DataStore + ?Sized + 'static,
    Store: DataStore<Model=<OtherStore as DataStoreBase>::Model> + ?Sized + 'static,
{
    pub fn from(parent: Box<dyn Handler<Store>>) -> Box<dyn Handler<OtherStore>> {
        Box::new(
            HandlerWrapper{
                _other: PhantomData,
                parent
            }
        )
    }
}

impl<Store, OtherStore> Handler<OtherStore> for HandlerWrapper<Store, OtherStore>
where
    OtherStore: DataStore + ?Sized,
    Store: DataStore<Model=<OtherStore as DataStoreBase>::Model> + ?Sized,
{
    fn handle(&self, message: StoreMsg<<OtherStore as DataStoreBase>::Model>) -> bool {
        self.parent.handle(message)
    }
}