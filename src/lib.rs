//! Win32 GUI and related APIs in safe, idiomatic Rust.
//!
//! WinSafe has:
//!
//! * high-level structs to build native Win32 GUI applications;
//! * low-level Win32 API constants, functions and structs related to GUI.
//!
//! If you're looking for a comprehensive Win32 coverage, take a look at
//! [winapi](https://crates.io/crates/winapi) or
//! [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but
//! have everything.
//!
//! Links:
//! * Crate – [crates.io/crates/winsafe](https://crates.io/crates/winsafe);
//! * GitHub – [github.com/rodrigocfd/winsafe](https://github.com/rodrigocfd/winsafe);
//! * examples – [github.com/rodrigocfd/winsafe-examples](https://github.com/rodrigocfd/winsafe-examples).
//!
//! # Usage
//!
//! Add the dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! winsafe = "0.0.4"
//! ```
//!
//! To enable the DirectShow COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.4", features = ["dshow"] }
//! ```
//!
//! To enable the Shell COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.4", features = ["shell"] }
//! ```
//!
//! # Modules overview
//!
//! The Win32 bindings are divided into a few modules:
//!
//! * root – the root of the crate has Win32 free functions and structs;
//! * [`co`](crate::co) – types and values of Win32 constants;
//! * [`msg`](crate::msg) – window messages;
//! * [`gui`](crate::gui) – high-level GUI wrappers.
//!
//! And additionally:
//!
//! * [`dshow`](crate::dshow) – Win32 DirectShow COM interfaces;
//! * [`shell`](crate::shell) – Win32 Shell COM interfaces.
//!
//! # The GUI API
//!
//! WinSafe features idiomatic bindings for the Win32 API, but on top of that,
//! it features a set of high-level GUI structs, which scaffolds the boilerplate
//! needed to build native Win32 GUI applications, event-oriented. Unless you're
//! doing something really specific, these high-level wrappers are highly
//! recommended – you'll usually start with the
//! [`WindowMain`](crate::gui::WindowMain).
//!
//! One of the greatest strenghts of the GUI API is supporting the use of
//! resource files, which can be created with a WYSIWYG
//! [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).
//!
//! GUI structs can be found in module [`gui`](crate::gui).
//!
//! # Native function calls
//!
//! The best way to understand the idea behind WinSafe bindings is comparing
//! them to the correspondent C code.
//!
//! For example, take the following C code:
//!
//! ```c
//! HWND hwnd = GetDesktopWindow();
//! SetFocus(hwnd);
//! ```
//!
//! This is equivalent to:
//!
//! ```rust,ignore
//! use winsafe::HWND;
//!
//! let hwnd = HWND::GetDesktopWindow();
//! hwnd.SetFocus();
//! ```
//!
//! Note how [`GetDesktopWindow`](crate::HWND::GetDesktopWindow) is a static
//! method of [`HWND`](crate::HWND), and [`SetFocus`](crate::HWND::SetFocus) is
//! an instance method called directly upon `hwnd`. All native handles (`HWND`,
//! [`HDC`](crate::HDC), [`HINSTANCE`](crate::HINSTANCE), etc.) are structs,
//! thus:
//!
//! * native Win32 functions that return a handle are *static methods* in
//! WinSafe;
//! * native Win32 functions whose *first parameter* is a handle are *instance
//! methods*.
//!
//! Now this C code:
//!
//! ```c
//! PostQuitMessage(0);
//! ```
//!
//! Is equivalent to:
//!
//! ```rust,ignore
//! use winsafe::PostQuitMessage;
//!
//! PostQuitMessage(0);
//! ```
//!
//! Since [`PostQuitMessage`](crate::PostQuitMessage) is a free function, it's
//! simply at the root of the crate.
//!
//! # Native constants
//!
//! All native Win32 constants can be found in the [`co`](crate::co) module.
//! They're all *typed*, what means that different constant types cannot be
//! mixed (unless you explicitly say so).
//!
//! Technically, each constant type is simply a
//! [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! with a couple implementations, including those allowing bitflag operations.
//! Also, all constant values can be converted to its underlying
//! [integer type](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types).
//!
//! The name of the constant type is often its prefix. For example, constants of
//! [`MessageBox`](crate::HWND::MessageBox) function, like `MB_OKCANCEL`, belong
//! to a type called [`MB`](crate::co::MB).
//!
//! For example, take the following C code:
//!
//! ```c
//! let hwnd = GetDesktopWindow();
//! MessageBox(hwnd, "Hello, world", "My hello", MB_OKCANCEL | MB_ICONINFORMATION);
//! ```
//!
//! This is equivalent to:
//!
//! ```rust,ignore
//! use winsafe::{co::MB, HWND};
//!
//! let hwnd = HWND::GetDesktopWindow();
//! hwnd.MessageBox("Hello, world", "Title", MB::OKCANCEL | MB::ICONINFORMATION)
//!     .unwrap();
//! ```
//!
//! The method [`MessageBox`](crate::HWND::MessageBox), like all native
//! functions that can return errors, will return
//! [`WinResult`](crate::WinResult), which can contain an
//! [`ERROR`](crate::co::ERROR) constant.
//!
//! # Native structs
//!
//! WinSafe implements native Win32 structs in a very restricted way. First off,
//! fields which control the size of the struct – often named `cbSize` – are
//! *private* and automatically set when the struct is instantiated.
//!
//! Pointer fields are also private, and they can be set and retrieved *only*
//! through getter and setter methods. In particular, when setting a string
//! pointer field, you need to pass a reference to a [`WString`](crate::WString)
//! buffer, which will keep the actual string contents.
//!
//! For example, the following C code:
//!
//! ```c
//! WNDCLASSEX wcx = {0};
//! wcx.cbSize = sizeof(WNDCLASSEX);
//! wcx.lpszClassName = "MY_WINDOW";
//!
//! if (RegisterClassEx(&wcx) == 0) {
//!     DWORD err = GetLastError();
//!     // handle error...
//! }
//! ```
//!
//! Is equivalent to:
//!
//! ```rust,ignore
//! use winsafe::{RegisterClassEx, WNDCLASSEX, WString};
//!
//! let buf = WString::from_str("MY_WINDOW");
//!
//! let mut wcx = WNDCLASSEX::default();
//! wcx.set_lpszClassName(&buf);
//!
//! if let Err(err) = RegisterClassEx(&wcx) {
//!     // handle error...
//! }
//! ```
//!
//! Note how you *don't need* to call [`GetLastError`](crate::GetLastError) to
//! retrieve the error code: it's returned by the method itself in the
//! [`WinResult`](crate::WinResult).
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

#[macro_use]
pub mod co;

#[macro_use]
mod structs;

mod ffi;
mod privs;

pub mod gui;
pub mod msg;

mod aliases;
mod com;
mod enums;
mod funcs;
mod handles;
mod w_string;

pub use aliases::*;
pub use com::*;
pub use enums::*;
pub use funcs::*;
pub use handles::*;
pub use structs::*;
pub use w_string::WString;
