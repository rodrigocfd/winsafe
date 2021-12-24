pub(in crate::version) mod ffi;

mod funcs;
mod resource_info;
mod structs;

pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::resource_info::*;
	pub use super::structs::*;
}
