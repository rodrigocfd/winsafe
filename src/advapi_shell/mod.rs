#![cfg(all(feature = "advapi", feature = "shell"))]

mod ffi;
mod funcs;
mod structs;

pub mod co;

pub mod decl {
	pub use super::funcs::*;
	pub use super::structs::*;
}
