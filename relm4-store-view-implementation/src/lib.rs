//! Create contains implementation of the store view

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

mod implementation;
mod widgets;
mod window_changeset;

pub use implementation::StoreViewImplementation;
pub use window_changeset::WindowChangeset;
