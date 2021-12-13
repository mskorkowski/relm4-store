//! Base store implementation

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use record::Identifiable;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::StoreId;

/// DataStore wrapper around all the rc/refcell stuff. Makes ownership bit easier
#[derive(Debug)]
pub struct Store<Backend> 
where
    Backend: DataStore,
{
    backend: Rc<RefCell<Backend>>,
}

impl<Backend> Store<Backend> 
where
    Backend: DataStore,
{
    /// Creates new instance of the Store
    pub fn new(backend: Rc<RefCell<Backend>>) -> Self {
        Store {
            backend,
        }
    }
}

impl<Backend> Identifiable<Store<Backend>, <Backend::Allocator as TemporaryIdAllocator>::Type> for Store<Backend>
where
    Backend: DataStore
{
    type Id=StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get_id().transfer()
    }
}

impl<Backend> DataStore for Store<Backend> 
where
    Backend: DataStore,
{
    type Allocator = Backend::Allocator;
    type Record = Backend::Record;

    fn len(&self) -> usize {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().len()
    }

    fn is_empty(&self) -> bool {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().is_empty()
    }

    fn get(&self, id: &record::Id<Self::Record>) -> Option<Self::Record> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get(id)
    }

    fn get_range(&self, range: &crate::math::Range) -> Vec<Self::Record> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().get_range(range)
    }

    fn listen(&self, id: StoreId<Self>, sender: reexport::relm4::Sender<crate::StoreMsg<Self::Record>>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().listen(id.transfer(), sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().unlisten(handler_ref.transfer());
    }

    fn sender(&self) -> reexport::relm4::Sender<crate::StoreMsg<Self::Record>> {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().sender()
    }

    fn send(&self, msg: crate::StoreMsg<Self::Record>) {
        let be: &RefCell<Backend> = self.backend.borrow();
        be.borrow().send(msg)
    }
}
