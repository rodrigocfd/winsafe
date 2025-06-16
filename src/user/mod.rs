#![cfg(feature = "user")]

mod aliases;
mod callbacks;
mod enums;
mod funcs;
mod handles;
mod structs;

pub mod co;
pub(in crate::user) mod ffi;
pub mod guards;
pub(in crate::user) mod iterators;
pub mod messages;
pub(crate) mod privs;
pub(crate) mod traits;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}
