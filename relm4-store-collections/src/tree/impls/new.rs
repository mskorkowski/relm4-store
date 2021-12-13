use std::mem;

use super::*;

impl<Value, Comparator> TreeImpl<Value, Comparator> {
    /// Creates new instance of TreeImpl
    pub fn new(configuration: TreeConfiguration) -> Self {
        assert!(mem::size_of::<Value>() != 0, "We're not ready to handle ZSTs");

        Self{
            root: NonNull::dangling(),
            count: 0,
            _configuration: configuration,
            _comparator: PhantomData,
            _value: PhantomData,
        }
    }
}