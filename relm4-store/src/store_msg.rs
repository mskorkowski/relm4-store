use std::fmt::Debug;

use record::Id;
use record::Record;
use record::TemporaryIdAllocator;

use super::Position;

/// Messages sent to/between stores
#[derive(Clone,Debug)]
pub enum StoreMsg<T: Record<Allocator> + Debug + Clone, Allocator: TemporaryIdAllocator + Clone> {
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
    Commit(T),
    /// Record was updates in the store
    Update(Id<T, Allocator>),
    /// Store should be reloaded fully, dump all data, indexes, etc... and reload the data
    Reload,
}
