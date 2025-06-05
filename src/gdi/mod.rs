#![cfg(feature = "gdi")]

mod enums;
mod funcs;
mod gdi_traits;
mod handles;
mod structs;

pub mod co;
pub(in crate::gdi) mod ffi;
pub mod guards;
pub mod messages;
pub(crate) mod privs;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::gdi_traits::*;
}
