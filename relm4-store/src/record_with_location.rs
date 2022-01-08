use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::Deref;

use record::Record;

use super::Position;

/// Passes information about record and it's order in store
pub struct RecordWithLocation<T>
where 
    T: Record,
{
    /// Position at which record is placed in the store
    /// 
    /// Defines order of records
    pub position: Position,

    /// Copy of record in the store
    pub record: T,
}

impl<T> Deref for RecordWithLocation<T> 
where 
    T: Record,
{
    type Target = T;

    /// Auto dereferences RecordWithLocation to `T`
    fn deref(&self) -> &T {
        &self.record
    }
}

impl<T> RecordWithLocation<T> 
where 
    T: Record,
{
    /// Creates new instance of RecordWithLocation
    pub fn new(position: Position, record: T) -> Self {
        RecordWithLocation{
            position,
            record,
        }
    }
}

impl<T> PartialOrd for RecordWithLocation<T> 
where 
    T: Record,
{
    /// Records have a natural order by the position
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl<T> PartialEq for RecordWithLocation<T> 
where 
    T: Record,
{
    /// Two records are equal if their id's are equal
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<T> Eq for RecordWithLocation<T> 
where 
    T: Record,
{}

impl<T> Ord for RecordWithLocation<T> 
where 
    T: Record,
{

    /// Records have a natural order by the position
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl<T> Debug for RecordWithLocation<T> 
where
    T: Record + Debug 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecordWithLocation")
            .field("position", &self.position)
            .field("record", &self.record)
            .finish()
    }
}