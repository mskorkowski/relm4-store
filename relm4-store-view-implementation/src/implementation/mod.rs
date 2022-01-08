mod data_container;
mod data_store;

use reexport::log;
use reexport::relm4;
use store::StoreViewMsg;


use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use relm4::Model as ViewModel;
use relm4::Sender;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use record::Id;

use store::DataStore;
use store::StoreViewPrototype;
use store::Position;
use store::math::Range;
use store::window::StoreState;
use store::window::WindowBehavior;
use store::window::WindowTransition;

use self::data_container::DataContainer;

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
/// 
/// **Warning** This implementation of the store view doesn't work for multisets (aka data repetition).
pub struct StoreViewImplementation<Configuration>
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    store: Configuration::Store,
    #[allow(clippy::type_complexity)]
    view: Rc<RefCell<DataContainer<<Configuration::Store as DataStore>::Record>>>,
    #[allow(clippy::type_complexity)]
    widgets: Rc<RefCell<HashMap<Id<<Configuration::Store as DataStore>::Record>, widgets::Widgets<Configuration::RecordWidgets, <Configuration::View as FactoryView<Configuration::Root>>::Root>>>>,
    changes: Rc<RefCell<Vec<StoreViewMsg<<Configuration::Store as DataStore>::Record>>>>,
    range: Rc<RefCell<Range>>,
    size: usize,
}

impl<Configuration> std::fmt::Debug for StoreViewImplementation<Configuration> 
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoreViewImplementation")
            .field("size", &self.size)
            .field("range", &self.range)
            .finish_non_exhaustive()
    }
}

impl<Configuration> StoreViewImplementation<Configuration> 
where
    Configuration: ?Sized + StoreViewPrototype + 'static,
{
    ///Creates  new instance of this struct
    /// 
    /// - **store** store which will provide a source data
    /// - **size** size of the page
    pub fn new(store: Configuration::Store, size: usize) -> Self {
        let range = Rc::new(RefCell::new(Range::new(0, size)));

        let changes = Rc::new(RefCell::new(Vec::new()));

        changes.borrow_mut().push(StoreViewMsg::Reload);

        Self{
            store,
            view: Rc::new(RefCell::new(DataContainer::new(size))),
            widgets: Rc::new(RefCell::new(HashMap::new())),
            changes,
            range,
            size,
        }
    }

    /// Adds message to the inbox
    /// 
    /// Messages are handled at the render time in batch
    pub fn inbox(&self, message: StoreViewMsg<<Configuration::Store as DataStore>::Record>) {
        self.changes.borrow_mut().push(message);
    }

    fn convert_to_transition(&self, state: &StoreState<'_>, message: &StoreViewMsg<<Configuration::Store as DataStore>::Record>) -> WindowTransition {
        match message {
            StoreViewMsg::NewAt(p) => {
                Configuration::Window::insert(state, &p.to_point())
            },
            StoreViewMsg::Move{from, to} => {
                Configuration::Window::slide(state, &Range::new(from.0, to.0))
            },
            StoreViewMsg::Reorder{from, to} => {
                Configuration::Window::slide(state, &Range::new(from.0, to.0))
            },
            StoreViewMsg::Remove(at) => {
                Configuration::Window::remove(state, &at.to_point())
            },
            StoreViewMsg::Commit(_) => {
                WindowTransition::Identity
            },
            StoreViewMsg::Update(_) => {
                WindowTransition::Identity
            },
            StoreViewMsg::Reload => {
                WindowTransition::Identity
            },
        }
    }

    fn reload(&self, changeset: &mut WindowChangeset<<Configuration::Store as DataStore>::Record>) {
        let range_of_changes = self.range.borrow().clone();
        log::trace!("Range of changes {:?}", range_of_changes);
        let new_records: Vec<<Configuration::Store as DataStore>::Record> = self.store.get_range(&range_of_changes);
        log::trace!("New records length: {}", new_records.len());
        let mut view = self.view.borrow_mut();
        
        view.reload(changeset, new_records);
    }

    /// Inserts `by` elements at the position `pos`
    /// 
    /// Insert is limited by the page size. For example if the window starts at `10` and ends at `20`, and you insert
    /// `5` records at position `18` you will basically insert two elements 18 and 19.
    fn insert_right(&self, changeset: &mut WindowChangeset<<Configuration::Store as DataStore>::Record>, pos: usize, by: usize) {
        // it's responsibility of the WindowBehavior to make this math valid and truncate `pos` and `by` to the acceptable range
        // and WindowBehavior logic was called before we reached here so we can assume we are safe here
        let mut view = self.view.borrow_mut();
        let range = self.range.borrow();

        let range_of_changes = Range::new(pos, pos+by);
        let data = self.store.get_range(&range_of_changes);
        let position = pos - range.start();

        view.insert_right(changeset, position, data);
        
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
    fn insert_left(&self, changeset: &mut WindowChangeset<<Configuration::Store as DataStore>::Record>, pos: usize, by: usize) {
        // it's responsibility of the WindowBehavior to make this math valid and truncate `pos` and `by` to the acceptable range
        // and WindowBehavior logic was called before we reached here so we can assume we are safe here

        let mut view = self.view.borrow_mut();
        
        let start = {
            let range = self.range.borrow();
            *range.start()
        };
        let range_of_changes = Range::new(pos, pos+by);
        let data = self.store.get_range(&range_of_changes);
        
        let position = pos - start;

        if start == 0 && view.len() + by <= self.size {
            view.insert_right(changeset, position, data);
        }
        else {
            view.insert_left(changeset, position, data);
            //insert_left is not neutral in terms of the data range
            //so we need to move the window to the right by the `by`
            
            let new_range = {
                let range = self.range.borrow();
                range.to_right(by)
            };
            self.range.replace(new_range);
        }


    }

    fn compile_changes(&self) -> WindowChangeset<<Configuration::Store as DataStore>::Record> {
        let mut changeset = WindowChangeset::default();

        for change in self.changes.borrow_mut().iter() {
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
                    match change {
                        StoreViewMsg::Update(id) => {
                            let mut view = self.view.borrow_mut();
                            if let Some(record) = self.store.get(id) {
                                changeset.ids_to_update.insert(*id);
                                view.update(record);
                            }
                        },
                        StoreViewMsg::Reload => {
                            log::trace!("Reload");
                            changeset.reload = true;
                            self.reload(&mut changeset);
                        },
                        _ => {}
                    }
                },
                WindowTransition::InsertLeft{pos, by } => {
                    log::trace!("Insert left");
                    self.insert_left(&mut changeset, pos, by);
                }
                WindowTransition::InsertRight{pos, by} => {
                    log::trace!("Insert right");
                    self.insert_right(&mut changeset, pos, by);
                }
                WindowTransition::RemoveLeft{pos: _, by: _} => {
                    log::error!("RemoveLeft - unimplemented yet");
                }
                WindowTransition::RemoveRight{pos: _, by: _} => {
                    log::error!("RemoveRight - unimplemented yet");
                }
                WindowTransition::SlideLeft(_by) => {
                    log::error!("SlideLeft - unimplemented yet");
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

        self.changes.replace(vec![]);

        // if we need to add record since it was not present we can remove it from updates
        //
        // TODO: Is it still worthy to run this loop? Might be since rendering is separate if statements over changeset
        for id in &changeset.ids_to_add {
            changeset.ids_to_update.remove(id);
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

        let old_order = {
            let view_order = self.view.borrow();
            let iter = view_order.ordered_record_ids();
            let mut v = Vec::with_capacity(view_order.len());
            for id in iter { //manual copy cos of the lifetime
                v.push(id.clone());
            }

            v
        };
        let old_order_len = old_order.len();

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

        log::trace!("[StoreViewImplementation::generate] view should have same length as data.\t\tview.len(): {}", view_order.len());
        log::trace!("[StoreViewImplementation::generate] widgets should have same length as view.\twidgets.len(): {}", widgets.len());
        log::trace!("[StoreViewImplementation::generate] Ordered record ids should have same length as view. \tview_order.ordered_record_ids(): {}", view_order.ordered_record_ids().len());
        log::trace!("[StoreViewImplementation::generate] Changes should be empty. Is it? {}", self.changes.borrow().is_empty());
        log::trace!("[StoreViewImplementation::generate]");
        log::trace!("[StoreViewImplementation::generate] ids_to_add.len(): {}", ids_to_add.len());
        log::trace!("[StoreViewImplementation::generate] ids_to_update.len(): {}", ids_to_update.len());

        let mut position: Position = Position(*self.range.borrow().start());
        let range = self.range.borrow();
        for id in view_order.ordered_record_ids() {
            if ids_to_add.contains(id) {
                log::trace!("[StoreViewImplementation::generate] Id to add: {:?}", id);
                if let Some(record) = self.get(id) {
                    log::trace!("[StoreViewImplementation::generate] Got record {:?}", record);
                    let new_widgets = Configuration::generate(&record, position, sender.clone());
                    let widgets_root = Configuration::get_root(&new_widgets);

                    let root = if widgets.is_empty() || position.0 == *range.start() {
                        log::trace!("[StoreViewImplementation::generate] Adding first element");
                        view.push_front(widgets_root)
                    }
                    else {
                        log::trace!("[StoreViewImplementation::generate] Adding non first element");
                        let prev_idx = (position - 1 - *range.start()).0;
                        log::info!("Index of previous elements: {}", prev_idx);
                        let prev_id = view_order.get_order_idx((position - 1 - *range.start()).0);
                        let prev = widgets.get(&prev_id).unwrap();
                        view.insert_after(widgets_root, &prev.root)
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
                        <Configuration as StoreViewPrototype>::update_record(record, position, &widget.widgets);
                        if old_order_len > position.0 {
                            if old_order[position.0] != *id {
                                // things got reordered so we need to remove widget from old place and attach it to the new one
                                if let Some(widget) = widgets.remove(&id) {
                                    view.remove(&widget.root);
                                    let root = if position.0 == *range.start() {
                                        view.push_front(Configuration::get_root(&widget.widgets))
                                    }
                                    else {
                                        let prev_idx = (position - 1 - *range.start()).0;
                                        log::info!("Index of previous elements: {}", prev_idx);
                                        let prev_id = view_order.get_order_idx((position - 1 - *range.start()).0);
                                        let prev = widgets.get(&prev_id).unwrap();
                                        view.insert_after(Configuration::get_root(&widget.widgets), &prev.root)
                                    };

                                    widgets.insert(
                                        *id,
                                        widgets::Widgets{
                                            widgets: widget.widgets,
                                            root,
                                        }
                                    );
                                }
                            }
                        }
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

        log::trace!("after");
        log::trace!("[StoreViewImplementation::generate] view should have same length as data.\t\tview.len(): {}", view_order.len());
        log::trace!("[StoreViewImplementation::generate] widgets should have same length as view.\twidgets.len(): {}", widgets.len());
        log::trace!("[StoreViewImplementation::generate] Should be empty. Is it? {}", self.changes.borrow().is_empty());
    }
}
