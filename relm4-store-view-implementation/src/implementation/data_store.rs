use reexport::relm4;

use relm4::Sender;

use record::Id;
use record::Identifiable;
use record::TemporaryIdAllocator;

use store::DataStore;
use store::FactoryConfiguration;
use store::Pagination;
use store::Position;
use store::RecordWithLocation;
use store::StoreId;
use store::StoreMsg;
use store::StoreView;

use store::math::Range;

use super::StoreViewImplementation;

impl<Configuration, Allocator, StoreIdAllocator> Identifiable<Self, StoreIdAllocator::Type> for StoreViewImplementation<Configuration, Allocator, StoreIdAllocator>
where
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Id = StoreId<Self, Allocator, StoreIdAllocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Configuration, Allocator, StoreIdAllocator> DataStore<Allocator, StoreIdAllocator> for StoreViewImplementation<Configuration, Allocator, StoreIdAllocator> 
where 
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Record = <Configuration::Store as DataStore<Allocator, StoreIdAllocator>>::Record;

    fn len(&self) -> usize {
        self.store.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }

    fn get_range(&self, range: &Range) -> Vec<<Configuration::Store as DataStore<Allocator, StoreIdAllocator>>::Record> {
        self.store.borrow().get_range(range)
    }

    fn get(&self, id: &Id<Self::Record, Allocator>) -> Option<Self::Record> {
        self.store.borrow().get(id)
    }

    fn listen(&self, id: StoreId<Self, Allocator, StoreIdAllocator>, sender: Sender<StoreMsg<Self::Record, Allocator>>) {
        self.handlers.borrow_mut().insert(id, sender);
    }

    fn unlisten(&self, handler_ref: StoreId<Self, Allocator, StoreIdAllocator>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }

    fn sender(&self) -> Sender<StoreMsg<Self::Record, Allocator>> {
        self.sender.clone()
    }

    fn send(&self, msg: StoreMsg<Self::Record, Allocator>) {
        self.sender.send(msg).expect("WTF? Since store view is here why it failed?");
    }
}

impl<Configuration, Allocator, StoreIdAllocator> StoreView<Allocator, StoreIdAllocator> for StoreViewImplementation<Configuration, Allocator, StoreIdAllocator> 
where
    Configuration: 'static + ?Sized + FactoryConfiguration<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator + 'static,
    StoreIdAllocator: TemporaryIdAllocator,
{
    type Configuration = Configuration;

    fn window_size(&self) -> usize {
        self.range.borrow().len()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<<Configuration::Store as DataStore<Allocator, StoreIdAllocator>>::Record, Allocator>> {
        let view = self.view.borrow();
        let mut result = Vec::with_capacity(view.len());

        let start = *self.range.borrow().start();

        for (idx, id) in view.ordered_record_ids().enumerate() {
            let pos = Position(idx+start);
            let record = view.get_record(id).unwrap().clone();
            result.push(RecordWithLocation::new(pos, record));
        }

        result
    }

    fn current_len(&self) -> usize {
        self.view.borrow().len()
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

    fn get_position(&self, id: &Id<Self::Record, Allocator>) -> Option<Position> {
        let view = self.view.borrow();
        for (pos, view_id) in view.record_ids().enumerate() {
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
