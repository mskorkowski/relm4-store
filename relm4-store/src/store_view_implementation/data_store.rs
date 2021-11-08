use reexport::relm4;

use std::cell::RefCell;
use std::rc::Weak;

use relm4::send;
use relm4::Sender;

use model::Identifiable;

use super::DataStore;
use super::DataStoreBase;
use super::DataStoreListenable;
use super::FactoryBuilder;
use super::Handler;
use super::IdentifiableStore;
use super::Pagination;
use super::Position;
use super::RecordWithLocation;
use super::StoreId;
use super::StoreMsg;
use super::StoreView;
use super::StoreViewImplementation;

use crate::math::Range;
use crate::redraw_messages::RedrawMessages;

impl<Builder> IdentifiableStore for StoreViewImplementation<Builder> 
where
    Builder: FactoryBuilder + 'static,
{}

impl<Builder> Identifiable for StoreViewImplementation<Builder>
where
    Builder: FactoryBuilder,
{
    type Id = StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}


impl<Builder> DataStoreBase for StoreViewImplementation<Builder> 
where
    Builder: FactoryBuilder + 'static
{
    type Model = <Builder::Store as DataStoreBase>::Model;

    fn len(&self) -> usize {
        self.store.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }

    fn inbox(&self, message: StoreMsg<Self::Model>) {
        self.changes.borrow_mut().push(message);
    }

    fn get_range(&self, range: &Range) -> Vec<<Builder::Store as DataStoreBase>::Model> {
        self.store.borrow().get_range(range)
    }

    fn get(&self, id: &<Self::Model as Identifiable>::Id) -> Option<Self::Model> {
        self.store.borrow().get(id)
    }
}

impl<Builder: FactoryBuilder> DataStoreListenable for StoreViewImplementation<Builder> {
    fn listen(&self, handler_ref: StoreId<Self>, handler: Box<dyn Handler<Self>>) {
        self.handlers.borrow_mut().insert(handler_ref, handler);         
    }

    fn unlisten(&self, handler_ref: StoreId<Self>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }
}

impl<Builder> DataStore for StoreViewImplementation<Builder> 
where 
    Builder: FactoryBuilder + 'static,
{}

impl<Builder> StoreView for StoreViewImplementation<Builder> 
where
    Builder: FactoryBuilder + 'static,
{
    type Builder = Builder;

    fn window_size(&self) -> usize {
        self.range.borrow().len()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<<Builder::Store as DataStoreBase>::Model>> {
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

    fn get_position(&self, id: &<<Builder::Store as DataStoreBase>::Model as Identifiable>::Id) -> Option<Position> {
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

pub struct StoreViewImplHandler<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    view: Weak<RefCell<StoreViewImplementation<Builder>>>,
    sender: Sender<RedrawMessages>,
}

impl<Builder> StoreViewImplHandler<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    pub fn new(view: Weak<RefCell<StoreViewImplementation<Builder>>>, sender: Sender<RedrawMessages>) -> Self {
        Self {
            view,
            sender,
        }
    }
}

impl<Builder> Handler<Builder::Store> for StoreViewImplHandler<Builder> 
where
    Builder: 'static + FactoryBuilder
{
    fn handle(&self, message: StoreMsg<<Builder::Store as DataStoreBase>::Model>) -> bool {
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