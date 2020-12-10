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
//! # Constants
//!
//! The Win32 API, being a C API, has all its constants as simple `#define`
//! macros. This has the drawback of different constant types being allowed to
//! be mixed.
//!
//! WinSafe, in the other hand, defines types for all constants, so they cannot
//! be mixed unless explicitly told to. All constants are defined inside the
//! [`co`](crate::co) module.
//!
//! Most constant types are named according to their prefix. For example,
//! [`MessageBox`](crate::HWND::MessageBox) constants, like `MB_OKCANCEL`,
//! belong to a type called [`MB`](crate::co::MB). These types implement bitwise
//! operator traits, like
//! [`BirOr`](https://doc.rust-lang.org/std/ops/trait.BitOr.html) and
//! [`BitAnd`](https://doc.rust-lang.org/std/ops/trait.BitAnd.html).
//!
//! Typed constants are used in function arguments and also in struct fields.
//! For example, struct [`WNDCLASSEX`](crate::WNDCLASSEX) has a `style` field
//! typed as [`CS`](crate::co::CS), which restricts the possible values.
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