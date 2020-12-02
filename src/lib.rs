//! Safe, hand-crafted Win32 API bindings, in idiomatic Rust.

pub mod co;

mod ffi;

mod handles;
pub use handles::*;

mod utf16;
pub use utf16::*;