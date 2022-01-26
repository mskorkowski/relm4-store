//! This crate contains implementation of collections required by the relm4-data-store to operate efficiently
//! 
//! Currently implemented collections:
//! 
//! | Collection | Description | Status |
//! |:-----------|:------------|:-------|
//! | [data_container::DataContainer] | Collection which can be repurposed to implement custom [store::StoreViews]. | Complete |
//! | [Tree]     | BTree with order statistics | Work in progress, not exported yet |
//! 

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod data_container;

// mod tree;

// pub use tree::Tree;
// pub use tree::TreeConfiguration;


// // REMOVE FROM EXPORTS
// pub use tree::TreeImpl;