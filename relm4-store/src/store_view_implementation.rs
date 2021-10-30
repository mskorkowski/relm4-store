use reexport::relm4;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::rc::Weak;

use relm4::Sender;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use model::Identifiable;
use model::Id;

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
use super::WindowChangeset;

use super::math::Point;
use super::math::Range;
use super::math::Window as DataWindow;
use super::math::WindowTransition;
use super::widgets::Widgets;

/// View of the store
/// 
/// State of view reflects subset of the state of store. Like a page of the data.
/// You can ask the view for data. But there is no way to interact with
/// content directly in any meaningful way and that's by design.
/// 
/// To interact with content you should use Store. Store will handle all the
/// make sure all the updates are propagated to the view.
pub struct StoreViewImpl<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    id: StoreId<Self>,
    store: Rc<RefCell<Builder::Store>>,
    handlers: RefCell<HashMap<StoreId<Self>, Box<dyn Handler<Self>>>>,
    #[allow(clippy::type_complexity)]
    view_data: RefCell<HashMap<Id<<Builder::Store as DataStoreBase>::Model>, <Builder::Store as DataStoreBase>::Model>>,
    view: RefCell<VecDeque<Id<<Builder::Store as DataStoreBase>::Model>>>,
    #[allow(clippy::type_complexity)]
    widgets: RefCell<HashMap<Id<<Builder::Store as DataStoreBase>::Model>, Widgets<Builder::Widgets, <Builder::View as FactoryView<Builder::Root>>::Root>>>,
    changes: RefCell<Vec<StoreMsg<<Builder::Store as DataStoreBase>::Model>>>,
    range: RefCell<Range>,
    size: usize,
}

impl<'me, Builder> StoreViewImpl<Builder> 
where
    Builder: FactoryBuilder + 'static,
{
    pub fn new(store: Rc<RefCell<Builder::Store>>, size: usize) -> Self {
        let range = RefCell::new(Range::new(0, size));

        let view_data = RefCell::new(HashMap::new());
        let view = RefCell::new(VecDeque::new());
        let changes = RefCell::new(Vec::new());
        changes.borrow_mut().push(StoreMsg::Reload);

        Self{
            id: StoreId::new(),
            store,
            handlers: RefCell::new(HashMap::new()),
            view_data,
            view,    
            widgets: RefCell::new(HashMap::new()),
            changes,
            range,
            size,
        }
    }

    fn convert_to_transition(&self, range: &Range, message: &StoreMsg<<Builder::Store as DataStoreBase>::Model>) -> WindowTransition {
        match message {
            StoreMsg::New(_record) => {
                println!("[{:?}] convert new message", self.id);
                Builder::Window::insert(range, &Point::new(self.view_data.borrow().len()))
            },
            StoreMsg::NewAt(p) => {
                println!("[{:?}] convert new at message", self.id);
                Builder::Window::insert(range, &p.to_point())
            },
            StoreMsg::Move{from, to} => {
                println!("[{:?}] convert movr message", self.id);
                Builder::Window::slide(range, &Range::new(from.0, to.0))
            },
            StoreMsg::Reorder{from, to} => {
                println!("[{:?}] convert reorder message", self.id);
                Builder::Window::slide(range, &Range::new(from.0, to.0))
            },
            StoreMsg::Remove(at) => {
                println!("[{:?}] convert remove message", self.id);
                Builder::Window::remove(range, &at.to_point())
            },
            StoreMsg::Commit(_) => {
                println!("[{:?}] convert commit message", self.id);
                WindowTransition::Identity
            },
            StoreMsg::Update(_) => {
                println!("[{:?}] convert update message", self.id);
                WindowTransition::Identity
            },
            StoreMsg::Reload => {
                println!("[{:?}] convert reload message", self.id);
                WindowTransition::InsertRight{pos: range.start(), by: range.len()}
            },
        }
    }

    fn compile_changes(&self) -> WindowChangeset<Builder> {
        println!("[{:?}] compile changes", self.id);
        let mut widgets_to_remove = HashSet::new();
        let mut ids_to_add = HashSet::new();
        let mut ids_to_update = HashSet::new();
        let mut changes = self.changes.borrow_mut();

        for change in changes.iter() {
            let transition = self.convert_to_transition(&self.range.borrow(), change);

            if let StoreMsg::Update(id) = change {
                {
                    let store = self.store.borrow();
                    let mut view_data = self.view_data.borrow_mut();
                    
                    if view_data.get(id).is_some() {
                        if let Some(record) = store.get(id) {
                            ids_to_update.insert(*id);
                            view_data.insert(*id, record.clone());
                        }
                    }
                }
            }

            match transition {
                WindowTransition::Identity => (),
                WindowTransition::InsertLeft{pos: _, by: _} => {

                }
                WindowTransition::InsertRight{pos, by} => {
                    let store = self.store.borrow();
                    let end = self.range.borrow().end();
                    let start = self.range.borrow().start();
                    let range_of_changes = Range::new(pos, end);
                    let mut new_items: Vec<<Self as DataStoreBase>::Model> = store.get_range(&range_of_changes);
                    // new_items.sort();

                    let mut view = self.view.borrow_mut();

                    //remove unused data
                    for id in view.range(pos-start..) {
                        self.view_data.borrow_mut().remove(id);
                        widgets_to_remove.insert(*id);
                    }
                    view.truncate(pos-start);

                    //remove unneeded data from view
                    let mut len = 0;
                    let new_items_len = new_items.len();
                    while !store.is_empty() && len <  new_items_len && len < by {
                        let record = new_items.get(len).unwrap();
                        view.remove(pos+len);

                        view.insert(pos+len-start, record.get_id());
                        self.view_data.borrow_mut().insert(record.get_id(), record.clone());
                        ids_to_add.insert(record.get_id());
                        len += 1;
                    }
                }
                WindowTransition::RemoveLeft{pos: _, by: _} => {

                }
                WindowTransition::RemoveRight{pos: _, by: _} => {

                }
                WindowTransition::SlideLeft(_by) => {

                }
                WindowTransition::SlideRight(_by) => {

                }
            }
        }

        changes.clear();

        for id in &ids_to_update {
            ids_to_add.remove(id);
        }

        WindowChangeset{
            widgets_to_remove,
            ids_to_add,
            ids_to_update,
        }
    }

    pub fn generate(&self, view: &Builder::View, sender: Sender<Builder::Msg>) {
        println!("[{:?}] generate", self.id);
        let WindowChangeset{
            widgets_to_remove,
            ids_to_add,
            ids_to_update,
        } = self.compile_changes();


        let mut widgets = self.widgets.borrow_mut();
        let view_order = self.view.borrow();

        for id in widgets_to_remove {
            if let Some(widget) = widgets.remove(&id) {
                view.remove(&widget.root);
            }
        }

        let mut position: Position = Position(self.range.borrow().start());
        for id in view_order.iter() {
            if ids_to_add.contains(id) {
                if let Some(record) = self.get(id) {
                    let new_widgets = Builder::generate(&record, position, sender.clone());
                    let root = Builder::get_root(&new_widgets);
                    let root = if widgets.is_empty() || position.get() == 0 {
                        view.push_front(root)
                    }
                    else {
                        let range = self.range.borrow();
                        let prev_id = view_order[(position - 1 - range.start()).get()];
                        let prev = widgets.get(&prev_id).unwrap();
                        view.insert_after(root, &prev.root)
                    };
    
                    widgets.insert(
                        *id,
                        Widgets{
                            widgets: new_widgets,
                            root,
                        }
                    );
                }
            }

            if ids_to_update.contains(id) {
                if let Some(record) = self.get(id) {
                    if let Some( widget ) = widgets.get_mut(id) {
                        Builder::update(record, position, &widget.widgets);
                    }
                }
            }


            position = position + 1;
        }
    }
}

impl<Builder> IdentifiableStore for StoreViewImpl<Builder> 
where
    Builder: FactoryBuilder + 'static,
{}

impl<Builder> Identifiable for StoreViewImpl<Builder>
where
    Builder: FactoryBuilder,
{
    type Id = StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}


impl<Builder> DataStoreBase for StoreViewImpl<Builder> 
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

impl<Builder: FactoryBuilder> DataStoreListenable for StoreViewImpl<Builder> {
    fn listen(&self, handler_ref: StoreId<Self>, handler: Box<dyn Handler<Self>>) {
        self.handlers.borrow_mut().insert(handler_ref, handler);         
    }

    fn unlisten(&self, handler_ref: StoreId<Self>) {
        self.handlers.borrow_mut().remove(&handler_ref);
    }
}

impl<Builder> DataStore for StoreViewImpl<Builder> 
where 
    Builder: FactoryBuilder + 'static,
{}

impl<Builder> StoreView for StoreViewImpl<Builder> 
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

        let mut i = self.range.borrow().start();
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

        if range.start() < self.store.borrow().len() {
            self.range.replace(range);
            self.inbox(StoreMsg::Reload);
        }
    }

    fn last_page(&self) {
        let range = {
            let last_page = self.total_pages();
            let start = (last_page-1)*self.size;

            self.range.borrow().slide(start)
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
}


pub struct StoreViewImplHandler<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    view: Weak<RefCell<StoreViewImpl<Builder>>>
}

impl<Builder> StoreViewImplHandler<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    pub fn new(view: Weak<RefCell<StoreViewImpl<Builder>>>) -> Self {
        Self {
            view,
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
            false
        }
        else {
            true
        }
    }
}