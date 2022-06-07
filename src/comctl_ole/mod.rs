pub(in crate::comctl_ole) mod ffi;

mod aliases;
mod enums;
mod funcs;
mod hwnd;
mod structs;

pub mod messages;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::hwnd::ComctlOleHwnd;
}
