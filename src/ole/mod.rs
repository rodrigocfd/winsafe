#![cfg(feature = "ole")]

mod aliases;
mod com_interfaces;
mod funcs;
mod handles;
mod structs;

pub(in crate::ole) mod ffi;
pub(crate) mod privs;
pub(crate) mod vts;
pub mod co;
pub mod guard;

pub mod decl {
	pub use super::aliases::*;
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;

	impl_handle! { HMETAFILEPICT;
		/// Handle to a
		/// [metafile](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-ustgmedium-r1).
	}

	impl_handle! { HENHMETAFILE;
		/// Handle to an
		/// [enhanced metafile](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-ustgmedium-r1).
	}
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}
