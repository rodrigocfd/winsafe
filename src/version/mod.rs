pub(in crate::version) mod ffi;

mod funcs;
mod structs;
mod utilities;

pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}
