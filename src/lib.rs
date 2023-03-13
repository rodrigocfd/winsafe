//! Windows API and GUI in safe, idiomatic Rust.
//!
//! [Crate](https://crates.io/crates/winsafe) •
//! [GitHub](https://github.com/rodrigocfd/winsafe) •
//! [Docs (stable)](https://docs.rs/winsafe/) •
//! [Docs (master branch)](https://rodrigocfd.github.io/winsafe/winsafe/) •
//! [Examples](https://github.com/rodrigocfd/winsafe-examples)
//!
//! WinSafe has:
//!
//! * low-level Win32 API constants, functions and structs;
//! * high-level structs to build native Win32 GUI applications.
//!
//! If you're looking for a comprehensive Win32 coverage, take a look at
//! [winapi](https://crates.io/crates/winapi) or
//! [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but
//! have everything.
//!
//! # Usage
//!
//! Add the dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.14", features = [] }
//! ```
//!
//! Then you must enable the
//! [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
//! you want to be included – these modules are named after native Windows DLL
//! and library names, mostly.
//!
//! The following Cargo features are available so far:
//!
//! | Feature | Description |
//! | - | - |
//! | `comctl` | ComCtl32.dll, for [Common Controls](https://learn.microsoft.com/en-us/windows/win32/api/_controls/) |
//! | `comdlg` | ComDlg32.dll, for the old [Common Dialogs](https://learn.microsoft.com/en-us/windows/win32/uxguide/win-common-dlg) |
//! | `dshow` | [DirectShow](https://learn.microsoft.com/en-us/windows/win32/directshow/directshow) |
//! | `dxgi` | [DirectX Graphics Infrastructure](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi) |
//! | `gdi` | Gdi32.dll, the [Windows GDI](https://learn.microsoft.com/en-us/windows/win32/gdi/windows-gdi) |
//! | **`gui`** | **The WinSafe high-level GUI abstractions** |
//! | `kernel` | Kernel32.dll, Advapi32.dll and Ktmw32.dll – all others will include it |
//! | `msimg` | Msimg32.dll |
//! | `ole` | OLE and basic COM support |
//! | `oleaut` | [OLE Automation](https://learn.microsoft.com/en-us/windows/win32/api/_automat/) |
//! | `shell` | Shell32.dll and Shlwapi.dll, the COM-based [Windows Shell](https://learn.microsoft.com/en-us/windows/win32/shell/shell-entry) |
//! | `user` | User32.dll, the basic Windows GUI support |
//! | `uxtheme` | UxTheme.dll, extended window theming |
//! | `version` | Version.dll, to manipulate *.exe version info |
//!
//! Note that a Cargo feature may depend on other features, which will be
//! enabled automatically.
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
//! use winsafe::prelude::*;
//! use winsafe::HWND;
//!
//! let hwnd = HWND::GetDesktopWindow();
//! hwnd.SetFocus();
//! ```
//!
//! Note how [`GetDesktopWindow`](crate::prelude::user_Hwnd::GetDesktopWindow)
//! is a static method of [`HWND`](crate::HWND), and
//! [`SetFocus`](crate::prelude::user_Hwnd::SetFocus) is an instance method
//! called directly upon `hwnd`. All native handles (`HWND`,
//! [`HDC`](crate::HDC), [`HINSTANCE`](crate::HINSTANCE), etc.) are structs,
//! thus:
//!
//! * native Win32 functions that return a handle are *static methods* in WinSafe;
//! * native Win32 functions whose *first parameter* is a handle are *instance methods*.
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
//! use winsafe::prelude::*;
//! use winsafe::PostQuitMessage;
//!
//! PostQuitMessage(0);
//! ```
//!
//! Since [`PostQuitMessage`](crate::PostQuitMessage) is a free function, it's
//! simply at the root of the crate.
//!
//! Also note that some functions which require a cleanup routine – like
//! [`BeginPaint`](crate::prelude::user_Hwnd::BeginPaint), for example – will
//! return the resource wrapped in a [guard](crate::guard), which will perform
//! the cleanup automatically. You'll never have to manually call
//! [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint).
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
//! [`MessageBox`](crate::prelude::user_Hwnd::MessageBox) function, like
//! `MB_OKCANCEL`, belong to a type called [`MB`](crate::co::MB).
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
//! use winsafe::prelude::*;
//! use winsafe::{co::MB, HWND};
//!
//! let hwnd = HWND::GetDesktopWindow();
//! hwnd.MessageBox("Hello, world", "Title", MB::OKCANCEL | MB::ICONINFORMATION)?;
//! # Ok::<_, winsafe::co::ERROR>(())
//! ```
//!
//! The method [`MessageBox`](crate::prelude::user_Hwnd::MessageBox), like most
//! functions that can return errors, will return
//! [`SysResult`](crate::SysResult), which can contain an
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
//! use winsafe::prelude::*;
//! use winsafe::{RegisterClassEx, WNDCLASSEX, WString};
//!
//! let mut wcx = WNDCLASSEX::default();
//!
//! let mut buf = WString::from_str("MY_WINDOW");
//! wcx.set_lpszClassName(Some(&mut buf));
//!
//! if let Err(err) = RegisterClassEx(&wcx) {
//!     // handle error...
//! }
//! ```
//!
//! Note how you *don't need* to call [`GetLastError`](crate::GetLastError) to
//! retrieve the error code: it's returned by the method itself in the
//! [`SysResult`](crate::SysResult).
//!
//! # Text encoding
//!
//! Windows natively uses
//! [Unicode UTF-16](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings).
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
//! # Errors and result aliases
//!
//! WinSafe declares a few
//! [`Result` aliases](https://doc.rust-lang.org/rust-by-example/error/result/result_alias.html)
//! which are returned by its functions and methods:
//!
//! | Alias | Error | Used for |
//! | - | - | - |
//! | [`SysResult`](crate::SysResult) | [`ERROR`](crate::co::ERROR) | Standard [system errors](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes). |
//! | [`HrResult`](crate::HrResult) | [`HRESULT`](crate::co::HRESULT) | [COM errors](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a).
//! | [`AnyResult`](crate::AnyResult) | `Box<dyn Error + Send + Sync>` | Holding different error types. All other `Result` aliases can be converted into it. |
//!
//! # Utilities
//!
//! Beyond the [GUI](crate::gui) API, WinSafe features a few high-level
//! abstractions to deal with some particularly complex Win32 topics. Unless you
//! need something specific, prefer using these over the raw, native calls:
//!
//! | Utility | Used for |
//! | - | - |
//! | [`Encoding`](crate::Encoding) | String encodings. |
//! | [`File`](crate::File) | File read/write and other operations. |
//! | [`FileMapped`](crate::FileMapped) | Memory-mapped file operations. |
//! | [`Ini`](crate::Ini) | Managing key/value pairs of a `.ini` file. |
//! | [`path`](crate::path) | File path operations. |
//! | [`ResourceInfo`](crate::ResourceInfo) | Retrieve embedded data from executables or DLLs. |
//! | [`task_dlg`](crate::task_dlg) | Various dialog prompts. |
//! | [`WString`](crate::WString) | Managing native wide strings. |

#![cfg_attr(docsrs, feature(doc_cfg))]

// Declarations of macros used throughout the library.
// No macros are public.

#[macro_use] mod macros;

// Declarations of modules themselves.

#[cfg(feature = "comctl")] mod comctl;
#[cfg(feature = "comdlg")] mod comdlg;
#[cfg(feature = "dshow")] mod dshow;
#[cfg(feature = "dxgi")] mod dxgi;
#[cfg(feature = "gdi")] mod gdi;
#[cfg(feature = "kernel")] mod kernel;
#[cfg(feature = "msimg")] mod msimg;
#[cfg(feature = "ole")] mod ole;
#[cfg(feature = "oleaut")] mod oleaut;
#[cfg(feature = "shell")] mod shell;
#[cfg(feature = "user")] mod user;
#[cfg(feature = "uxtheme")] mod uxtheme;
#[cfg(feature = "version")] mod version;
#[cfg(all(feature = "comctl", feature = "gdi"))] mod comctl_gdi;
#[cfg(all(feature = "comctl", feature = "ole"))] mod comctl_ole;
#[cfg(all(feature = "comctl", feature = "shell"))] mod comctl_shell;
#[cfg(all(feature = "dshow", feature = "gdi"))] mod dshow_gdi;
#[cfg(all(feature = "gdi", feature = "ole"))] mod gdi_ole;

// The gui module itself is public.

#[cfg(feature = "gui")] pub mod gui;

// Declarations inside decl are public, placed at the root of the crate.

#[cfg(feature = "comctl")] pub use comctl::decl::*;
#[cfg(feature = "comdlg")] pub use comdlg::decl::*;
#[cfg(feature = "dshow")] pub use dshow::decl::*;
#[cfg(feature = "dxgi")] pub use dxgi::decl::*;
#[cfg(feature = "gdi")] pub use gdi::decl::*;
#[cfg(feature = "kernel")] pub use kernel::decl::*;
#[cfg(feature = "ole")] pub use ole::decl::*;
#[cfg(feature = "oleaut")] pub use oleaut::decl::*;
#[cfg(feature = "shell")] pub use shell::decl::*;
#[cfg(feature = "user")] pub use user::decl::*;
#[cfg(feature = "uxtheme")] pub use uxtheme::decl::*;
#[cfg(feature = "version")] pub use version::decl::*;
#[cfg(all(feature = "comctl", feature = "gdi"))] pub use comctl_gdi::decl::*;
#[cfg(all(feature = "comctl", feature = "ole"))] pub use comctl_ole::decl::*;

#[cfg(feature = "kernel")]
pub mod co {
	//! Native constants.
	//!
	//! All types can be converted from/to their underlying integer type. They
	//! all implement the [`NativeConst`](crate::prelude::NativeConst) trait;
	//! those who can be combined as bitflags also implement
	//! [`NativeBitflag`](crate::prelude::NativeBitflag).
	//!
	//! Among these constant types, three are error types:
	//! [`CDERR`](crate::co::CDERR), [`ERROR`](crate::co::ERROR) and
	//! [`HRESULT`](crate::co::HRESULT).
	#[cfg(feature = "comctl")] pub use super::comctl::co::*;
	#[cfg(feature = "comdlg")] pub use super::comdlg::co::*;
	#[cfg(feature = "dshow")] pub use super::dshow::co::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::co::*;
	#[cfg(feature = "gdi")] pub use super::gdi::co::*;
	#[cfg(feature = "kernel")] pub use super::kernel::co::*;
	#[cfg(feature = "ole")] pub use super::ole::co::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::co::*;
	#[cfg(feature = "shell")] pub use super::shell::co::*;
	#[cfg(feature = "user")] pub use super::user::co::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::co::*;
	#[cfg(feature = "version")] pub use super::version::co::*;
}

#[cfg(feature = "kernel")]
pub mod guard {
	//! RAII implementation for various resources, which automatically perform
	//! cleanup routines when the object goes out of scope.
	//!
	//! The guards are named after the functions they call.
	#[cfg(feature = "comctl")] pub use super::comctl::guard::*;
	#[cfg(feature = "gdi")] pub use super::gdi::guard::*;
	#[cfg(feature = "kernel")] pub use super::kernel::guard::*;
	#[cfg(feature = "ole")] pub use super::ole::guard::*;
	#[cfg(feature = "shell")] pub use super::shell::guard::*;
	#[cfg(feature = "user")] pub use super::user::guard::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::guard::*;
}

#[cfg(feature = "user")]
pub mod msg {
	//! Parameters of
	//! [window messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues).
	//!
	//! [`WndMsg`](crate::msg::WndMsg) is the generic message, with `WPARAM` and
	//! `LPARAM` fields. Other messages belong to a module according to its
	//! prefix, for example, [`BM_CLICK`](crate::msg::bm::Click) can be found in
	//! [`bm`](crate::msg::bm) module.
	//!
	//! # Examples
	//!
	//! We want to delete the 3rd element of a
	//! [`ListView`](crate::gui::ListView) control. This can be done by sending
	//! it an [`LVM_DELETEITEM`](crate::msg::lvm::DeleteItem) message via
	//! [`HWND::SendMessage`](crate::prelude::user_Hwnd::SendMessage). The
	//! message itself is a struct, which is initialized with the specific
	//! message parameters.
	//!
	//! The message struct also defines the data type returned by `SendMessage`.
	//! In the example below, `LVM_DELETEITEM` returns `SysResult<()>`.
	//!
	//! ```rust,ignore
	//! use winsafe::prelude::*;
	//! use winsafe::{HWND, msg::lvm};
	//!
	//! let hlistview: HWND; // initialized somewhere
	//! # let hlistview = HWND::NULL;
	//!
	//! hlistview.SendMessage(
	//!     lvm::DeleteItem {
	//!         index: 2,
	//!     },
	//! ).expect("Failed to delete item 2.");
	//! ```
	//!
	//! Messages are organized into modules according to their prefixes:
	//! [`wm`](crate::msg::wm) (window messages), [`lvm`](crate::msg::lvm) (list
	//! view messages), and so on.

	pub use super::user::messages::WndMsg;

	#[cfg(feature = "user")]
	pub mod bm {
		//! Button control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages),
		//! whose constants have [`BM`](crate::co::BM) and
		//! [`BCM`](crate::co::BCM) prefixes.
		pub use super::super::user::messages::bm::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::bcm::*;
	}

	#[cfg(feature = "user")]
	pub mod cb {
		//! Combo box control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages),
		//! whose constants have [`CB`](crate::co::CB) prefix.
		pub use super::super::user::messages::cb::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::cb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod dtm {
		//! Date and time picker control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages),
		//! whose constants have [`DTM`](crate::co::DTM) prefix.
		pub use super::super::comctl::messages::dtm::*;
		#[cfg(feature = "gdi")] pub use super::super::comctl_gdi::messages::dtm::*;
	}

	#[cfg(feature = "user")]
	pub mod em {
		//! Edit control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages),
		//! whose constants have [`EM`](crate::co::EM) prefix.
		pub use super::super::user::messages::em::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::em::*;
	}

	#[cfg(feature = "comctl")]
	pub mod hdm {
		//! Header control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages),
		//! whose constants have [`HDM`](crate::co::HDM) prefix.
		pub use super::super::comctl::messages::hdm::*;
	}

	#[cfg(feature = "user")]
	pub mod lb {
		//! ListBox control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages),
		//! whose constants have [`LB`](crate::co::LB) prefix.
		pub use super::super::user::messages::lb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod lvm {
		//! List view control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-messages),
		//! whose constants have [`LVM`](crate::co::LVM) prefix.
		pub use super::super::comctl::messages::lvm::*;
		#[cfg(feature = "ole")] pub use super::super::comctl_ole::messages::lvm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod mcm {
		//! Month calendar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-messages),
		//! whose constants have [`MCM`](crate::co::MCM) prefix.
		pub use super::super::comctl::messages::mcm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod pbm {
		//! Progress bar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-progress-bar-control-reference-messages),
		//! whose constants have [`PBM`](crate::co::PBM) prefix.
		pub use super::super::comctl::messages::pbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod sb {
		//! Status bar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-messages),
		//! whose constants have [`SB`](crate::co::SB) prefix.
		pub use super::super::comctl::messages::sb::*;
	}

	#[cfg(feature = "comctl")]
	pub mod stm {
		//! Static control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages),
		//! whose constants have [`STM`](crate::co::STM) prefix.
		pub use super::super::comctl::messages::stm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tbm {
		//! Toolbar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages),
		//! whose constants have [`TBM`](crate::co::TBM) prefix.
		pub use super::super::comctl::messages::tbm::*;
		#[cfg(feature = "ole")] pub use super::super::comctl_ole::messages::tbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tcm {
		//! Tab control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-messages),
		//! whose constants have [`TCM`](crate::co::TCM) prefix.
		pub use super::super::comctl::messages::tcm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod trbm {
		//! Trackbar control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages),
		//! whose constants have [`TRBM`](crate::co::TRBM) prefix.
		//!
		//! Originally has `TBM` prefix.
		pub use super::super::comctl::messages::trbm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod tvm {
		//! Tree view control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages),
		//! whose constants have [`TVM`](crate::co::TVM) prefix.
		pub use super::super::comctl::messages::tvm::*;
		#[cfg(feature = "ole")] pub use super::super::comctl_ole::messages::tvm::*;
	}

	#[cfg(feature = "comctl")]
	pub mod udm {
		//! UpDown control
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-messages),
		//! whose constants have [`UDM`](crate::co::UDM) prefix.
		pub use super::super::comctl::messages::udm::*;
	}

	#[cfg(feature = "user")]
	pub mod wm {
		//! Generic window
		//! [messages](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
		//! whose constants have [`WM`](crate::co::WM) prefix.
		pub use super::super::user::messages::wm::*;
		#[cfg(feature = "comctl")] pub use super::super::comctl::messages::wm::*;
		#[cfg(feature = "gdi")] pub use super::super::gdi::messages::wm::*;
		#[cfg(feature = "shell")] pub use super::super::shell::messages::wm::*;
	}
}

#[cfg(feature = "kernel")]
pub mod prelude {
	//! The WinSafe prelude.
	//!
	//! The purpose of this module is to alleviate imports of many common
	//! traits. To use it, add a glob import to the top of all your modules that
	//! use the library:
	//!
	//! ```rust,no_run
	//! use winsafe::prelude::*;
	//! ```
	#[cfg(feature = "comctl")] pub use super::comctl::traits::*;
	#[cfg(feature = "dshow")] pub use super::dshow::traits::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::traits::*;
	#[cfg(feature = "gdi")] pub use super::gdi::traits::*;
	#[cfg(feature = "gui")] pub use super::gui::traits::*;
	#[cfg(feature = "kernel")] pub use super::kernel::traits::*;
	#[cfg(feature = "msimg")] pub use super::msimg::traits::*;
	#[cfg(feature = "ole")] pub use super::ole::traits::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::traits::*;
	#[cfg(feature = "shell")] pub use super::shell::traits::*;
	#[cfg(feature = "user")] pub use super::user::traits::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::traits::*;
	#[cfg(all(feature = "gdi", feature = "ole"))] pub use super::gdi_ole::traits::*;
	#[cfg(all(feature = "comctl", feature = "ole"))] pub use super::comctl_ole::traits::*;
	#[cfg(all(feature = "comctl", feature = "shell"))] pub use super::comctl_shell::traits::*;
	#[cfg(all(feature = "dshow", feature = "gdi"))] pub use super::dshow_gdi::traits::*;
}

#[cfg(feature = "ole")]
pub mod vt {
	//! Virtual tables of COM interfaces.
	#[cfg(feature = "dshow")] pub use super::dshow::vt::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::vt::*;
	#[cfg(feature = "ole")] pub use super::ole::vt::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::vt::*;
	#[cfg(feature = "shell")] pub use super::shell::vt::*;
}
