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
use crate::FactoryContainerWidgets;
use crate::Handler;
use crate::Pagination;
use crate::Position;
use crate::RecordWithLocation;
use crate::StoreId;
use crate::StoreMsg;
use crate::StoreView;

use crate::factory_configuration::StoreViewInnerComponent;
use crate::math::Range;
use crate::redraw_messages::RedrawMessages;

use super::StoreViewImplementation;
impl<Widgets, Configuration, Components, Allocator> Identifiable<Self, Allocator::Type> for StoreViewImplementation<Widgets, Configuration, Components, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Components, Allocator>,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{
    type Id = StoreId<Self, Allocator>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl<Widgets, Configuration, Components, Allocator> DataStore<Allocator> for StoreViewImplementation<Widgets, Configuration, Components, Allocator> 
where 
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Components, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{
    type Record = <Configuration::Store as DataStore<Allocator>>::Record;

    fn len(&self) -> usize {
        self.store.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.store.borrow().is_empty()
    }

    fn inbox(&self, message: StoreMsg<Self::Record>) {
        self.changes.borrow_mut().push(message);
        self.redraw_sender.send(RedrawMessages::Redraw).expect("Unexpected failure while sending message via redraw_sender");
    }

    fn get_range(&self, range: &Range) -> Vec<<Configuration::Store as DataStore<Allocator>>::Record> {
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

impl<Widgets, Configuration, Components, Allocator> StoreView<Widgets, Components, Allocator> for StoreViewImplementation<Widgets, Configuration, Components, Allocator> 
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Components, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
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

/// Handler which is used by the [StoreViewImplementation] to bind to the underlying data stores
pub struct StoreViewImplHandler<Widgets, Configuration, Components, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Components, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{
    view: Weak<RefCell<StoreViewImplementation<Widgets, Configuration, Components, Allocator>>>,
    sender: Sender<RedrawMessages>,
}

impl<Widgets, Configuration, Components, Allocator> StoreViewImplHandler<Widgets, Configuration, Components, Allocator>
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: FactoryConfiguration<Widgets, Components, Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{

    /// Creates new instance of this handler
    pub fn new(view: Weak<RefCell<StoreViewImplementation<Widgets, Configuration, Components, Allocator>>>, sender: Sender<RedrawMessages>) -> Self {
        Self {
            view,
            sender,
        }
    }
}

impl<Widgets, Configuration, Components, Allocator> Handler<Configuration::Store, Allocator> for StoreViewImplHandler<Widgets, Configuration, Components, Allocator> 
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: 'static + FactoryConfiguration<Widgets, Components, Allocator>,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{
    fn handle(&self, message: StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>) -> bool {
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

impl<Widgets, Configuration, Components, Allocator> std::fmt::Debug for StoreViewImplHandler<Widgets, Configuration, Components, Allocator> 
where
    Widgets: ?Sized + FactoryContainerWidgets<Configuration, Components, Allocator>,
    Configuration: 'static + FactoryConfiguration<Widgets, Components, Allocator>,
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<Configuration>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoreViewImplHandler").finish_non_exhaustive()
    }
}