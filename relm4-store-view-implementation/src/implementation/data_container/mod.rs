//! Module defines a data container for the store view
//! 
//! In the store view order of widgets and copy of records held in the store view are tightly related
//! If you add records you it must be also added into collection responsible about the order
//! If you remove a record you must remove it from the view

#[cfg(test)]
mod tests;

use std::slice::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Keys;

use record::Id;
use record::Record;
use record::TemporaryIdAllocator;
use store::DataStore;
use store::FactoryConfiguration;

use crate::WindowChangeset;

/// Data container for the store view implementation
/// 
/// Implementation guarantees:
/// 
/// 1. `data` and `order` at the end of any method have the same length
/// 2. `order` doesn't contain values not present in `data`
pub struct DataContainer<Record, Allocator>
where
    Record: record::Record<Allocator> +'static,
    Allocator: TemporaryIdAllocator,
{
    #[allow(clippy::type_complexity)]
    data: HashMap<Id<Record, Allocator>, Record>,
    order: Vec<Id<Record, Allocator>>,
    max_size: usize,
}

impl<Record, Allocator> DataContainer<Record, Allocator>
where
    Record: record::Record<Allocator> +'static,
    Allocator: TemporaryIdAllocator,
{
    pub fn new(max_size: usize) -> Self {
        let dc = DataContainer{
            data: HashMap::default(),
            order: Vec::default(),
            max_size,
        };

        dc.invariants();

        dc
    }

    fn invariants(&self) {
        for r in &self.order {
            assert!(self.data.contains_key(r), "`data` must contain records for all id's in `order`");
        }
        assert_eq!(self.data.len(), self.order.len(), "`data` and `order` collections must have the same length");
    }

    pub fn record_ids(&self) -> Keys<'_, Id<Record, Allocator>, Record> {
        let keys = self.data.keys();
        keys
    }

    pub fn ordered_record_ids(&self) -> Iter<'_, Id<Record, Allocator>> {
        self.order.iter()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
        self.invariants();
    }

    pub fn reload(&mut self, changeset: &mut WindowChangeset<Record, Allocator>, records: Vec<Record>) {
        let mut old_order = HashSet::new(); 
        old_order.extend(self.order.clone());

        let last_idx = std::cmp::min(self.max_size, records.len());

        self.clear();

        for idx in 0..last_idx {
            let record = records[idx].clone();
            let id = record.get_id();
            self.order.push(id);
            if old_order.contains(&id) {
                //old view had this record
                old_order.remove(&id); //removes id's from old view. It will allow us to remove unneeded data from the view
                changeset.ids_to_update.insert(id);
            }
            else {
                self.data.insert(id, record);
                changeset.ids_to_add.insert(id);
            }
        }

        for id in old_order {           
            changeset.widgets_to_remove.insert(id);
        }

        self.invariants();
    }

    pub fn insert_right(
        &mut self, 
        changeset: &mut WindowChangeset<Record, Allocator>, 
        position: usize, 
        records: Vec<Record>
    ) {
        /*
        
        if for_removal > 0 {
            // Mark data for removal by adding them to changeset and remove them from view_data
            for idx in view_len-for_removal..view_len {
                let id_to_remove = view[idx].clone();
                if view_data.contains_key(&id_to_remove) {
                    view_data.remove(&id_to_remove);
                }
                else {
                    panic!("view_data doesn't contain id which is expected to be there");
                }
                changeset.widgets_to_remove.insert(id_to_remove);
            }
            
            // move all preserved id's to the right
            // insert_right(_, 3, 3)
            // |1, 2, 3, 4, 5, 6 , 7, 8, 9, 10| --> |1, 2, 3, [4], [5], [6], 4, 5, 6, 7|
            //
            // [x] - element to override in next step
            for idx in (start_idx..view_len-for_removal).rev() {
                view[idx+by] = view[idx];
                changeset.ids_to_update.insert(view[idx].clone());
            }

        }

        assert_eq!(view_data.len()+for_removal, view_data_len, "We've just removed `{}` elements from the view_data", for_removal);
        assert_eq!(view_data.len()+for_removal, view.len(), "View data length must be in sync with view");

        // fill up 
        for (idx, record) in data.iter().enumerate() {
            // add new data so we can later generate widgets for them
            let view_insertion_point = idx+start_idx;
            changeset.ids_to_add.insert(record.get_id());
            view_data.insert(record.get_id(), record.clone());

            if view.len() <= view_insertion_point {
                view.push(record.get_id());
            }
            else {
                view[view_insertion_point] = record.get_id();
            }
        }

        */


        self.invariants();
    }

    pub fn insert_left(
        &mut self,
    ) {
        /*

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

        */

        self.invariants();
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    pub fn update(&mut self, r: Record) {
        let id = r.get_id();
        if self.data.contains_key(&id) {
            self.data.insert(id, r);
        }
    }

    pub fn get_order_idx(&self, idx: usize) -> &Id<Record, Allocator>{
        &self.order[idx]
    }

    pub fn get_record(&self, id: &Id<Record, Allocator>) -> Option<&Record> {
        self.data.get(id)
    }

    pub fn get_record_in_order(&self, idx: usize) -> Option<&Record> {
        let id = self.get_order_idx(idx);
        self.get_record(id)
    }
}