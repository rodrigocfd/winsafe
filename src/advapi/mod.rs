#![cfg(feature = "advapi")]

mod enums;
mod ffi;
mod funcs;
mod handles;
mod iterators;
mod privs;
mod proc;
mod structs;

pub mod co;
pub mod guards;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}
