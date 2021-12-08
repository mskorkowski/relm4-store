use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

use record::Record;
use record::TemporaryIdAllocator;

use super::Position;

/// Passes information about record and it's order in store
#[derive(Debug)]
pub struct RecordWithLocation<T, Allocator>
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{
    /// Position at which record is placed in the store
    /// 
    /// Defines order of records
    pub position: Position,

    /// Copy of record in the store
    pub record: T,

    _allocator: PhantomData<*const Allocator>,
}

impl<T, Allocator> Deref for RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{
    type Target = T;

    /// Auto dereferences RecordWithLocation to `T`
    fn deref(&self) -> &T {
        &self.record
    }
}

impl<T, Allocator> RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{
    /// Creates new instance of RecordWithLocation
    pub fn new(position: Position, record: T) -> Self {
        RecordWithLocation{
            position,
            record,
            _allocator: PhantomData,
        }
    }
}

impl<T, Allocator> PartialOrd for RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{
    /// Records have a natural order by the position
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl<T, Allocator> PartialEq for RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator,
{
    /// Two records are equal if their id's are equal
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl<T, Allocator> Eq for RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{}

impl<T, Allocator> Ord for RecordWithLocation<T, Allocator> 
where 
    T: Record<Allocator> + Clone + Debug,
    Allocator: TemporaryIdAllocator
{

    /// Records have a natural order by the position
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}