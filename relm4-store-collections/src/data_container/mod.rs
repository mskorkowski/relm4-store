//! Module defines a data container for the store view
//! 
//! In the store view order of widgets and copy of records held in the store view are tightly related. If you add records it
//! must be also added into collection responsible for handling order. If you remove a record you must remove it from the view.
//! 
//! Thats complex enough but view also needs to calculate the range of data which needs to be loaded from data store so operation
//! can behave in a sane way. View must optimise the request so it won't be asking for unnecessary data on the other hand, view also
//! should reduce amount of queries sent to the data store. Selecting which strategy is more important can depend on the data store
//! implementation, network speed, hardware being used, etc.
//! 
//! To make implementation of the store view simpler the logic related to keeping data and order consistent has been extracted to the
//! [DataContainer] structure.

#[cfg(test)]
mod tests;

mod window_changeset;

use reexport::log;

use std::cmp::min;
use std::slice::Iter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Keys;
use std::ops::Range;

use record::Id;

pub use window_changeset::WindowChangeset;

/// Data container for the store view implementation
/// 
/// ## Implementation guarantees:
/// 
/// 1. `data` and `order` at the end of any method have the same length
/// 2. `order` doesn't contain values not present in `data`
/// 
/// ## Why `data` and `order`?
/// 
/// - Records in the `data` in most of the cases is many times larger comparing to the size of an id
/// - Size of record might not be known at compilation time
/// - It's impossible to implement `Copy` for the records in generic case
/// - It's trivial to implement `Copy` for id in most cases and all other cases I'm aware of it's enough to overallocate
///   id's so they can hold any id which can be set
/// 
/// ## Why `left` and `right`
/// 
/// Methods of the container are designed to reduce allocations required. For example if I need to remove two records
/// from the collection and there exist a value to fill their place I can remove records, move values to make a place
/// on the proper side of collection and override whatever values were on given end. Since in most cases this collection
/// should be relatively small (less then 100 elements) this move operation should be fast. What's more it's about moving
/// copyable values (id's) which are relatively small.
/// 
/// Records are stored in the hashmap and are being treated as unmovable objects as much as possible since they are much
/// bigger then id's. Since this collection is small hashmap index should not break memory locality too much. Only value
/// we need from the hash map is the reference to the record which is also compact. Only time when locality would be
/// broken is when `generate`/`update` method would be run but at that point locality is already broken seriously since
/// this method are used to either define ui or update ui which in itself is costly operation also from memory side.
/// 
/// ## Left and right operations ranges
/// 
/// | Operation                                   | Arguments*        | Range                                 |
/// |:--------------------------------------------|:------------------|:--------------------------------------|
/// | [insert_right][DataContainer::insert_right] | position, records | [position, position + records.len() ) |
/// | [remove_right][DataContainer::remove_right] | position, by      | [position, position + by              |
/// | [insert_left][DataContainer::insert_left]   | position, records | [position - records.len(), position ) |
/// | [remove_left][DataContainer::remove_left]   | position, by      | [position - by, position )            |
/// 
/// `*` Only arguments meaningful for discussion are shown here
/// 
/// As you can see for left operations first record which is outside of scope is at `position`. For right first affected
/// record is the one at the position index. This makes glueing operations much easier.
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
pub struct DataContainer<Record>
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
    /// Creates new instance of the DataContainer with given maximum size
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
            assert!(self.data.contains_key(r), "`data` must contain records for all id's in `order`. Missing record for: {:?}", r);
        }

        for record in self.data.values() {
            assert!(self.order.contains(&record.get_id()), "Order must contain entries for all records in data. Missing order entry for {:#?}", record);
        }
        assert_eq!(self.data.len(), self.order.len(), "`data` and `order` collections must have the same length");
        assert!(self.max_size >= self.len(), "DataContainer size can't exceed max size");
    }

    /// Returns iterator with id's of records in the container
    pub fn record_ids(&self) -> Keys<'_, Id<Record>, Record> {
        self.data.keys()
    }

    /// Returns iterator with id's of records in the container in order in which they should be shown 
    pub fn ordered_record_ids(&self) -> Iter<'_, Id<Record>> {
        self.order.iter()
    }

    /// Removes all data from the container
    pub fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
        self.invariants();
    }

    /// Removes all data from the container and adds up to `[max_size][DataContainer::max_size]` of values from `records`
    /// 
    /// Data which were removed are added to the `changeset`
    pub fn reload(&mut self, changeset: &mut WindowChangeset<Record>, records: Vec<Record>) {
        let mut old_order = HashSet::new(); 
        old_order.extend(self.order.clone());

        
        let last_idx = std::cmp::min(self.max_size, records.len());
        
        self.clear();

        log::trace!("[reload] old_order.len(): {}", old_order.len());
        log::trace!("[reload] last idx: {}", last_idx);

        for record in records.iter().take(last_idx) {
            let id = record.get_id();
            self.order.push(id);
            self.data.insert(id, record.clone());
            if old_order.contains(&id) {
                log::trace!("[reload] old order contains the record");
                //old view had this record
                old_order.remove(&id); //removes id's from old view. It will allow us to remove unneeded data from the view
                changeset.update(id);
            }
            else {
                log::trace!("[reload] old order doesn't contain the record");
                changeset.add(id);
            }
        }

        for id in old_order {           
            changeset.remove(id);
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
    pub fn insert_right(
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
        
        self.mark_removed(changeset, remove_start_idx..remove_end_idx);

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
            changeset.update(id);
        }

        // fill up 
        for (idx, record) in records.iter().enumerate().take(records_len) {
            // add new data so we can later generate widgets for them
            let view_insertion_point = idx+position;
            changeset.add(record.get_id());
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
    pub fn insert_left(
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

            self.mark_removed(changeset, 0..remove_end_idx);

            let move_start_idx = records_len; // 0<= move_start_idx < position since 0 < records_len <= position

            let move_end_idx = std::cmp::min(
                starting_len,
                end
            );

            for idx in move_start_idx..move_end_idx {
                let idx_from_start = idx - move_start_idx;
                let id = self.order[idx];
                self.order[idx_from_start] = id;
                changeset.update(id);
            }
        }


        // fill up 
        for idx in 0..records_len {
            let idx_in_records = total_records_len - records_len + idx;
            let record = &records[idx_in_records];
            // add new data so we can later generate widgets for them
            let view_insertion_point = end - records_len + idx;
            changeset.add(record.get_id());
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
    pub fn remove_right(
        &mut self,
        changeset: &mut WindowChangeset<Record>, 
        position: usize,
        by: usize,
        records: Vec<Record>
    ) {
        let starting_len = self.len();
        let records_len = records.len();

        let end_of_move = if starting_len == 0 || position >= starting_len {
            starting_len
        }
        else {
            for idx in position..position+by {
                let id = self.order[idx];
                self.data.remove(&id);
                changeset.remove(id);
            }

            let to_remove = if by > records_len {
                by - records_len
            }
            else {
                0
            };

            let first_move_idx = if position + by > starting_len {
                // there is not enough records to be removed
                starting_len
            }
            else {
                position + by
            };

            let max_delta = starting_len - first_move_idx;

            // remove records from data
            // move all records to the left to make a place for new values
            for delta in 0..max_delta {
                let pos = position+delta;
                let move_pos = first_move_idx+delta;

                let id_to_update = self.order[move_pos];
                self.order[pos] = self.order[move_pos];
                changeset.update(id_to_update);
            }

            for _ in 0..to_remove {
                //we'e moved all required records to the left
                //now we need to remove repeated id's which won't be overridden in next step
                if let Some(id) = self.order.pop() {
                    //if we are don't have enough to move earlier then we might hit the case
                    //when we remove values unknown to changeset yet
                    if !changeset.remove_contains(&id) && max_delta == 0 {
                        changeset.remove(id);
                        self.data.remove(&id);
                    }
                }
            }

            position+max_delta
        };

        let end_of_insert = if end_of_move + records_len > self.max_size {
            self.max_size
        }
        else {
            end_of_move + records_len
        };

        let max_insert_range_delta = end_of_insert - end_of_move;

        // make sure that we don't try to insert more records then we have
        let max_insert_delta = if records_len < max_insert_range_delta {
            records_len
        }
        else {
            max_insert_range_delta
        };

        for idx in 0..max_insert_delta {
            let pos = end_of_move+idx;
            let record = records[idx].clone();
            let id = record.get_id();
            self.data.insert(id, record);
            if pos >= self.order.len() {
                self.order.push(id);
            }
            else {
                self.order[pos] = id;
            }
            changeset.add(id);
        }

        self.invariants();
    }

    /// Removes records to the left of the position
    /// 
    /// If you remove 5 records, records_left will be used to fill up to 5 records.
    /// 
    /// Last record removed is at `position-1`.
    /// 
    /// If there is more **records** then place to insert the data only fitting data from the beginning of the
    /// **left_records** will be inserted
    /// 
    /// If **left_records** are not enough to fill the container up to the max amount of records **right_records**
    /// will be use used to fill the dat from the right hand side
    /// 
    /// - **changeset** structure holding information which elements of the view require update
    /// - **position** index at which first record will be removed
    /// - **left_records** ordered vector holding values to be inserted from the left. These are not a "new" records. They are records existing in the
    ///     data store before
    /// - **right_records** ordered vector holding value to be inserted from the right if there is not enough values
    pub fn remove_left(
        &mut self,
        changeset: &mut WindowChangeset<Record>, 
        position: usize,
        by: usize,
        left_records: Vec<Record>,
        right_records: Vec<Record>,
    ) {
        let starting_len = self.len();
        let left_records_len = left_records.len();
        let right_records_len = right_records.len();

        if starting_len == 0 {
            return;
        }

        // we are going to remove [first_removed_idx, position)
        //
        // first_removed_idx: [0, position]
        let first_removed_idx = if position < by {
            0
        }
        else {
            position - by
        };

        // this many records will be removed from container
        let records_to_remove = position - first_removed_idx;

        // moved indexes are from [0, first_removed_idx)
        // so we are going to move `first_removed_idx` of elements
        // so we need as much of elements in the left_records
        // if there is not enough elements in the left_records we need to move values after position to the left by the missing part
        //
        // left_move_size: how many records we will moved to the right **from left side** to cover a gap created by deletion
        //                  [left_records_len, position] =>
        //                  [0, position]
        // right_move_size: how many records will be moved to the left **from right side** to cover a gap created by deletion
        //                  [0,  first_removed_idx-left_records_len] <=
        //                  [0, position - left_records_len] <=
        //                  [0, position]
        let (left_move_size, right_move_size) = if records_to_remove > left_records_len {
            //we need to move records to the wright
            //
            (left_records_len, records_to_remove - left_records_len)
        }
        else {
            (records_to_remove, 0)
        };

        //marking records as to be deleted
        self.mark_removed(changeset, first_removed_idx..position);

        if left_move_size > 0 { //if we don't have a need for a loop don't do it
            //moving records to the right so we can make a place for values in the left_records
            for idx in (0..first_removed_idx).rev() {
                //these records don't change location or value so we don't mark them as to be updated
                //they are only at different part in the order vector (behaves like a scroll)
                self.order[idx+left_move_size] = self.order[idx];
            }
        }

        // we copy from the back of the left_records_len, so if it's larger first records on the list are ignored
        if left_records_len > 0 {
            let last_idx_in_left = left_records_len - 1;
            for idx in 0..left_move_size {
                let left_idx = last_idx_in_left - idx;

                let record = left_records[left_idx].clone();
                let id = record.get_id();
                self.order[idx] = id;
                self.data.insert(id, record);
                changeset.add(id);
            }
        }

        //we perform move of values being to the right of removed range 
        let first_free_idx = if right_move_size > 0 && starting_len >= position {            
            // idx: [position, starting_len]
            for idx in position..starting_len {
                let id = self.order[idx];
                self.order[idx - right_move_size] = id;
                changeset.update(id);
            }

            starting_len - right_move_size
        }
        else {
            starting_len
        };

        // WARNING: operation order matters
        //   If page size is set to UNLIMITED then `self.max_size+right_move_size` would overflow
        //   `self.max_size - starting_len` is always valid since `starting_len` is in range [0, self.max_size]
        let free_space = self.max_size - starting_len + right_move_size;

        let max_right_insert_size = min(free_space, right_records_len);
        let max_right_insert_idx = max_right_insert_size + first_free_idx;
        let right_insert_size = if max_right_insert_idx > self.max_size {
            max_right_insert_idx - self.max_size
        } 
        else {
            max_right_insert_size
        };

        if first_free_idx < self.max_size && right_records_len > 0{
            for idx in 0..right_insert_size {
                let record = right_records[idx].clone();
                let id = record.get_id();
                let order_idx = first_free_idx + idx;

                self.data.insert(id, record);
                if order_idx < starting_len {
                    self.order[first_free_idx+idx] = id;
                }
                else {
                    self.order.push(id);
                }

                changeset.add(id);
            }
        }

        // remove values which were not overridden by the right_records
        // since there are not enough values in right records
        if max_right_insert_idx < starting_len {
            let remove_len = starting_len - max_right_insert_idx;
            
            for _ in 0..remove_len {
                //these records were moved away earlier so no changeset updates is required
                self.order.pop();
            }
        }
    }

    /// Returns current length of the container
    pub fn len(&self) -> usize {
        self.order.len()
    }

    /// Updates given record if it exists in the data container
    pub fn update(&mut self, r: Record) {
        let id = r.get_id();
        if self.data.contains_key(&id) {
            self.data.insert(id, r);
        }
    }

    /// Returns `nth` record id as data are ordered
    pub fn get_record_id_at(&self, nth: usize) -> &Id<Record>{
        &self.order[nth]
    }

    /// Returns record for given id
    pub fn get_record(&self, id: &Id<Record>) -> Option<&Record> {
        self.data.get(id)
    }

    /// marks records as removed in changeset
    /// removes record from data
    /// doesn't remove record from order
    #[inline]
    fn mark_removed(&mut self, changeset: &mut WindowChangeset<Record>, range: Range<usize>) {
        for idx in range {
            let id = self.order[idx];
            changeset.remove(id);
            self.data.remove(&id);
        }
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
            if let Some(r) = self.data.get(id) {
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
            if !self.order.contains(id) {
                out_of_order.push(record);
            }
        }

        sf.field("out of order values", &out_of_order);

        sf.finish()
    }
}
