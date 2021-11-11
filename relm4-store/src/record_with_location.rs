use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::ops::Deref;

use record::Record;

use super::Position;

/// Passes information about record and it's order in store
#[derive(Debug)]
pub struct RecordWithLocation<T>
where T: Record + Clone + Debug
{
    /// Position at which record is placed in the store
    /// 
    /// Defines order of records
    pub position: Position,

    /// Copy of record in the store
    pub record: T,
}

impl<T> Deref for RecordWithLocation<T> 
where T: Record + Clone + Debug
{
    type Target = T;

    /// Auto dereferences RecordWithLocation to `T`
    fn deref(&self) -> &T {
        &self.record
    }
}

impl<T> RecordWithLocation<T> 
where T: Record + Clone + Debug
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
where T: Record + Clone + Debug
{
    /// Records have a natural order by the position
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl<T> PartialEq for RecordWithLocation<T> 
where T: Record + Clone + Debug
{
    /// Two records are equal if their id's are equal
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<T> Eq for RecordWithLocation<T> 
where T: Record + Clone + Debug
{}

impl<T> Ord for RecordWithLocation<T> 
where T: Record + Clone + Debug
{

    /// Records have a natural order by the position
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}