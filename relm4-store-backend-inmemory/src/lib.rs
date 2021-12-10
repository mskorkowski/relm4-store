//! Crate implements in memory data store

// #![warn(
    // missing_debug_implementations,
    // missing_docs,
    // rust_2018_idioms,
    // unreachable_pub
// )]

mod backend;
mod backend_sorted;

pub use backend::InMemoryBackend;
pub use backend::InMemoryBackendConfiguration;

pub use backend_sorted::SortedInMemoryBackend;
pub use backend_sorted::SortedInMemoryBackendConfiguration;
pub use backend_sorted::Sorter;

pub use backend_sorted::OrderedStore;