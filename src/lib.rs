//! Safe, hand-crafted
//! [Win32 API](https://docs.microsoft.com/en-us/windows/win32/) bindings, in
//! idiomatic Rust.
//!
//! This crate provides bindings for Win32 functions, structs and constants. In
//! addition, it provides high-level GUI wrappers for windows and controls,
//! scaffolding the boilerplate.
//!
//! This crate intends to cover the most important parts of the Windows API, but
//! not all of it (well, maybe one day) because the whole API is too huge.
//!
//! # Functions
//!
//! WinSafe exposes native Win32 functions using the same original name, so it
//! should look familiar to anyone who knows Win32. The functions can be
//! categorized in three types:
//!
//! * free functions;
//! * static methods;
//! * instance methods.
//!
//! Free functions are those found at the root of the crate:
//!
//! ```rust,ignore
//! use winsafe::PostQuitMessage;
//!
//! PostQuitMessage(0);
//! ```
//!
//! Both static and instance methods belong to handle types, like
//! [`HDC`](crate::HDC) or [`HWND`](crate::HWND). Handle types always start with
//! the letter "H". Static methods create new handle objects, while instance
//! methods perform operations on existing handle objects.
//!
//! Take the following C code:
//!
//! ```c
//! HWND h = GetDesktopWindow();
//! SetFocus(h);
//! ```
//!
//! This is the equivalent of:
//!
//! ```rust,ignore
//! use winsafe::HWND;
//!
//! let h = HWND::GetDesktopWindow();
//! h.SetFocus();
//! ```
//!
//! # Structs
//!
//! WinSafe structs are internally marked with `#[repr(C)]`. They all implement
//! `Default` trait, and size fields like `cbSize` are private and automatically
//! initialized.
//!
//! String pointer fields are also private, and can be retrieved using a getter
//! method with the same field name. They can be set through a setter method,
//! which often requires a buddy [`WString`](crate::WString) buffer:
//!
//! ```rust,ignore
//! use winsafe::{WNDCLASSEX, WString};
//!
//! let mut wcx = WNDCLASSEX::default(); // cbSize automatically set
//! println!("Class name: {}", wcx.lpszClassName()); // initially an empty string
//!
//! let buf = WString::from_str("CLASS_NAME");
//! wcx.set_lpszClassName(&buf); // set string pointer field
//! ```
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
//! [`BitAnd`](https://doc.rust-lang.org/std/ops/trait.BitAnd.html). They also
//! implement [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
//! trait conversions for the underlying integer type – usually `u32`, but it
//! varies.
//!
//! Typed constants are used in function arguments and also in struct fields.
//! For example, struct [`WNDCLASSEX`](crate::WNDCLASSEX) has a `style` field
//! typed as [`CS`](crate::co::CS), which restricts the possible values.
//!
//! A message box "hello world" in C:
//!
//! ```c
//! HWND h = GetDesktopWindow();
//! MessageBox(h, L"Hello, world", L"Title", MB_OKCANCEL | MB_ICONINFORMATION);
//! ```
//!
//! Is equivalent to:
//!
//! ```rust,ignore
//! use winsafe::{co::MB, HWND};
//!
//! let h = HWND::GetDesktopWindow();
//! h.MessageBox("Hello, world", "Title", MB::OKCANCEL | MB::ICONINFORMATION)
//!   .unwrap();
//! ```
//!
//! # Errors
//!
//! [Win32 errors](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes),
//! natively returned by [`GetLastError`](crate::GetLastError) function, belong
//! to the constant type [`ERROR`](crate::co::ERROR), which also holds
//! [`HRESULT`](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a)
//! values. Most Win32 functions return a [`WinResult`](crate::WinResult) with a
//! possible `ERROR`.
//!
//! Panics will happen only in case of misuse or internal bug.
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
//! However, there are cases where a string conversion is still needed, like
//! when dealing with native Win32 structs. In such cases, you can use the
//! [`WString`](crate::WString) struct, which is also capable of working as a
//! buffer to receive text from Win32 calls.
//!
//! # High-level GUI
//!
//! On top of all Win32 FFI bindings, WinSafe features a set of high-level GUI
//! structs, which scaffolds the boilerplate needed to build windows and
//! controls.
//!
//! Unless you are writing something really specific, these high-level
//! abstractions is highly recommended. They can be found in the module
//! [`gui`](crate::gui).

#[macro_use]
pub mod co;

mod ffi;
mod privs;

pub mod gui;
pub mod msg;

mod aliases;
mod com;
mod enums;
mod funcs;
mod handles;
mod structs;
mod w_string;

pub use aliases::*;
pub use com::*;
pub use enums::*;
pub use funcs::*;
pub use handles::*;
pub use structs::*;
pub use w_string::WString;
