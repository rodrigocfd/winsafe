#![cfg(feature = "wininet")]

mod enums;
mod funcs;
mod handles;

pub mod co;
pub(in crate::wininet) mod ffi;
pub mod guards;
pub(crate) mod privs;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
}
