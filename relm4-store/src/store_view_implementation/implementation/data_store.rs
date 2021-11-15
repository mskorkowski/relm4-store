use reexport::relm4;

use relm4::Sender;

use record::Id;
use record::Identifiable;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::FactoryConfiguration;
use crate::Pagination;
use crate::Position;
use crate::RecordWithLocation;
use crate::StoreId;
use crate::StoreMsg;
use crate::StoreView;

use crate::math::Range;

use super::StoreViewImplementation;
impl<Configuration, Allocator> Identifiable<Self, Allocator::Type> for StoreViewImplementation<Configuration, Allocator>
where
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    type Id = StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Configuration, Allocator> DataStore<Allocator> for StoreViewImplementation<Configuration, Allocator> 
where 
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    type Record = <Configuration::Store as DataStore<Allocator>>::Record;

    fn len(&self) -> usize {
        self.store.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }

    fn get_range(&self, range: &Range) -> Vec<<Configuration::Store as DataStore<Allocator>>::Record> {
        self.store.borrow().get_range(range)
    }

    fn get(&self, id: &Id<Self::Record>) -> Option<Self::Record> {
        self.store.borrow().get(id)
    }

    fn listen(&self, id: StoreId<Self, Allocator>, sender: Sender<StoreMsg<Self::Record>>) {
        self.handlers.borrow_mut().insert(id, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record>> {
        self.sender.clone()
    }

    fn send(&self, msg: StoreMsg<Self::Record>) {
        self.sender.send(msg).expect("WTF? Since store view is here why it failed?");
    }
}

impl<Configuration, Allocator> StoreView<Allocator> for StoreViewImplementation<Configuration, Allocator> 
where
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    type Configuration = Configuration;

    fn window_size(&self) -> usize {
        self.range.borrow().len()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<<Configuration::Store as DataStore<Allocator>>::Record>> {
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