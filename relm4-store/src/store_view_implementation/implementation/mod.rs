mod data_store;
mod factory;

use reexport::relm4;


use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use relm4::Model as ViewModel;
use relm4::Sender;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use record::DefaultIdAllocator;
use record::Id;
use record::Record;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::FactoryConfiguration;
use crate::Handler;
use crate::Position;
use crate::StoreId;
use crate::StoreMsg;
use crate::math::Range;
use crate::redraw_messages::RedrawMessages;
use crate::window::WindowBehavior;
use crate::window::WindowTransition;

use super::window_changeset::WindowChangeset;
use super::widgets;

pub use data_store::StoreViewImplHandler;

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
    handlers: RefCell<HashMap<StoreId<Self, Allocator>, Box<dyn Handler<Self, Allocator>>>>,
    #[allow(clippy::type_complexity)]
    view_data: RefCell<HashMap<Id<<Configuration::Store as DataStore<Allocator>>::Record>, <Configuration::Store as DataStore<Allocator>>::Record>>,
    view: RefCell<VecDeque<Id<<Configuration::Store as DataStore<Allocator>>::Record>>>,
    #[allow(clippy::type_complexity)]
    widgets: RefCell<HashMap<Id<<Configuration::Store as DataStore<Allocator>>::Record>, widgets::Widgets<Configuration::RecordWidgets, <Configuration::View as FactoryView<Configuration::Root>>::Root>>>,
    changes: RefCell<Vec<StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>>>,
    range: RefCell<Range>,
    size: usize,
    redraw_sender: Sender<RedrawMessages>,
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
            redraw_sender,
        }
    }

    fn convert_to_transition(&self, range: &Range, message: &StoreMsg<<Configuration::Store as DataStore<Allocator>>::Record>) -> WindowTransition {
        match message {
            StoreMsg::NewAt(p) => {
                Configuration::Window::insert(range, &p.to_point())
            },
            StoreMsg::Move{from, to} => {
                Configuration::Window::slide(range, &Range::new(from.0, to.0))
            },
            StoreMsg::Reorder{from, to} => {
                Configuration::Window::slide(range, &Range::new(from.0, to.0))
            },
            StoreMsg::Remove(at) => {
                Configuration::Window::remove(range, &at.to_point())
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

        //TODO: Optimise it... it has loads of unnecessary updates
        let store = self.store.borrow();
        let range_of_changes = self.range.borrow().clone();
        let new_items: Vec<<Self as DataStore<Allocator>>::Record> = store.get_range(&range_of_changes);
        let mut view = self.view.borrow_mut();

        //remove unused data
        let last_idx = 0;
        let view_range = view.range(last_idx..);

        for id in view_range {
            self.view_data.borrow_mut().remove(id);
            changeset.widgets_to_remove.insert(*id);
        }
        view.truncate(last_idx); //remove by elements from view


        //remove unneeded data from view
        let mut len = 0;
        let new_items_len = new_items.len();
        while !store.is_empty() && len <  new_items_len {
            let record = new_items.get(len).unwrap();
            view.remove(len);

            view.insert(len, record.get_id());
            self.view_data.borrow_mut().insert(record.get_id(), record.clone());
            changeset.ids_to_add.insert(record.get_id());
            len += 1;
        }
    }

    fn insert_right(&self, changeset: &mut WindowChangeset<Configuration, Allocator>, pos: usize, by: usize) {
        let store = self.store.borrow();
        // let end = *self.range.borrow().end();
        let start = *self.range.borrow().start();
        let range_of_changes = Range::new(pos, pos+by);
        let new_items: Vec<<Self as DataStore<Allocator>>::Record> = store.get_range(&range_of_changes);

        let mut view = self.view.borrow_mut();

        //remove unused data
        if view.len() + new_items.len() >= self.size {
            let last_idx = pos - start;
            let view_range = view.range(last_idx..);

            for id in view_range {
                self.view_data.borrow_mut().remove(id);
                changeset.widgets_to_remove.insert(*id);
            }
            view.truncate(last_idx); //remove by elements from view
        }

        //remove unneeded data from view
        let mut len = 0;
        let new_items_len = new_items.len();
        while !store.is_empty() && len <  new_items_len && len < by {
            let record = new_items.get(len).unwrap();
            view.remove(pos+len);

            view.insert(pos+len-start, record.get_id());
            self.view_data.borrow_mut().insert(record.get_id(), record.clone());
            changeset.ids_to_add.insert(record.get_id());
            len += 1;
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

        println!("Insert left");
        let store = self.store.borrow();
        let start = *self.range.borrow().start();

        if pos-start < by || start == 0 {
            self.insert_right(changeset, pos, by)                        
        }
        else {
            let start_pos = if pos-start < by { 
                println!("\t\tstart_pos = start");
                start 
            } else {
                println!("\t\tstart_pos = pos - by");
                pos-by
            };
            let end_pos = if start_pos == pos { 
                println!("\t\tend_pos = pos + by");
                pos + by 
            } else { 
                println!("\t\tend_pos = pos");
                pos 
            };
            let range_of_changes = Range::new(start_pos, end_pos);

            println!("\tpos: {}", pos);
            println!("\tby: {}", by);
            println!("\tstart: {}", start);
            println!("\tstart_pos: {}", start_pos);
            println!("\tend_pos: {}", end_pos);
            println!("\trange_of_changes: {}", range_of_changes);

            let new_items: Vec<<Self as DataStore<Allocator>>::Record> = store.get_range(&range_of_changes);

            let mut view = self.view.borrow_mut();

            //remove unused data
            if view.len() >= self.size {
                let view_range = view.range(..by);

                for id in view_range {
                    self.view_data.borrow_mut().remove(id);
                    changeset.widgets_to_remove.insert(*id);
                }

                //remove by elements from view
                for _ in 0..by {
                    view.pop_front(); 
                }

            }

            //remove unneeded data from view
            let mut len = 0;
            let new_items_len = new_items.len();
            while !store.is_empty() && len <  new_items_len && len < by {
                let record = new_items.get(len).unwrap();
                view.remove(pos+len);

                view.insert(pos+len-start, record.get_id());
                self.view_data.borrow_mut().insert(record.get_id(), record.clone());
                changeset.ids_to_add.insert(record.get_id());
                len += 1;
            }
        }                    
    }

    fn compile_changes(&self) -> WindowChangeset<Configuration, Allocator> {
        let mut changeset = WindowChangeset {
            widgets_to_remove: HashSet::new(),
            ids_to_add: HashSet::new(),
            ids_to_update: HashSet::new(),
        };

        let mut changes = self.changes.borrow_mut();

        for change in changes.iter() {
            let transition = self.convert_to_transition(&self.range.borrow(), change);

            match transition {
                WindowTransition::Identity => {
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
                            self.reload(&mut changeset);
                        },
                        _ => {}
                    }
                },
                WindowTransition::InsertLeft{pos, by } => {
                    self.insert_left(&mut changeset, pos, by);
                }
                WindowTransition::InsertRight{pos, by} => {
                    self.insert_right(&mut changeset, pos, by);
                }
                WindowTransition::RemoveLeft{pos: _, by: _} => {}
                WindowTransition::RemoveRight{pos: _, by: _} => {}
                WindowTransition::SlideLeft(_by) => {

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
        }

        changes.clear();

        for id in &changeset.ids_to_update {
            changeset.ids_to_add.remove(id);
        }

        changeset
    }

    /// Implementation of the [relm4::factory::FactoryPrototype::generate]
    pub fn view(&self, view: &Configuration::View, sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>) {
        println!("[StoreViewImplementation::generate]");

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
        } = self.compile_changes();

        if widgets_to_remove.is_empty() && ids_to_add.is_empty() && ids_to_update.is_empty() {
            //if all changes leads to identity then return
            return
        }

        let mut widgets = self.widgets.borrow_mut();
        let view_order = self.view.borrow();

        for id in widgets_to_remove {
            if let Some(widget) = widgets.remove(&id) {
                view.remove(&widget.root);
            }
        }

        let mut position: Position = Position(*self.range.borrow().start());
        for id in view_order.iter() {
            if ids_to_add.contains(id) {
                if let Some(record) = self.get(id) {
                    let new_widgets = Configuration::generate(&record, position, sender.clone());
                    let root = Configuration::get_root(&new_widgets);
                    let range = self.range.borrow();
                    let root = if widgets.is_empty() || position.get() == *range.start() {
                        view.push_front(root)
                    }
                    else {
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
    }
}