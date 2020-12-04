//! Safe, hand-crafted Win32 API bindings, in idiomatic Rust.

pub mod co;

pub mod com;

mod ffi;

mod funcs;
pub use funcs::*;

mod handles;
pub use handles::*;

mod structs;
pub use structs::*;

mod utf16;
pub use utf16::*;