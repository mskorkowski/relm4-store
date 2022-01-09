//! Drop logic for the tree
use std::alloc::dealloc;
use std::alloc::Layout;

use super::*;

impl<Value, Comparator> Drop for TreeImpl<Value, Comparator> {
    fn drop(&mut self) {
        if self.count > 0 {
            let layout = Layout::new::<Node<Value,Comparator>>();
            unsafe {
                self.root.as_mut().clean_internal(layout);
                dealloc(self.root.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<Value, Comparator> Node<Value, Comparator> {
    fn clean_internal(&mut self, layout: Layout) {
        while self.elems.pop_front().is_some() {}
        while let Some(mut kid) = self.kids.pop_front() {
            unsafe {
                kid.node.as_mut().clean_internal(layout);
                dealloc(kid.node.as_ptr() as *mut u8, layout)
            }
        }
    }
}

impl<Value, Comparator> Drop for Node<Value, Comparator> {
    fn drop(&mut self) {
        let layout = Layout::new::<Node<Value,Comparator>>();
        self.clean_internal(layout)
    }
}