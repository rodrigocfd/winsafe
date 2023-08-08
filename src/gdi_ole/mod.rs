#![cfg(all(feature = "gdi", feature = "ole"))]

mod com_interfaces;
mod handles;

pub(crate) mod privs;

pub mod traits {
	pub use super::com_interfaces::traits::*;
	pub use super::handles::traits::*;
}
