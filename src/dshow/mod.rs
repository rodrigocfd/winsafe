mod clsid;
mod com_interfaces;
mod structs;

pub mod co;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
