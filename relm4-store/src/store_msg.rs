use std::fmt::Debug;

use record::Id;
use record::Record;

/// Messages sent to/between stores
#[derive(Clone,Debug)]
pub enum StoreMsg<T: Record> {
    /// Record was committed to the store
    Commit(T),
    /// Record was updates in the store
    Update(Id<T>),
    /// Removes record from the store
    Delete(Id<T>),
    /// Store should be reloaded fully, dump all data, indexes, etc... and reload the data
    Reload,
}
