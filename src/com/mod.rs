//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

pub mod shell;

mod comvtbl;
mod iunknown;

pub use comvtbl::*;
pub use iunknown::*;