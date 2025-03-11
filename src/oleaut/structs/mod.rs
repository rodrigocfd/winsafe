mod bstr;
mod propvariant;
mod structs_other;
mod variant_traits;
mod variant;

pub mod decl {
	pub use super::bstr::BSTR;
	pub use super::propvariant::PROPVARIANT;
	pub use super::structs_other::*;
	pub use super::variant::VARIANT;
}

pub mod traits {
	pub use super::variant_traits::*;
}
