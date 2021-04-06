//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

#[macro_use]
mod macros;

pub mod shell;

mod comvt;
mod funcs;
mod iunknown;

pub use comvt::{ComVT, PPComVT};
pub use funcs::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize};
pub use iunknown::{IUnknown, IUnknownVT};
