use record::TemporaryIdAllocator;
use reexport::relm4;

use std::cell::RefCell;
use std::rc::Weak;

use relm4::send;
use relm4::Sender;

use record::Id;
use record::Identifiable;

use crate::DataStore;
use crate::FactoryConfiguration;
use crate::Handler;
use crate::Pagination;
use crate::Position;
use crate::RecordWithLocation;
use crate::StoreId;
use crate::StoreMsg;
use crate::StoreView;

use crate::math::Range;
use crate::redraw_messages::RedrawMessages;

use super::StoreViewImplementation;
impl<Builder, Allocator> Identifiable<Self, Allocator::Type> for StoreViewImplementation<Builder, Allocator>
where
    Builder: FactoryConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    type Id = StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Builder, Allocator> DataStore<Allocator> for StoreViewImplementation<Builder, Allocator> 
where 
    Builder: FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    type Record = <Builder::Store as DataStore<Allocator>>::Record;

    fn len(&self) -> usize {
        self.store.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }

    fn inbox(&self, message: StoreMsg<Self::Record>) {
        self.changes.borrow_mut().push(message);
    }

    fn get_range(&self, range: &Range) -> Vec<<Builder::Store as DataStore<Allocator>>::Record> {
        self.store.borrow().get_range(range)
    }

    fn get(&self, id: &Id<Self::Record>) -> Option<Self::Record> {
        self.store.borrow().get(id)
    }

    fn listen(&self, handler_ref: StoreId<Self, Allocator>, handler: Box<dyn Handler<Self, Allocator>>) {
        self.handlers.borrow_mut().insert(handler_ref, handler);         
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }
}

impl<Builder, Allocator> StoreView<Allocator> for StoreViewImplementation<Builder, Allocator> 
where
    Builder: FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    type Builder = Builder;

    fn window_size(&self) -> usize {
        self.range.borrow().len()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<<Builder::Store as DataStore<Allocator>>::Record>> {
        let mut result = Vec::new();

        let data = self.get_range(&self.range.borrow());

        let mut i = *self.range.borrow().start();
        for record in data {
            //TODO: unsafe in case when view is out of sync with store
            let pos = Position(i);
            result.push(RecordWithLocation::new(pos, record));
            i += 1;
        }

        result
    }

    fn first_page(&self) {
        let range = {
            self.range.borrow().slide(0)
        };
        self.range.replace(range);
        self.inbox(StoreMsg::Reload);
    }

    fn prev_page(&self) {
        let range = {
            self.range.borrow().to_left(self.size)
        };
        self.range.replace(range);
        self.inbox(StoreMsg::Reload);
    }

    fn next_page(&self) {
        let range = {
            self.range.borrow().to_right(self.size)
        };

        if *range.start() < self.store.borrow().len() {
            self.range.replace(range);
            self.inbox(StoreMsg::Reload);
        }
    }

    fn last_page(&self) {
        let range = {
            let last_page = self.total_pages();
            let start = (last_page-1)*self.size;

            Range::new(start, start+self.size)
        };
        self.range.replace(range);
        self.inbox(StoreMsg::Reload);
    }

    fn get_window(&self) -> Range {
        self.range.borrow().clone()
    }

    fn get_position(&self, id: &Id<Self::Record>) -> Option<Position> {
        let view = self.view.borrow();
        for (pos, view_id) in view.iter().enumerate() {
            if view_id == id {
                return Some(Position(pos))
            }
        }

        None
    }

    fn set_window(&self, range: Range) {
        self.range.replace(range);
        self.inbox(StoreMsg::Reload);
    }

    fn inbox_queue_size(&self) -> usize {
        self.changes.borrow().len()
    }
}

pub struct StoreViewImplHandler<Builder, Allocator>
where
    Builder: FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    view: Weak<RefCell<StoreViewImplementation<Builder, Allocator>>>,
    sender: Sender<RedrawMessages>,
}

impl<Builder, Allocator> StoreViewImplHandler<Builder, Allocator>
where
    Builder: FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    pub fn new(view: Weak<RefCell<StoreViewImplementation<Builder, Allocator>>>, sender: Sender<RedrawMessages>) -> Self {
        Self {
            view,
            sender,
        }
    }
}

impl<Builder, Allocator> Handler<Builder::Store, Allocator> for StoreViewImplHandler<Builder, Allocator> 
where
    Builder: 'static + FactoryConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    fn handle(&self, message: StoreMsg<<Builder::Store as DataStore<Allocator>>::Record>) -> bool {
        if let Some(view) = self.view.upgrade() {
            view.borrow().inbox(message);
            let sender = &self.sender;
            send!(sender, RedrawMessages::Redraw);
            false
        }
        else {
            true
        }
    }
}