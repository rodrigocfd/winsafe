#![cfg(feature = "comctl")]

mod aliases;
mod enums;
mod funcs;
mod handles;
mod proc;
mod structs;

pub mod co;
pub(in crate::comctl) mod ffi;
pub mod guard;
pub(in crate::comctl) mod iterators;
pub mod messages;
pub(crate) mod privs;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
