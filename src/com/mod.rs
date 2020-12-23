//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

#[macro_use]
mod macros;

pub mod shell;

mod iunknown;
mod vtbl;

pub use iunknown::{IUnknown, IUnknownVtbl};
pub use vtbl::{PPVtbl, Vtbl};