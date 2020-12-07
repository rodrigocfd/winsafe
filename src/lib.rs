//! Safe, hand-crafted Win32 API bindings, in idiomatic Rust.

pub mod co;

mod ffi;

mod com;
mod funcs;
mod handles;
mod structs;
mod utf16;

pub use com::*;
pub use funcs::*;
pub use handles::*;
pub use structs::*;
pub use utf16::*;