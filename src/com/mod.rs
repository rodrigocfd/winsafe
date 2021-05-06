//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

#[macro_use] mod macros;

#[macro_use] mod iunknown;
#[macro_use] mod idispatch;
#[macro_use] mod ipersist;

pub mod dshow;
pub mod shell;

mod comvt;
mod funcs;

pub use comvt::{ComVT, PPComVT};
pub use funcs::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize};
pub use idispatch::{IDispatch, IDispatchVT};
pub use ipersist::{IPersist, IPersistVT};
pub use iunknown::{IUnknown, IUnknownVT};
