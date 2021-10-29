use std::fmt::Debug;

use model::Id;
use model::Model;

use super::Position;

#[derive(Clone,Debug)]
pub enum StoreMsg<T: Model + Debug + Clone> {
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