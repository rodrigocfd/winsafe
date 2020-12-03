//! Raw FFI bindings.

/// An empty enum, used to represent void pointers.
pub enum Void {}

pub mod kernel32;
pub mod ole32;
pub mod user32;