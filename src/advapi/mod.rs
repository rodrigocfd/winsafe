#![cfg(feature = "advapi")]

mod callbacks;
mod enums;
mod ffi;
mod funcs;
mod guards;
mod handles;
mod iterators;
mod privs;
mod structs;

pub mod co;

pub mod decl {
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::guards::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}
