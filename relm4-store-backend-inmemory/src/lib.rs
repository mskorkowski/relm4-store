//! Crate implements in memory data store

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod backend;

pub use backend::InMemoryBackend;
pub use backend::InMemoryBackendConfiguration;