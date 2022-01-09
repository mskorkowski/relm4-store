use record::Id;

use store::DataStore;
use store::StoreViewPrototype;
use store::Position;
use store::RecordWithLocation;

use store::math::Range;

use super::StoreViewImplementation;

/// DataStore methods implementation
impl<Configuration> StoreViewImplementation<Configuration> 
where 
    Configuration: 'static + ?Sized + StoreViewPrototype,
{
    /// [store::DataStore::len()]
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// [store::DataStore::is_empty()]
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// [store::DataStore::get_range()]
    pub fn get_range(&self, range: &Range) -> Vec<<Configuration::Store as DataStore>::Record> {
        self.store.get_range(range)
    }

    /// [store::DataStore::get()]
    pub fn get(&self, id: &Id<<Configuration::Store as DataStore>::Record>) -> Option<<Configuration::Store as DataStore>::Record> {
        self.store.get(id)
    }
}

/// StoreView methods implementation
impl<Configuration> StoreViewImplementation<Configuration> 
where
    Configuration: 'static + ?Sized + StoreViewPrototype,
{
    /// [store::StoreView::window_size()]
    pub fn window_size(&self) -> usize {
        self.range.borrow().len()
    }

    /// [store::StoreView::get_view_data()]
    pub fn get_view_data(&self) -> Vec<RecordWithLocation<<Configuration::Store as DataStore>::Record>> {
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

    /// [store::StoreView::current_len()]
    pub fn current_len(&self) -> usize {
        self.view.borrow().len()
    }

    /// [store::StoreView::get_window()]
    pub fn get_window(&self) -> Range {
        *self.range.borrow()
    }

    /// [store::StoreView::get_position()]
    pub fn get_position(&self, id: &Id<<Configuration::Store as DataStore>::Record>) -> Option<Position> {
        let view = self.view.borrow();
        for (pos, view_id) in view.record_ids().enumerate() {
            if view_id == id {
                return Some(Position(pos))
            }
        }

        None
    }

    /// [store::StoreView::set_window()]
    pub fn set_window(&self, range: Range) {
        self.range.replace(range);
    }

    /// [store::StoreView::inbox_queue_size()]
    pub fn inbox_queue_size(&self) -> usize {
        self.changes.borrow().len()
    }
}
