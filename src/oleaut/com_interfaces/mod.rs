mod idispatch;
mod itypeinfo;
mod ipicture;

pub mod decl {
	pub use super::idispatch::IDispatch;
	pub use super::itypeinfo::ITypeInfo;
	pub use super::ipicture::IPicture;
}

pub mod traits {
	pub use super::idispatch::oleaut_IDispatch;
	pub use super::itypeinfo::oleaut_ITypeInfo;
	pub use super::ipicture::oleaut_IPicture;
}

pub mod vt {
	pub use super::idispatch::IDispatchVT;
	pub use super::itypeinfo::ITypeInfoVT;
	pub use super::ipicture::IPictureVT;
}
