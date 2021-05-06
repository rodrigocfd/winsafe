//! Native Win32 structs.

#[macro_use]
mod macros;

mod any_newtypes;
mod any_structs;
mod any_structs_cc;
mod guid;

pub use any_newtypes::*;
pub use any_structs::*;
pub use any_structs_cc::*;
pub use guid::{CLSID, GUID, IID};
