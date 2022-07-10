pub(crate) mod privs;

mod com_interfaces;
mod handles;

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}