//! Counting for the tree

use super::*;


impl<Value, Comparator> TreeImpl<Value, Comparator> {
    /// Returns number of elements in this tree
    pub fn len(&self) -> usize {
        self.count
    }

    /// Alias to len for ease of writing unsafe code
    /// 
    /// Will be removed in near future
    #[deprecated]
    pub fn count(&self) -> usize {
        self.len()
    }

    /// Returns true if this tree doesn't contain any data
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    } 
}

impl<Value, Comparator> Node<Value, Comparator> {
    // Returns number of elements in this subtree
    // pub fn len(&self) -> usize {
    //     let mut count: usize = 0;

    //     for c in &self.kids {
    //         count += c.count
    //     }
        
    //     count += self.elems.len();

    //     return count
    // }
    
    // /// Alias to len for ease of writing unsafe code
    // /// 
    // /// Will be removed in near future
    // #[deprecated]    
    // pub fn count(&self) -> usize {
    //     self.len()
    // }

    // /// Returns true if this sub tree is empty
    // pub fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }
}