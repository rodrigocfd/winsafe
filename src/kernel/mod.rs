#![cfg(feature = "kernel")]

mod aliases;
mod enums;
mod ffi;
mod funcs;
mod handles;
mod iterators;
mod proc;
mod structs;
mod utilities;

pub mod co;
pub(crate) mod ffi_types;
pub mod guards;
pub(crate) mod privs;
pub(crate) mod traits;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}
