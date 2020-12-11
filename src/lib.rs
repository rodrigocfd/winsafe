//! Safe, hand-crafted
//! [Win32 API](https://docs.microsoft.com/en-us/windows/win32/) bindings, in
//! idiomatic Rust.
//!
//! This crate intends to cover the most important parts of the Windows API, but
//! due to its hugeness, it will probably remain as a work-in-progress for a
//! while.
//!
//! # Functions
//!
//! WinSafe categorizes Win32 API functions in three types:
//!
//! * free functions;
//! * static methods;
//! * instance methods.
//!
//! Free functions, like [`PostQuitMessage`](crate::PostQuitMessage), are those
//! found at the root of the crate.
//!
//! Both static and instance methods belong to handle types, like
//! [`HDC`](crate::HDC) or [`HWND`](crate::HWND). Handle types always start with
//! the letter "H", and they are opaque pointers provided by the Win32 API which
//! we can call functions upon.
//!
//! Static methods create new handle objects, whereas instance methods perform
//! actions on existing handle objects.
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
//! # Errors
//!
//! [Win32 errors](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes),
//! usually returned by [`GetLastError`](crate::GetLastError) function, belong
//! to the constant type [`ERROR`](crate::co::ERROR), which also holds
//! [`HRESULT`](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)
//! values.
//!
//! Most functions return a
//! [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) with a
//! possible `ERROR`, but some functions simply return an empty error value.
//! This happens because many Win32 functions can fail without providing any
//! error information.
//!
//! You can retrieve the textual description of an `ERROR` by calling its
//! [`FormatMessage`](crate::co::ERROR::FormatMessage) instance method. Also,
//! `ERROR` implements `From<u32>` trait, which allows you to retrieve the
//! numeric error code.
//!
//! No WinSafe function will panic.
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

#[macro_use]
mod internal_defs;

mod aliases;
mod com;
mod enums;
mod funcs;
mod handles;
mod structs;
mod utf16;

pub use aliases::*;
pub use com::*;
pub use enums::*;
pub use funcs::*;
pub use handles::*;
pub use structs::*;
pub use utf16::*;