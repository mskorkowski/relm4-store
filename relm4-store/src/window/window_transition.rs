/// Describe transitions in relation to range of items displayed by window
#[derive(Debug)]
pub enum WindowTransition {
    /// Nothing to do
    Identity,
    /// Insert records and change indexes to right of new items
    ///
    /// Starting from `pos`, insert `by` records, change indexes of items from `pos` by `idx + by`, drop last `by` records
    InsertRight{
        /// position in the store
        pos: usize,
        /// how many records
        by:  usize,
    },

    /// Insert records and change indexes to left of new items
    ///
    /// Drop first `by` records, change indexes of items from first to `pos` by `idx - by`, insert `by` records
    InsertLeft{
        /// position in the store
        pos: usize,
        /// how many records
        by: usize,
    },

    /// Remove records and change indexes to right of new items
    ///
    /// Starting from `pos`, remove `by` records, change indexes of items from `pos` by `idx - by`, insert last `by` records
    RemoveRight{
        /// position in the store
        pos: usize,
        /// how many elements
        by:  usize,
    },

    /// Remove records and change indexes to left of new items
    ///
    /// Change indexes of items from first to `pos` by `idx - by`. Insert at the beginning `by` records.
    /// From pos remove `by` records
    RemoveLeft{
        /// position in the store
        pos: usize,
        /// how many elements
        by: usize,
    },

    /// Move window to the right. No data changes
    SlideRight(usize),

    /// Move window to the left. No data changes
    SlideLeft(usize),

}