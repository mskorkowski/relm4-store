mod data_store;
mod factory;

use reexport::gtk;
use reexport::log;
use reexport::relm4;


use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use gtk::glib;

use relm4::Model as ViewModel;
use relm4::Sender;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use record::DefaultIdAllocator;
use record::Id;
use record::Record;
use record::TemporaryIdAllocator;

use store::DataStore;
use store::FactoryConfiguration;
use store::Position;
use store::StoreId;
use store::StoreMsg;
use store::math::Range;
use store::redraw_messages::RedrawMessages;
use store::window::StoreState;
use store::window::WindowBehavior;
use store::window::WindowTransition;

use super::window_changeset::WindowChangeset;
use super::widgets;

/// View of the store
/// 
/// State of view reflects subset of the state of store. Like a page of the data.
/// You can ask the view for data. But there is no way to interact with
/// content directly in any meaningful way and that's by design.
/// 
/// To interact with content you should use Store. Store will handle all the
/// make sure all the updates are propagated to the view.
pub struct StoreViewImplementation<Configuration, Allocator=DefaultIdAllocator>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    id: StoreId<Self, Allocator>,
    store: Rc<RefCell<Configuration::Store>>,
    handlers: Rc<RefCell<HashMap<StoreId<Self, Allocator>, Sender<StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>>>>>,
    #[allow(clippy::type_complexity)]
    view_data: Rc<RefCell<HashMap<Id<<Configuration::Store as DataStore<Allocator>>::Record>, <Configuration::Store as DataStore<Allocator>>::Record>>>,
    view: Rc<RefCell<Vec<Id<<Configuration::Store as DataStore<Allocator>>::Record>>>>,
    #[allow(clippy::type_complexity)]
    widgets: Rc<RefCell<HashMap<Id<<Configuration::Store as DataStore<Allocator>>::Record>, widgets::Widgets<Configuration::RecordWidgets, <Configuration::View as FactoryView<Configuration::Root>>::Root>>>>,
    changes: Rc<RefCell<Vec<StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>>>>,
    range: Rc<RefCell<Range>>,
    size: usize,
    redraw_sender: Sender<RedrawMessages>,
    sender: Sender<StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>>,
}

impl<Configuration, Allocator> Debug for StoreViewImplementation<Configuration, Allocator>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoreViewImplementation")
            .field("id", &self.id)
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

impl<Configuration, Allocator> StoreViewImplementation<Configuration, Allocator> 
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator,
{
    ///Creates  new instance of this struct
    /// 
    /// - **store** store which will provide a source data
    /// - **size** size of the page
    pub fn new(store: Rc<RefCell<Configuration::Store>>, size: usize, redraw_sender: Sender<RedrawMessages>) -> Self {
        let range = Rc::new(RefCell::new(Range::new(0, size)));
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    
        let view_data = Rc::new(RefCell::new(HashMap::new()));
        let view = Rc::new(RefCell::new(Vec::new()));
        let changes = Rc::new(RefCell::new(Vec::new()));
        let handler_changes = changes.clone();
        let handler_redraw_sender = redraw_sender.clone();
        {
            let context = glib::MainContext::default();
            receiver.attach(Some(&context), move |msg| {
                log::info!("Received message in store view: {:?}", &msg);
                if let Ok(mut changes) = handler_changes.try_borrow_mut() {
                    changes.push(msg);
                    log::info!("Now changes has {} messages", changes.len());
                    handler_redraw_sender.send(RedrawMessages::Redraw).expect("Unexpected failure while sending message via redraw_sender");
                }
                else {
                    log::warn!("Unable to borrow mutably the changes. Please drop all the references to changes!");
                }

                glib::Continue(true)
            });
        }

        let id: StoreId<Self, Allocator> = StoreId::new();
        
        store.borrow().listen(id.transfer(), sender.clone());
        changes.borrow_mut().push(StoreMsg::Reload);

        Self{
            id,
            store,
            handlers: Rc::new(RefCell::new(HashMap::new())),
            view_data,
            view,    
            widgets: Rc::new(RefCell::new(HashMap::new())),
            changes,
            range,
            size,
            redraw_sender,
            sender,
        }
    }

    fn inbox(&self, message: StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>) {
        self.changes.borrow_mut().push(message);
        self.redraw_sender.send(RedrawMessages::Redraw).expect("Unexpected failure while sending message via redraw_sender");
    }

    fn convert_to_transition(&self, state: &StoreState<'_>, message: &StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>) -> WindowTransition {
        match message {
            StoreMsg::NewAt(p) => {
                Configuration::Window::insert(state, &p.to_point())
            },
            StoreMsg::Move{from, to} => {
                Configuration::Window::slide(state, &Range::new(from.0, to.0))
            },
            StoreMsg::Reorder{from, to} => {
                Configuration::Window::slide(state, &Range::new(from.0, to.0))
            },
            StoreMsg::Remove(at) => {
                Configuration::Window::remove(state, &at.to_point())
            },
            StoreMsg::Commit(_) => {
                WindowTransition::Identity
            },
            StoreMsg::Update(_) => {
                WindowTransition::Identity
            },
            StoreMsg::Reload => {
                WindowTransition::Identity
            },
        }
    }

    fn reload(&self, changeset: &mut WindowChangeset<Configuration, Allocator>) {
        let store = self.store.borrow();
        let range_of_changes = self.range.borrow().clone();
        let new_record: Vec<<Self as DataStore<Allocator>>::Record> = store.get_range(&range_of_changes);
        let mut view = self.view.borrow_mut();
        let mut view_data = self.view_data.borrow_mut();
        
        // borrowing widgets so this method can implement the total cleanup. If for whatever reason other methods are broken reload will bring
        // view into sane state
        let widgets = self.widgets.borrow();

        // old_data, old_widget_ids, old_view are used to 
        let mut old_data: HashSet<Id<<Configuration::Store as DataStore<Allocator>>::Record>> = HashSet::new();
        old_data.extend(view_data.keys());

        let mut old_widget_ids: HashSet<Id<<Configuration::Store as DataStore<Allocator>>::Record>> = HashSet::new();
        old_widget_ids.extend(widgets.keys());

        let mut old_view = HashSet::new(); 
        old_view.extend(view.clone());

        view.clear();
        for record in new_record {
            let id = record.get_id();

            view.push(id);
            old_widget_ids.remove(&id);
            old_data.remove(&id);
            if old_view.contains(&id) {
                //old view had this record
                old_view.remove(&id); //removes id's from old view. It will allow us to remove unneeded data from the view
                changeset.ids_to_update.insert(id);
            }
            else {
                view_data.insert(id, record);
                changeset.ids_to_add.insert(id);
            }
        }

        for id in old_view {           
            changeset.widgets_to_remove.insert(id);
        }

        for id in old_widget_ids {
            changeset.widgets_to_remove.insert(id);
        }

        for id in old_data {
            view_data.remove(&id);
        }
    }

    /// Inserts `by` elements at the position `pos`
    /// 
    /// Insert is limited by the page size. For example if the window starts at `10` and ends at `20`, and you insert
    /// `5` records at position `18` you will basically insert two elements 18 and 19.
    fn insert_right(&self, changeset: &mut WindowChangeset<Configuration, Allocator>, pos: usize, by: usize) {
        // it's responsibility of the WindowBehavior to make this math valid and truncate `pos` and `by` to the acceptable range
        // and WindowBehavior logic was called before we reached here so we can assume we are safe here

        let range = self.range.borrow();
        let mut view = self.view.borrow_mut();
        let mut view_data = self.view_data.borrow_mut();
        let store = self.store.borrow();

        let start = *range.start();
        let size = range.len();

        let start_idx = pos - start; // index of first element in the view which is changed
        let end_idx = start_idx + by;

        let range_of_changes = Range::new(pos, end_idx+start);
        let data = store.get_range(&range_of_changes);

        let view_len = view.len();

        // how many elements I need to remove?
        let for_removal: usize = if view_len + by <= size { 0 } else { view_len + by - size };
        if for_removal > 0 {
            for idx in view_len-for_removal..view_len {
                let id_to_remove = view[idx].clone();
                view_data.remove(&id_to_remove);
                changeset.widgets_to_remove.insert(id_to_remove);
            }

            // move all preserved id's to the right
            for idx in (start_idx..view_len-for_removal).rev() {
                view[idx+by] = view[idx];
                changeset.ids_to_update.insert(view[idx].clone());
            }
        }

        for idx in start_idx..end_idx {
            // add new data so we can later generate widgets for them
            let record = &data[idx - start_idx];
            changeset.ids_to_add.insert(record.get_id());
            view_data.insert(record.get_id(), record.clone());

            if view.len() < size {
                view.insert(idx, record.get_id());
            }
            else {
                view[idx] = record.get_id();
            }
        }
    }

    /// InsertRight is always the same, since you can always remove elements from model being on the
    /// right hand side of the position to which you insert. In most extreme case removed elements is
    /// just an empty set.
    ///
    /// On the other hand insert left has a few flavours. 
    /// First case:
    /// Inserting left while left side of visible range is `0` (so there is nothing to remove from the view) 
    /// or `range.start()-by < 0` and there is less data in the store then one page. This is equivalent to
    /// `InsertRight{pos, by}`
    /// 
    /// Second case:
    /// Inserting data while left side of visible range is `0` or `range.start() - by < 0` and there is more
    /// then single page of data. It's equivalent of `InsertRight{pos, by}`
    ///
    /// Third case:
    /// 
    fn insert_left(&self, changeset: &mut WindowChangeset<Configuration, Allocator>, pos: usize, by: usize) {

        log::trace!("Insert left");
        let (start, size) = {
            let range = self.range.borrow();
            (*range.start(), range.len())
        };
        let view_len = self.view.borrow().len();
        
        if start == 0 && view_len < size {
            log::trace!("\t\t Doing insert right");
            self.insert_right(changeset, pos, by)                        
        }
        else {
            // it's responsibility of the WindowBehavior to make this math valid and truncate `pos` and `by` to the acceptable range
            // and WindowBehavior logic was called before we reached here so we can assume we are safe here

            let mut view = self.view.borrow_mut();
            let mut view_data = self.view_data.borrow_mut();
            let store = self.store.borrow();

            let start_idx = pos - start; // index of first element in the view which is changed
            let end_idx = start_idx + by;

            let range_of_changes = Range::new(pos, end_idx+start);
            let data = store.get_range(&range_of_changes);

            // how many elements I need to remove?
            let for_removal: usize = if view_len + by <= size { 0 } else { view_len + by - size };
            if for_removal > 0 {
                for idx in 0..for_removal {
                    let id_to_remove = view[idx].clone();
                    view_data.remove(&id_to_remove);
                    changeset.widgets_to_remove.insert(id_to_remove);
                }

                // move all preserved id's to the left
                for idx in (for_removal..start_idx).rev() {
                    view[idx-by] = view[idx];
                    changeset.ids_to_update.insert(view[idx].clone());
                }
            }

            for idx in start_idx..end_idx {
                // add new data so we can later generate widgets for them
                let record = &data[idx - start_idx];
                changeset.ids_to_add.insert(record.get_id());
                view_data.insert(record.get_id(), record.clone());

                if view.len() < size {
                    view.insert(idx, record.get_id());
                }
                else {
                    view[idx] = record.get_id();
                }
            }

            let new_range = {
                let range = self.range.borrow();
                range.to_left(by)
            };
            self.range.replace(new_range);
        }                    
    }

    fn compile_changes(&self) -> WindowChangeset<Configuration, Allocator> {
        let mut changeset = WindowChangeset {
            widgets_to_remove: HashSet::new(),
            ids_to_add: HashSet::new(),
            ids_to_update: HashSet::new(),
            reload: false,
        };

        let mut changes = self.changes.borrow_mut();

        for change in changes.iter() {
            let transition = {
                let state = StoreState{
                    page: {
                        &self.range.borrow()
                    },
                    view: {
                        self.view.borrow().len()
                    },
                };
                self.convert_to_transition(&state, change)
            };

            match transition {
                WindowTransition::Identity => {
                    log::warn!("Identity");
                    match change {
                        StoreMsg::Update(id) => {
                            let store = self.store.borrow();
                            let mut view_data = self.view_data.borrow_mut();
                            
                            if view_data.get(id).is_some() {
                                if let Some(record) = store.get(id) {
                                    changeset.ids_to_update.insert(*id);
                                    view_data.insert(*id, record.clone());
                                }
                            }
                        },
                        StoreMsg::Reload => {
                            changeset.reload = true;
                            self.reload(&mut changeset);
                        },
                        _ => {}
                    }
                },
                WindowTransition::InsertLeft{pos, by } => {
                    log::warn!("InsertLeft");
                    self.insert_left(&mut changeset, pos, by);
                }
                WindowTransition::InsertRight{pos, by} => {
                    log::warn!("InsertRight");
                    self.insert_right(&mut changeset, pos, by);
                }
                WindowTransition::RemoveLeft{pos: _, by: _} => {
                    log::warn!("RemoveLeft");
                }
                WindowTransition::RemoveRight{pos: _, by: _} => {
                    log::warn!("RemoveRight");
                }
                WindowTransition::SlideLeft(_by) => {
                    log::warn!("SlideLeft");
                }
                WindowTransition::SlideRight(by) => {
                    //exceeds is true if we try to slide outside of available data
                    let exceeds = {
                        let range = self.range.borrow();
                        self.len() >= range.end() + by
                    };

                    if by > self.size || exceeds {
                        // Two cases solved here
                        // 1. Sliding more then page size, so we must reload whole range
                        // 2. Trying to go outside of data range of the store
                        //   2.1 There is less data in the store then one page so keep being on the first page
                        //   2.2 In all other cases stay on the last page
                        //
                        // TODO: Check in case 2 if we can reuse the data which already are in the store
                        //   currently we don't check for data overlap, just force a full reload which might
                        //   be more expensive then needed
                        let new_range = {
                            let range = self.range.borrow();
                            let new_end = range.end() + by;
                            if new_end > self.len() {
                                // Case 2
                                if self.size > self.len() {
                                    // Case 2.2
                                    range.slide(self.len()-self.size)
                                }
                                else {
                                    // Case 2.1
                                    range.slide(0)
                                }
                            }
                            else {
                                // Case 1
                                range.slide(range.start()+by)
                            }
                        };

                        self.range.replace(new_range);
                        self.reload(&mut changeset);
                    }
                    else {
                        let new_range = {
                            let range = self.range.borrow();
                            range.slide(range.start() + by)
                        };
                        self.range.replace(new_range);
                        self.reload(&mut changeset);
                    }
                }
            }

            let (page_len, data_len, view_len) = {
                let view = self.view.borrow();
                let view_data = self.view_data.borrow();
                let range = self.range.borrow();

                (range.len(), view_data.len(), view.len())
            };

            assert!(data_len == view_len, "Length of the view and length of the data must be equal");
            assert!(page_len >= data_len, "We can't have more data then page size");
        }

        changes.clear();

        for id in &changeset.ids_to_update {
            changeset.ids_to_add.remove(id);
        }

        changeset
    }

    /// Implementation of the [relm4::factory::FactoryPrototype::generate]
    pub fn view(&self, view: &Configuration::View, sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>) {
        log::info!("[StoreViewImplementation::generate]");

        let empty = {
            let changes = self.changes.borrow();
            changes.is_empty()
        };

        if empty { 
            //fast track for no changes in case redraw logic was invoked many times
            return
        }

        let WindowChangeset{
            widgets_to_remove,
            ids_to_add,
            ids_to_update,
            reload: _,
        } = self.compile_changes();

        if widgets_to_remove.is_empty() && ids_to_add.is_empty() && ids_to_update.is_empty() {
            //if all changes leads to identity then return
            return
        }

        let mut widgets = self.widgets.borrow_mut();
        let view_order = self.view.borrow();

        log::warn!("before");
        log::info!("[StoreViewImplementation::generate] data in the store view \t\t\tdata.len(): {}", self.view_data.borrow().len());
        log::info!("[StoreViewImplementation::generate] view should have same length as data.\t\tview.len(): {}", view_order.len());
        log::info!("[StoreViewImplementation::generate] widgets should have same length as view.\twidgets.len(): {}", widgets.len());
        log::info!("[StoreViewImplementation::generate] Should be empty. Is it? {}", self.changes.borrow().is_empty());

        let mut position: Position = Position(*self.range.borrow().start());
        let range = self.range.borrow();
        for id in view_order.iter() {
            if ids_to_add.contains(id) {
                if let Some(record) = self.get(id) {
                    let new_widgets = Configuration::generate(&record, position, sender.clone());
                    let root = Configuration::get_root(&new_widgets);



                    let root = if widgets.is_empty() || position.get() == *range.start() {
                        view.push_front(root)
                    }
                    else {
                        let prev_idx = (position - 1 - *range.start()).get();
                        log::info!("Index of previous elements: {}", prev_idx);
                        let prev_id = view_order[(position - 1 - *range.start()).get()];
                        let prev = widgets.get(&prev_id).unwrap();
                        view.insert_after(root, &prev.root)
                    };
    
                    widgets.insert(
                        *id,
                        widgets::Widgets{
                            widgets: new_widgets,
                            root,
                        }
                    );
                }
            }
            
            if ids_to_update.contains(id) {
                if let Some(record) = self.get(id) {
                    if let Some( widget ) = widgets.get_mut(id) {
                        <Configuration as FactoryConfiguration<Allocator>>::update_record(record, position, &widget.widgets);
                    }
                }
            }

            position = position + 1;
        }

        for id in widgets_to_remove {
            if let Some(widget) = widgets.remove(&id) {
                view.remove(&widget.root);
            }
        }

        log::warn!("after");
        log::info!("[StoreViewImplementation::generate] data in the store view \t\t\tdata.len(): {}", self.view_data.borrow().len());
        log::info!("[StoreViewImplementation::generate] view should have same length as data.\t\tview.len(): {}", view_order.len());
        log::info!("[StoreViewImplementation::generate] widgets should have same length as view.\twidgets.len(): {}", widgets.len());
        log::info!("[StoreViewImplementation::generate] Should be empty. Is it? {}", self.changes.borrow().is_empty());
    }
}