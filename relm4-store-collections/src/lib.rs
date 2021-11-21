//! This crate contains implementation of collections required by the relm4-data-store to operate efficiently
//! 
//! Currently implemented collections:
//! 
//! | Collection | Description |
//! |:-----------|:------------|
//! | [Tree]     | BTree with order statistics |
//! 

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod tree;



pub use tree::Tree;
pub use tree::TreeConfiguration;


// REMOVE FROM EXPORTS
pub use tree::TreeImpl;