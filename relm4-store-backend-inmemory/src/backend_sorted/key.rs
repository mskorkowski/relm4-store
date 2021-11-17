use std::marker::PhantomData;


use super::Sorter;

#[derive(Debug)]
pub struct Key<'me, Record: record::Record, OrderBy: Sorter<Record>> {
    value: &'me Record,
    order_by: PhantomData<&'me OrderBy>,
}

impl<'me, Record: record::Record, OrderBy: Sorter<Record>> Key<'me, Record, OrderBy> {
    pub fn new<'a>(value: &'a Record) -> Key<'a, Record, OrderBy> {
        Key{
            value,
            order_by: PhantomData,
        }
    }
}

impl<'me, Record: record::Record, OrderBy: Sorter<Record>> PartialEq for Key<'me, Record, OrderBy> {
    fn eq(&self, other: &Self) -> bool {
        OrderBy::eq(self.value, other.value)
    }
}

impl<'me, Record: record::Record, OrderBy: Sorter<Record>> PartialOrd for Key<'me, Record, OrderBy> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(OrderBy::cmp(&self.value, other.value))
    }
}

impl<'me, Record: record::Record, OrderBy: Sorter<Record>> Eq for Key<'me, Record, OrderBy> {}

impl<'me, Record: record::Record, OrderBy: Sorter<Record>> Ord for Key<'me, Record, OrderBy> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderBy::cmp(&self.value, other.value)
    }
}