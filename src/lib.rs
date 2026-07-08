#![doc = include_str!("lib.md")]
#![cfg_attr(any(), rustfmt::skip)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Declaration of macros.

mod macros;

// Declarations of modules themselves.

#[cfg(feature = "advapi")] mod advapi;
#[cfg(feature = "comctl")] mod comctl;
#[cfg(feature = "dshow")] mod dshow;
#[cfg(feature = "dwm")] mod dwm;
#[cfg(feature = "dxgi")] mod dxgi;
#[cfg(feature = "gdi")] mod gdi;
#[cfg(feature = "htmlhelp")] mod htmlhelp;
#[cfg(feature = "kernel")] mod kernel;
#[cfg(feature = "mf")] mod mf;
#[cfg(feature = "ole")] mod ole;
#[cfg(feature = "oleaut")] mod oleaut;
#[cfg(feature = "psapi")] mod psapi;
#[cfg(feature = "shell")] mod shell;
#[cfg(feature = "taskschd")] mod taskschd;
#[cfg(feature = "user")] mod user;
#[cfg(feature = "uxtheme")] mod uxtheme;
#[cfg(feature = "version")] mod version;
#[cfg(feature = "winmm")] mod winmm;
#[cfg(feature = "wininet")] mod wininet;
#[cfg(feature = "winspool")] mod winspool;
#[cfg(all(feature = "advapi", feature = "comctl"))] mod advapi_comctl;
#[cfg(all(feature = "advapi", feature = "shell"))] mod advapi_shell;
#[cfg(all(feature = "comctl", feature = "gdi"))] mod comctl_gdi;
#[cfg(all(feature = "comctl", feature = "shell"))] mod comctl_shell;
#[cfg(all(feature = "gdi", feature = "mf"))] mod gdi_mf;

// The gui module itself is public.

#[cfg(feature = "gui")] pub mod gui;

// Declarations inside decl are public, placed at the root of the crate.

mod decl {
	#[cfg(feature = "advapi")] pub use super::advapi::decl::*;
	#[cfg(feature = "comctl")] pub use super::comctl::decl::*;
	#[cfg(feature = "dshow")] pub use super::dshow::decl::*;
	#[cfg(feature = "dwm")] pub use super::dwm::decl::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::decl::*;
	#[cfg(feature = "gdi")] pub use super::gdi::decl::*;
	#[cfg(feature = "htmlhelp")] pub use super::htmlhelp::decl::*;
	#[cfg(feature = "kernel")] pub use super::kernel::decl::*;
	#[cfg(feature = "mf")] pub use super::mf::decl::*;
	#[cfg(feature = "ole")] pub use super::ole::decl::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::decl::*;
	#[cfg(feature = "psapi")] pub use super::psapi::decl::*;
	#[cfg(feature = "shell")] pub use super::shell::decl::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::decl::*;
	#[cfg(feature = "user")] pub use super::user::decl::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::decl::*;
	#[cfg(feature = "version")] pub use super::version::decl::*;
	#[cfg(feature = "winmm")] pub use super::winmm::decl::*;
	#[cfg(feature = "wininet")] pub use super::wininet::decl::*;
	#[cfg(feature = "winspool")] pub use super::winspool::decl::*;
	#[cfg(all(feature = "advapi", feature = "comctl"))] pub use super::advapi_comctl::decl::*;
	#[cfg(all(feature = "advapi", feature = "shell"))] pub use super::advapi_shell::decl::*;
	#[cfg(all(feature = "comctl", feature = "gdi"))] pub use super::comctl_gdi::decl::*;
}

#[allow(unused)]
pub use decl::*;

pub mod co {
	//! Native constants.
	//!
	//! Among these constant types, three are error types:
	//! [`CDERR`], [`ERROR`] and [`HRESULT`].

	#[cfg(feature = "advapi")] pub use super::advapi::co::*;
	#[cfg(feature = "comctl")] pub use super::comctl::co::*;
	#[cfg(feature = "dshow")] pub use super::dshow::co::*;
	#[cfg(feature = "dwm")] pub use super::dwm::co::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::co::*;
	#[cfg(feature = "gdi")] pub use super::gdi::co::*;
	#[cfg(feature = "htmlhelp")] pub use super::htmlhelp::co::*;
	#[cfg(feature = "kernel")] pub use super::kernel::co::*;
	#[cfg(feature = "mf")] pub use super::mf::co::*;
	#[cfg(feature = "ole")] pub use super::ole::co::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::co::*;
	#[cfg(feature = "shell")] pub use super::shell::co::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::co::*;
	#[cfg(feature = "user")] pub use super::user::co::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::co::*;
	#[cfg(feature = "version")] pub use super::version::co::*;
	#[cfg(feature = "winmm")] pub use super::winmm::co::*;
	#[cfg(feature = "wininet")] pub use super::wininet::co::*;
	#[cfg(feature = "winspool")] pub use super::winspool::co::*;
	#[cfg(all(feature = "advapi", feature = "shell"))] pub use super::advapi_shell::co::*;
}

pub mod guard {
	//! RAII implementation for various resources, which automatically perform
	//! cleanup routines when the object goes out of scope.
	//!
	//! The guards are named after the functions they call.

	#[cfg(feature = "advapi")] pub use super::advapi::guards::*;
	#[cfg(feature = "comctl")] pub use super::comctl::guards::*;
	#[cfg(feature = "gdi")] pub use super::gdi::guards::*;
	#[cfg(feature = "kernel")] pub use super::kernel::guards::*;
	#[cfg(feature = "mf")] pub use super::mf::guards::*;
	#[cfg(feature = "ole")] pub use super::ole::guards::*;
	#[cfg(feature = "shell")] pub use super::shell::guards::*;
	#[cfg(feature = "user")] pub use super::user::guards::*;
	#[cfg(feature = "uxtheme")] pub use super::uxtheme::guards::*;
	#[cfg(feature = "version")] pub use super::version::guards::*;
	#[cfg(feature = "wininet")] pub use super::wininet::guards::*;
	#[cfg(feature = "winspool")] pub use super::winspool::guards::*;
}

#[cfg(feature = "user")]
pub mod msg {
	#![doc = include_str!("msg.md")]

	pub use super::user::messages::*;
	#[cfg(feature = "comctl")] pub use super::comctl::messages::*;
	#[cfg(feature = "gdi")] pub use super::gdi::messages_wm::*;
	#[cfg(feature = "shell")] pub use super::shell::messages_wm::*;
	#[cfg(all(feature = "advapi", feature = "comctl"))] pub use super::advapi_comctl::messages_tb::*;
	#[cfg(all(feature = "comctl", feature = "gdi"))] pub use super::comctl_gdi::messages_dtm::*;
}

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

	#[cfg(feature = "gdi")] pub use super::gdi::traits::*;
	#[cfg(feature = "gui")] pub use super::gui::traits::*;
	#[cfg(feature = "kernel")] pub use super::kernel::traits::*;
	#[cfg(feature = "user")] pub use super::user::traits::*;

	#[cfg(feature = "dshow")] pub use super::dshow::traits::*;
	#[cfg(feature = "dxgi")] pub use super::dxgi::traits::*;
	#[cfg(feature = "mf")] pub use super::mf::traits::*;
	#[cfg(feature = "ole")] pub use super::ole::traits::*;
	#[cfg(feature = "oleaut")] pub use super::oleaut::traits::*;
	#[cfg(feature = "shell")] pub use super::shell::traits::*;
	#[cfg(feature = "taskschd")] pub use super::taskschd::traits::*;
	#[cfg(all(feature = "gdi", feature = "mf"))] pub use super::gdi_mf::traits::*;
}
