//! COM interfaces.
//!
//! Since Rust, by design, doesn't support inheritance, the interfaces use
//! composition with its ancestors.

#[macro_use] mod macros;

#[macro_use] mod iunknown;

#[macro_use] mod idispatch;
#[macro_use] mod ipersist;
#[macro_use] mod itypeinfo;

pub mod dshow;
pub mod shell;

mod funcs;
mod vt;

pub use funcs::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize};
pub use idispatch::IDispatch;
pub use ipersist::IPersist;
pub use itypeinfo::ITypeInfo;
pub use iunknown::IUnknown;
pub use vt::*;
