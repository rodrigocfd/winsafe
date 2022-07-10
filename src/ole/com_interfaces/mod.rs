mod iunknown;

pub mod decl {
	pub use super::iunknown::{ComPtr, IUnknown};
}

pub mod traits {
	pub use super::iunknown::{ComInterface, ole_IUnknown};
}

pub mod vt {
	pub use super::iunknown::IUnknownVT;
}
