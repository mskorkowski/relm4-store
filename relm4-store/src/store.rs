//! Base store implementation
use record::DefaultIdAllocator;
use reexport::glib;
use reexport::relm4;
use reexport::log;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use relm4::Sender;

use record::Identifiable;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::OrderedBackend;
use crate::OrderedStore;
use crate::Replies;
use crate::StoreId;
use crate::StoreMsg;
use crate::StoreViewMsg;

/// Generic implementation of the DataStore
#[derive(Debug)]
pub struct Store<Backend, StoreIdAllocator=DefaultIdAllocator> 
where
    Backend: crate::Backend,
    StoreIdAllocator: TemporaryIdAllocator,
{
    id: StoreId<Self>,
    backend: Rc<RefCell<Backend>>,

    connections: Rc<RefCell<HashMap<StoreId<Self>, Sender<StoreViewMsg<Backend::Record>>>>>,
    sender: Sender<StoreMsg<Backend::Record>>,
}

impl<Backend, StoreIdAllocator> Store<Backend, StoreIdAllocator> 
where
    Backend: 'static + crate::Backend,
    StoreIdAllocator: 'static + TemporaryIdAllocator,
{
    /// Creates new instance of the Store
    pub fn new(backend: Backend) -> Self {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let id = StoreId::new();
        let shared_backed = Rc::new(RefCell::new(backend));
        let handler_backend = shared_backed.clone();

        let connections: Rc<RefCell<HashMap<StoreId<Self>, Sender<StoreViewMsg<Backend::Record>>>>> = Rc::new(RefCell::new(HashMap::new()));
        let handler_connections = connections.clone();

        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg:StoreMsg<Backend::Record>| {
                if let Ok(mut backend) = handler_backend.try_borrow_mut() {
                    let replies = backend.inbox(msg);
                    if let Ok(mut connections) = handler_connections.try_borrow_mut() { 
                        let mut to_remove = Vec::<StoreId<Store<Backend, StoreIdAllocator>>>::new();
                        for (sid,c) in connections.iter() {
                            for msg in &replies.replies {
                                if let Err(..) = c.send(msg.clone()) {
                                    // in case of broken channel (closed by other side), mark it for removal
                                    to_remove.push(sid.clone());
                                    break;
                                }
                            }
                        }
                        
                        for sid in to_remove {
                            connections.remove(&sid);
                        }
                    }
                    else {
                        log::warn!("Can't borrow connections. Remember to release leases");
                    }
                }
                else {
                    log::warn!("Can't borrow backend. Remember to release the leases");
                }
                glib::Continue(true)
            });
        }

        Store {
            id,
            backend: shared_backed,
            sender,
            connections,
        }
    }

    /// Allows to send message to all views attached to the store
    /// 
    /// Store is unable to check if your message would break the state of the store views. When you use this method
    /// please double check if you are not breaking something.
    pub fn fire_handlers(&self, messages: &Vec<StoreViewMsg<Backend::Record>>) {
        if let Ok(mut connections) = self.connections.try_borrow_mut() { 
            let mut to_remove = Vec::<StoreId<Store<Backend, StoreIdAllocator>>>::new();
            for (sid,c) in connections.iter() {
                for msg in messages {
                    if let Err(..) = c.send(msg.clone()) {
                        // in case of broken channel (closed by other side), mark it for removal
                        to_remove.push(sid.clone());
                        break;
                    }
                }
            }
            
            for sid in to_remove {
                connections.remove(&sid);
            }
        }
        else {
            log::warn!("Can't borrow connections. Remember to release leases");
        }
    }

    /// Returns shared reference to backend
    /// 
    /// You **must** make sure you return all the leases. Avoid this method as much as you can
    pub fn backend(&self) -> Rc<RefCell<Backend>> {
        self.backend.clone()
    }
}

impl<Backend, StoreIdAllocator> Identifiable<Store<Backend, StoreIdAllocator>, StoreIdAllocator::Type> for Store<Backend, StoreIdAllocator>
where
    Backend: crate::Backend,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Id=StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl<Backend, StoreIdAllocator> DataStore for Store<Backend, StoreIdAllocator> 
where
    Backend: crate::Backend,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Allocator = StoreIdAllocator;
    type Record = Backend::Record;
    type Messages = StoreMsg<Self::Record>;

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

    fn listen(&self, id: StoreId<Self>, sender: reexport::relm4::Sender<StoreViewMsg<Self::Record>>) {
        self.connections.borrow_mut().insert(id, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self>) {
        self.connections.borrow_mut().remove(&handler_ref);
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record>> {
        self.sender.clone()
    }

    fn send(&self, msg: crate::StoreMsg<Self::Record>) {
        // this shouldn't fail since receiver should still be there
        self.sender.send(msg).unwrap();
    }
}

impl<Backend, OrderBy, StoreIdAllocator> OrderedStore<OrderBy> for Store<Backend, StoreIdAllocator> 
where
    Backend: 'static + crate::Backend + OrderedBackend<OrderBy>,
    StoreIdAllocator: 'static + TemporaryIdAllocator,
{
    fn set_order(&self, order: OrderBy) {
        let be: &RefCell<Backend> = self.backend.borrow();
        let Replies{ replies } = be.borrow_mut().set_order(order);
        self.fire_handlers(&replies);
    }
}

impl<Backend, StoreIdAllocator> Clone for Store<Backend, StoreIdAllocator> 
where
    Backend: crate::Backend,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Implements shallow clone. Internally store is backed by `Rc<RefCell<Backend>>` so cloning doesn't
    /// detach from backend
    fn clone(&self) -> Self {
        Store{
            id: self.id.clone(),
            backend: self.backend.clone(),
            connections: self.connections.clone(),
            sender: self.sender.clone(),
        }
    }
}