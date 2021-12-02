//! Native Win32 structs.

#[macro_use] mod macros;

mod guid;
mod newtypes;
mod structs_cc;
mod structs_ord;

pub use guid::{CLSID, GUID, IID};
pub use newtypes::*;
pub use structs_cc::*;
pub use structs_ord::*;
