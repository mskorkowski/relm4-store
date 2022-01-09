//! Module defines a data container for the store view
//! 
//! In the store view order of widgets and copy of records held in the store view are tightly related
//! If you add records you it must be also added into collection responsible about the order
//! If you remove a record you must remove it from the view

#[cfg(test)]
mod tests;

use reexport::log;

use std::slice::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Keys;

use record::Id;

use crate::WindowChangeset;

/// Data container for the store view implementation
/// 
/// ## Implementation guarantees:
/// 
/// 1. `data` and `order` at the end of any method have the same length
/// 2. `order` doesn't contain values not present in `data`
/// 
/// ## Why `data` and `order`?
/// 
/// - Records in the `data` might be fairly big compared to id
/// - Size of record might not be known at compilation time
/// - It's impossible to implement `Copy` for the records in generic case
/// - It's trivial to implement `Copy` for id in most cases and all other cases I'm aware of it's enough to overallocate
///   id's so they can hold any id which can be set
/// 
/// ----
/// 
/// ## Overallocate example
/// 
/// If you are reading this I assume your db is properly designed and you have thought about it really hard. Personally
/// I would be thinking at least two times more about that.
/// 
/// You can't help it. For ease of thinking let's assume your id is a string of length of up to 50 latin characters.
/// In such case your id's should use a slice of characters with length of 50 `[char, 50]`. This creates a place for your
/// id and you can provide a copy for it. It will require from you to decide how to mark unused parts of your id, how to 
/// align your data in this 50 characters, how to compare them efficiently, etc... It's the best solution I can think of
/// in such a case.
/// 
/// If your id's are much shorter then 50 characters and this wastes memory that's a good indicator that your database design
/// probably needs rethinking.
pub(crate) struct DataContainer<Record>
where
    Record: 'static + ?Sized + record::Record + std::fmt::Debug,
{
    /// Keeps copy of records in the view
    data: HashMap<Id<Record>, Record>,
    /// Keeps ordered vector of records
    order: Vec<Id<Record>>,
    /// Maximum number of elements in the data container
    max_size: usize,
}

impl<Record> DataContainer<Record>
where
    Record: 'static + record::Record + std::fmt::Debug,
{
    pub(crate) fn new(max_size: usize) -> Self {
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

        for record in self.data.values() {
            assert!(self.order.contains(&record.get_id()), "Order must contain entries for all records in data");
        }
        assert_eq!(self.data.len(), self.order.len(), "`data` and `order` collections must have the same length");
        assert!(self.max_size >= self.len(), "DataContainer size can't exceed max size");
    }

    pub(crate) fn record_ids(&self) -> Keys<'_, Id<Record>, Record> {
        let keys = self.data.keys();
        keys
    }

    pub(crate) fn ordered_record_ids(&self) -> Iter<'_, Id<Record>> {
        self.order.iter()
    }

    pub(crate) fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
        self.invariants();
    }

    pub(crate) fn reload(&mut self, changeset: &mut WindowChangeset<Record>, records: Vec<Record>) {
        let mut old_order = HashSet::new(); 
        old_order.extend(self.order.clone());

        
        let last_idx = std::cmp::min(self.max_size, records.len());
        
        self.clear();

        log::trace!("[reload] old_order.len(): {}", old_order.len());
        log::trace!("[reload] last idx: {}", last_idx);

        for idx in 0..last_idx {
            let record = records[idx].clone();
            let id = record.get_id();
            self.order.push(id);
            self.data.insert(id, record);
            if old_order.contains(&id) {
                log::trace!("[reload] old order contains the record");
                //old view had this record
                old_order.remove(&id); //removes id's from old view. It will allow us to remove unneeded data from the view
                changeset.ids_to_update.insert(id);
            }
            else {
                log::trace!("[reload] old order doesn't contain the record");
                changeset.ids_to_add.insert(id);
            }
        }

        for id in old_order {           
            changeset.widgets_to_remove.insert(id);
        }

        self.invariants();
    }

    /// Inserts records to the right of the position
    /// 
    /// If there is more **records** then place to insert the data only fitting data from the beginning of the
    /// **records** will be inserted
    /// 
    /// - **changeset** structure holding information which elements of the view require update
    /// - **position** index at which first record will be inserted
    /// - **records** ordered vector holding values to be inserted
    pub(crate) fn insert_right(
        &mut self, 
        changeset: &mut WindowChangeset<Record>, 
        position: usize, 
        records: Vec<Record>
    ) {
        let starting_len = self.len();

        // if there is more records then place to put them truncate that to available space
        let records_len = std::cmp::min(
            records.len(), 
            self.max_size - position
        );

        let for_removal = if starting_len + records_len >= self.max_size {
            // returns amount of records above max size
            starting_len + records_len - self.max_size
        }
        else {
            // there is enough capacity in the data container to add all records without removal
            0
        };
        
        let remove_start_idx = starting_len - for_removal;
        let remove_end_idx = starting_len; //last index to remove (exclusive)
        
        // Mark data for removal by adding them to changeset and remove them from view_data
        for idx in remove_start_idx..remove_end_idx {
            let id_to_remove = self.order[idx].clone();
            
            // if we test whole implementation to hold invariants in all cases this can be simplify
            // to `self.data.remove(&id_to_remove)`
            if self.data.contains_key(&id_to_remove) {
                self.data.remove(&id_to_remove);
            }
            else {
                // if something is in the order it must be in data, if we are here invariants are broken
                panic!("view_data doesn't contain id which is expected to be there");
            }
            
            changeset.widgets_to_remove.insert(id_to_remove);
        }
        
        // move all preserved id's to the right
        // insert_right at 3, 3 records, max length 10
        // |1, 2, 3, 4, 5, 6, 7, 8, 9, 10| --> |1, 2, 3, [4], [5], [6], 4, 5, 6, 7|
        //
        // [x] - element to override in next step
        let move_end_idx = remove_start_idx; // range is exclusive at the right end
        for idx in (position..move_end_idx).rev() {
            let id = self.order[idx];
            let target_idx = idx+records_len;
            if target_idx < self.order.len() {
                self.order[idx+records_len] = id;
            }
            else {
                self.order.push(id);
            }
            changeset.ids_to_update.insert(id);
        }

        // fill up 
        for idx in 0..records_len {
            let record = &records[idx];
            // add new data so we can later generate widgets for them
            let view_insertion_point = idx+position;
            changeset.ids_to_add.insert(record.get_id());
            self.data.insert(record.get_id(), record.clone());

            if self.len() <= view_insertion_point {
                self.order.push(record.get_id());
            }
            else {
                self.order[view_insertion_point] = record.get_id();
            }
        }

        self.invariants();
    }


    /// Inserts records to the left of the position
    /// 
    /// If there is more **records** then place to insert the data only feting data from the end of the
    /// **records** will be inserted
    /// 
    /// - **changeset** structure holding information which elements of the view require update
    /// - **position** index at which last record will be inserted
    /// - **records** ordered vector holding values to be inserted
    pub(crate) fn insert_left(
        &mut self,
        changeset: &mut WindowChangeset<Record>, 
        position: usize, 
        records: Vec<Record>
    ) {
        let starting_len = self.len();
        let total_records_len = records.len();
        let end = position;


        let records_len = std::cmp::min(
            total_records_len,
            end,
        );

        if records_len > 0 {
            // we need to remove `records_len` elements so we can later insert this many 
            // records before `position`
            let remove_end_idx = std::cmp::min(
                starting_len,
                records_len
            );

            for idx in 0..remove_end_idx {
                let id_to_remove = self.order[idx];

                // if we test whole implementation to hold invariants in all cases this can be simplify
                // to `self.data.remove(&id_to_remove)`
                if self.data.contains_key(&id_to_remove) {
                    self.data.remove(&id_to_remove);
                }
                else {
                    // if something is in the order it must be in data, if we are here invariants are broken
                    panic!("view_data doesn't contain id which is expected to be there");
                }

                changeset.widgets_to_remove.insert(id_to_remove);
            }

            let move_start_idx = records_len; // 0<= move_start_idx < position since 0 < records_len <= position

            let move_end_idx = std::cmp::min(
                starting_len,
                end
            );

            for idx in move_start_idx..move_end_idx {
                let idx_from_start = idx - move_start_idx;
                let id = self.order[idx];
                self.order[idx_from_start] = id;
                changeset.ids_to_update.insert(id);
            }
        }


        // fill up 
        for idx in 0..records_len {
            let idx_in_records = total_records_len - records_len + idx;
            let record = &records[idx_in_records];
            // add new data so we can later generate widgets for them
            let view_insertion_point = end - records_len + idx;
            changeset.ids_to_add.insert(record.get_id());
            self.data.insert(record.get_id(), record.clone());

            if self.len() <= view_insertion_point {
                self.order.push(record.get_id());
            }
            else {
                self.order[view_insertion_point] = record.get_id();
            }
        }

        self.invariants();
    }

    /// Removes records to the right of the position
    /// 
    /// If there is more **records** then place to insert the data only fitting data from the beginning of the
    /// **records** will be inserted
    /// 
    /// - **changeset** structure holding information which elements of the view require update
    /// - **position** index at which first record will be removed
    /// - **records** ordered vector holding values to be inserted
    // pub(crate) fn remove_right(
    //     &mut self,
    //     changeset: &mut WindowChangeset<Record>, 
    //     position: usize, 
    //     records: Vec<Record>
    // ) {
    //     let starting_len = self.len();
        

    // }

    pub(crate) fn len(&self) -> usize {
        self.order.len()
    }

    pub(crate) fn update(&mut self, r: Record) {
        let id = r.get_id();
        if self.data.contains_key(&id) {
            self.data.insert(id, r);
        }
    }

    pub(crate) fn get_order_idx(&self, idx: usize) -> &Id<Record>{
        &self.order[idx]
    }

    pub(crate) fn get_record(&self, id: &Id<Record>) -> Option<&Record> {
        self.data.get(id)
    }
}

impl<Record> std::fmt::Debug for DataContainer<Record>
where
    Record: 'static + record::Record + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sf = f.debug_struct("DataContainer");
        
        let mut data = vec![];

        for (idx, id) in self.order.iter().enumerate() {
            if let Some(r) = self.data.get(&id) {
                data.push(
                    format!("{} => {:?}", idx, r)
                )
            }
            else {
                data.push(
                    format!("{} => Missing `{:#?}`", idx, id)
                )
            }
        }

        sf.field("data", &data);

        let mut out_of_order = vec![];

        for (id, record) in &self.data {
            if !self.order.contains(&id) {
                out_of_order.push(record);
            }
        }

        sf.field("out of order values", &out_of_order);

        sf.finish()
    }
}
