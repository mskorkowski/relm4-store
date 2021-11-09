use std::fmt::Debug;

use record::Id;
use record::Record;

use super::Position;

#[derive(Clone,Debug)]
pub enum StoreMsg<T: Record + Debug + Clone> {
    New(T),
    NewAt(Position),
    /// One element in store has been moved
    Move{
        from: Position,
        to: Position,
    },
    Reorder{
        from: Position,
        to: Position,
    },
    Remove(Position),
    Commit(T),
    Update(Id<T>),
    Reload,
}