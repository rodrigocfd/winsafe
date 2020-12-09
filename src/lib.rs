//! Safe, hand-crafted
//! [Win32 API](https://docs.microsoft.com/en-us/windows/win32/) bindings, in
//! idiomatic Rust.
//!
//! This crate intends to cover the most important parts of the Windows API, but
//! due to its hugeness, it will probably remain as a work-in-progress for a
//! while.
//!
//! # Text encoding
//!
//! Windows natively uses
//! [Unicode UTF-16](https://docs.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings).
//!
//! WinSafe uses Unicode UTF-16 internally but exposes idiomatic UTF-8,
//! performing conversions automatically when needed, so you don't have to worry
//! about [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html)
//! or any low-level conversion.
//!
//! However, if you still need any kind of string conversion, you can use the
//! [`Utf16`](crate::Utf16) struct, which is also capable of working as a buffer
//! to receive text from Win32 calls.
//!
//! # Examples
//!
//! A message box "hello world":
//! ```rust,ignore
//! use winsafe::{co, HWND};
//!
//! fn main() {
//!   HWND::GetDesktopWindow()
//!     .MessageBox("Hello, world", "Title", co::MB::OKCANCEL | co::MB::ICONINFORMATION)
//!     .unwrap();
//! }
//! ```

#[macro_use]
pub mod co;

mod ffi;

mod com;
mod enums;
mod funcs;
mod handles;
mod structs;
mod utf16;

pub use com::*;
pub use enums::*;
pub use funcs::*;
pub use handles::*;
pub use structs::*;
pub use utf16::*;