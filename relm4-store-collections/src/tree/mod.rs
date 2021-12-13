//! Implementation of B-tree with order statistics
//! 
//! ## Rationale for custom implementation
//! 
//! In [std::collections::BTree] every operation is expressed in terms of key which implements
//! total order. This is not efficient or useful when you need to deal with user interface.
//!
//! For example, if you have a long list of todo tasks looking like
//! 
//! ```rust
//! struct Task {
//!   description: String,
//! }
//! ```
//! 
//! And you would like to perform pagination (show first 50, 2nd 50, 3rd 50). Your list of tasks
//! is ordered by description.
//! 
//! ### Showing first 50 elements
//! 
//! To show first 50 elements, you can create an iterator from a tree and use first 50 elements only
//! 
//! ### Showing next page of elements
//! 
//! To show next page, you can take a range starting from last element of the page (exclusive) and
//! iterate next 50 elements
//! 
//! ### Showing previous page of elements
//! 
//! You can take a range ending (exclusive) at the first element of current page. But now you need
//! to iterate from the smallest element in the BTree to the first element of the current page and
//! build up a buffer of up to page of elements. If you are at the end of long list of tasks it's
//! becomes O(n) problem => slow
//! 
//! ### Showing arbitrary page of elements
//! 
//! You must iterate from start keeping a buffer of up to page elements to show => slow
//! 
//! ## Why standard crate is slow in ui use case?
//! 
//! The BTree in standard crate requires Ord but to perform fast in terms of the pagination you know
//! the distance between existing elements in the tree. That is in itself equivalent of keeping the
//! data in the continuos vector. If I can keep data in the vector, why would I use something so
//! complicated as a tree?
//! 
//! Let's go back to our `Task` example. Let's assume our tree is ordered by the description.
//! You must show to the user a page of the data (for example 50 tasks). 
//! 
//! 1. Description is the text.
//! 2. Description can be arbitrarily long.
//! 3. For simplicity case I assume that only small letters are allowed in description (no spaces, 
//! tabs, etc..). This simplification won't affect the conclusion. It will make some examples easier
//! to show.
//! 
//! So let's compute distance between some examples
//! 
//! * `dist("a", "a") = 0`, by definition
//! * `dist("a", "aa") = 1`, there is nothing which can go between `a` and `aa` in lexicographical
//! order
//! * `dist("a", "b") = ∞`, there is infinite number of texts starting with letter `a`
//! 
//! This last property is reason why we use a tree to keep data and also one of the reasons why
//! BTree implementation doesn't work for ui. To be exact [`std::collections::BTree::range`]
//! function is useless if you can't predict distance between elements if you need to provide
//! page of data.
//! 
//! ## Differences between [std::collections::BTree] and [Tree]
//! 
//! TODO: Write list of differences between this types both internal and usage

mod configuration;
mod impls;

#[cfg(test)]
mod test;

use std::marker::PhantomData;

pub use configuration::TreeConfiguration;

pub use impls::TreeImpl;

/// Implementation of B-tree with order statistics
/// 
/// For more details check [crate::tree] description
#[derive(Debug)]
pub struct Tree<K, V> 
where
K: 'static,
V: 'static,
{
    _marker: PhantomData<(K, V)>,
    // // root: Option<Node<K,V>>,
    // configuration: TreeConfiguration,
    // size: usize,
}

// impl<K, V> Tree<K, V> 
// where
//     K: 'static,
//     V: 'static,
// {
//     pub fn new(configuration: TreeConfiguration) -> Tree<K,V> {
//         Self{
//             _marker: PhantomData,
//             // root: None,
//             configuration,
//             size: 0,
//         }
//     }

//     pub fn clear(&mut self) {
//         *self = Tree::new(self.configuration)
//     }

//     pub fn insert(&mut self, key: K, value: V) -> Option<V> {
//         None
//     }

//     pub fn len(&self) -> usize {
//         self.size
//     } 

//     pub fn is_empty(&self) -> bool {
//         self.size == 0
//     }
// }

// impl<K, V> Default for Tree<K, V>
// where
//     K: 'static,
//     V: 'static,
// {
//     fn default() -> Self {
//         Tree::new(TreeConfiguration::default())
//     }
// }

// enum Node<K, V> 
// where
//     V: 'static,
//     K: 'static,
// {
//     Data(&'static DataNode<K, V>),
//     Inner(&'static InnerNode<K,V>)
// }

// /// Node in the tree
// struct InnerNode<K, V> 
// where
//     V: 'static,
//     K: 'static,
// {
//     /// Size of the whole subtree
//     size: usize,
//     data: Vec<(K, V)>,
//     link: Vec<Node<K,V>>,
// }

// struct DataNode<K, V> 
// where
//     K: 'static,
//     V: 'static,
// {
//     data: Vec<(K, V)>,
//     next: &'static DataNode<K,V>,
// }

