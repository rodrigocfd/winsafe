pub(in crate::gdi) mod ffi;
pub(crate) mod privs;

mod handles;
mod structs;

pub mod co;
pub mod messages;

pub mod decl {
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod guard {
	pub use super::handles::guard::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
