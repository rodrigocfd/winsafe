//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

pub mod shell;

mod iunknown;
pub use iunknown::*;