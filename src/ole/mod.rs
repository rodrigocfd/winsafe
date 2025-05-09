#![cfg(feature = "ole")]

mod aliases;
mod com_impls;
mod com_interfaces;
mod funcs;
mod handles;
mod structs;

pub mod co;
pub(in crate::ole) mod ffi;
pub mod guard;
pub(crate) mod privs;
pub(crate) mod vts;

pub mod decl {
	pub use super::aliases::*;
	pub use super::com_impls::decl::*;
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;

	handle! { HMETAFILEPICT;
		/// Handle to a
		/// [metafile](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-ustgmedium-r1).
	}

	handle! { HENHMETAFILE;
		/// Handle to an
		/// [enhanced metafile](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-ustgmedium-r1).
	}
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}
