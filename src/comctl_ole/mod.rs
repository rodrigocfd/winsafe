#![cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]

pub(in crate::comctl_ole) mod ffi;

mod aliases;
mod enums;
mod funcs;
mod handles;
mod structs;
mod utilities;

pub mod messages;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
