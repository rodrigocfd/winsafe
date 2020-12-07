//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

pub mod shell;

mod cominterface;
mod iunknown;

pub use cominterface::*;
pub use iunknown::*;