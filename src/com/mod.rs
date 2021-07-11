//! COM interfaces.

#[macro_use] mod macros;

#[macro_use] mod iunknown; // 1st level interface

#[macro_use] mod idispatch; // 2nd level interfaces
#[macro_use] mod ipersist;
#[macro_use] mod itypeinfo;

#[cfg(feature = "dshow")]
pub mod dshow;
#[cfg(feature = "shell")]
pub mod shell;

mod funcs;
mod traits;

pub use funcs::*;
pub use idispatch::{IDispatch, IDispatchVT};
pub use ipersist::{IPersist, IPersistVT};
pub use itypeinfo::{ITypeInfo, ITypeInfoVT};
pub use iunknown::{IUnknown, IUnknownVT};
pub use traits::*;
