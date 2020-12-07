//! Safe, hand-crafted Win32 API bindings, in idiomatic Rust.
//!
//! Example:
//! ```rust,ignore
//! use winsafe::co;
//! use winsafe::HWND;
//!
//! fn main() {
//!   HWND::default().MessageBox("Hello, world", "Title",
//!     co::MB::OKCANCEL | co::MB::ICONINFORMATION).unwrap();
//! }
//! ```
//!
//! This crate intends to cover the most important parts of the Windows API, but
//! due to its hugeness, it will probably remain as a work-in-progress for a
//! while.

#[macro_use]
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