//! Base store implementation

use std::borrow::Borrow;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use record::Identifiable;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::StoreId;

/// DataStore wrapper around all the rc/refcell stuff. Makes ownership bit easier
#[derive(Debug)]
pub struct Store<Backend, Allocator, StoreIdAllocator> 
where
    Backend: DataStore<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    backend: Rc<RefCell<Backend>>,
    _allocator: PhantomData<*const Allocator>,
    _store_id_allocator: PhantomData<*const StoreIdAllocator>,
}

impl<Backend, Allocator, StoreIdAllocator> Store<Backend, Allocator, StoreIdAllocator> 
where
    Backend: DataStore<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Creates new instance of the Store
    pub fn new(backend: Rc<RefCell<Backend>>) -> Self {
        Store {
            backend,
            _allocator: PhantomData,
            _store_id_allocator: PhantomData,
        }
    }
}

impl<Backend, Allocator, StoreIdAllocator> Identifiable<Store<Backend, Allocator, StoreIdAllocator>, StoreIdAllocator::Type> for Store<Backend, Allocator, StoreIdAllocator>
where 
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
    Backend: DataStore<Allocator, StoreIdAllocator>
{
    type Id=StoreId<Self, Allocator, StoreIdAllocator>;

    fn get_id(&self) -> Self::Id {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get_id().transfer()
    }
}

impl<Backend, Allocator, StoreIdAllocator> DataStore<Allocator, StoreIdAllocator> for Store<Backend, Allocator, StoreIdAllocator> 
where
    Backend: DataStore<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Record = Backend::Record;

    fn len(&self) -> usize {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().len()
    }

    fn is_empty(&self) -> bool {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().is_empty()
    }

    fn get(&self, id: &record::Id<Self::Record, Allocator>) -> Option<Self::Record> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get(id)
    }

    fn get_range(&self, range: &crate::math::Range) -> Vec<Self::Record> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get_range(range)
    }

    fn listen(&self, id: StoreId<Self, Allocator, StoreIdAllocator>, sender: reexport::relm4::Sender<crate::StoreMsg<Self::Record, Allocator>>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().listen(id.transfer(), sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator, StoreIdAllocator>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().unlisten(handler_ref.transfer());
    }

    fn sender(&self) -> reexport::relm4::Sender<crate::StoreMsg<Self::Record, Allocator>> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().sender()
    }

    fn send(&self, msg: crate::StoreMsg<Self::Record, Allocator>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().send(msg)
    }
}
