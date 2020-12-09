//! Safe, hand-crafted
//! [Win32 API](https://docs.microsoft.com/en-us/windows/win32/) bindings, in
//! idiomatic Rust.
//!
//! This crate intends to cover the most important parts of the Windows API, but
//! due to its hugeness, it will probably remain as a work-in-progress for a
//! while.
//!
//! # Examples
//!
//! A message box "hello world":
//! ```rust,ignore
//! use winsafe::{co, HWND};
//!
//! fn main() {
//!   HWND::default().MessageBox("Hello, world", "Title",
//!     co::MB::OKCANCEL | co::MB::ICONINFORMATION).unwrap();
//! }
//! ```

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