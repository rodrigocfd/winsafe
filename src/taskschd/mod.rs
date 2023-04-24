#![cfg_attr(docsrs, doc(cfg(feature = "taskschd")))]

pub mod co;

mod com_interfaces;

pub mod decl {
	pub use super::com_interfaces::decl::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
