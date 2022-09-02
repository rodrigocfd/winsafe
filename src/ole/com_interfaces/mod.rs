mod com_ptr;
mod iunknown;

pub mod decl {
	pub use super::com_ptr::ComPtr;
	pub use super::iunknown::IUnknown;
}

pub mod traits {
	pub use super::iunknown::ole_IUnknown;
}

pub mod vt {
	pub use super::iunknown::IUnknownVT;
}
