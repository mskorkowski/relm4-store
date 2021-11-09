use std::marker::PhantomData;

use record::TemporaryIdAllocator;

use super::DataStore;
use super::Handler;
use super::StoreMsg;

pub struct HandlerWrapper<Store, OtherStore, Allocator, OtherAllocator>
where
    OtherStore: DataStore<OtherAllocator> + ?Sized,
    Store: DataStore<Allocator, Record=<OtherStore as DataStore<OtherAllocator>>::Record> + ?Sized,
    OtherAllocator: TemporaryIdAllocator,
    Allocator: TemporaryIdAllocator<Type=OtherAllocator::Type>,
{
    parent: Box<dyn Handler<Store, Allocator>>,
    _other_store: PhantomData<*const OtherStore>,
    _other_allocator: PhantomData<*const OtherAllocator>,
}

impl<Store, OtherStore, Allocator, OtherAllocator> HandlerWrapper<Store, OtherStore, Allocator, OtherAllocator>
where
    OtherStore: DataStore<OtherAllocator> + ?Sized + 'static,
    Store: DataStore<Allocator, Record=<OtherStore as DataStore<OtherAllocator>>::Record> + ?Sized + 'static,
    OtherAllocator: TemporaryIdAllocator + 'static,
    Allocator: TemporaryIdAllocator<Type=OtherAllocator::Type> + 'static,
{
    pub fn from(parent: Box<dyn Handler<Store, Allocator>>) -> Box<dyn Handler<OtherStore, OtherAllocator>> {
        Box::new(
            HandlerWrapper{
                parent,
                _other_store: PhantomData,
                _other_allocator: PhantomData,
            }
        )
    }
}

impl<Store, OtherStore, Allocator, OtherAllocator> Handler<OtherStore, OtherAllocator> for HandlerWrapper<Store, OtherStore, Allocator, OtherAllocator>
where
    OtherStore: DataStore<OtherAllocator> + ?Sized,
    Store: DataStore<Allocator, Record=<OtherStore as DataStore<OtherAllocator>>::Record> + ?Sized,
    OtherAllocator: TemporaryIdAllocator,
    Allocator: TemporaryIdAllocator<Type=OtherAllocator::Type>,
{
    fn handle(&self, message: StoreMsg<<OtherStore as DataStore<OtherAllocator>>::Record>) -> bool {
        self.parent.handle(message)
    }
}