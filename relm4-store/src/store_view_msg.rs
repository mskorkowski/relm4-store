use std::fmt::Debug;

use record::Id;
use record::Record;

use super::Position;

/// Messages sent to/between stores
#[derive(Clone,Debug)]
pub enum StoreViewMsg<T: Record> {
    /// New record was added at the given position
    NewAt(Position),
    /// One record in store has been moved
    Move{
        /// Position at which record was
        from: Position,
        /// Position at which record is now
        to: Position,
    },
    /// There is big reorder and in the given region of the store
    Reorder{
        /// beginning of the region reorder
        from: Position,
        /// end of the region of reorder
        to: Position,
    },
    /// Record was removed from the store at given position
    Remove(Position),
    /// Record was committed to the store
    Update(Id<T>),
    /// Store should be reloaded fully, dump all data, indexes, etc... and reload the data
    Reload,
    /// Move the window such that first shown record is at given position
    SlideTo(Position),
}
