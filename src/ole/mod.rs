pub(in crate::ole) mod ffi;
pub(crate) mod privs;

mod aliases;
mod funcs;
mod iunknown;
mod structs;

pub mod co;

pub mod decl {
	pub use super::aliases::*;
	pub use super::funcs::*;
	pub use super::iunknown::{ComPtr, IUnknown};
	pub use super::structs::*;
}

pub mod traits {
	pub use super::iunknown::{ComInterface, OleIUnknown};
}

pub mod vt {
	pub use super::iunknown::IUnknownVT;
}
