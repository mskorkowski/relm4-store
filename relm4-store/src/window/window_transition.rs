/// Describe transitions in relation to range of items displayed by window
pub enum WindowTransition {
    /// Nothing to do
    Identity,
    /// Insert elements and change indexes to right of new items
    ///
    /// Starting from `pos`, insert `by` elements, change indexes of items from `pos` by `idx + by`, drop last `by` elements
    InsertRight{
        pos: usize,
        by:  usize,
    },

    /// Insert elements and change indexes to left of new items
    ///
    /// Drop first `by` elements, change indexes of items from first to `pos` by `idx - by`, insert `by` elements
    InsertLeft{
        pos: usize,
        by: usize,
    },

    /// Remove elements and change indexes to right of new items
    ///
    /// Starting from `pos`, remove `by` elements, change indexes of items from `pos` by `idx - by`, insert last `by` elements
    RemoveRight{
        pos: usize,
        by:  usize,
    },

    /// Remove elements and change indexes to left of new items
    ///
    /// Change indexes of items from first to `pos` by `idx - by`. Insert at the beginning `by` elements.
    /// From pos remove `by` elements
    RemoveLeft{
        pos: usize,
        by: usize,
    },

    /// Move window to the right. No data changes
    SlideRight(usize),

    /// Move window to the left. No data changes
    SlideLeft(usize),

}