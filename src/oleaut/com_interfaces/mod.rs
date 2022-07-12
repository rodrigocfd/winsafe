mod idispatch;
mod itypeinfo;
mod ipicture;
mod ipropertystore;

pub mod decl {
	pub use super::idispatch::IDispatch;
	pub use super::itypeinfo::ITypeInfo;
	pub use super::ipicture::IPicture;
	pub use super::ipropertystore::IPropertyStore;
}

pub mod traits {
	pub use super::idispatch::oleaut_IDispatch;
	pub use super::itypeinfo::oleaut_ITypeInfo;
	pub use super::ipicture::oleaut_IPicture;
	pub use super::ipropertystore::oleaut_IPropertyStore;
}

pub mod vt {
	pub use super::idispatch::IDispatchVT;
	pub use super::itypeinfo::ITypeInfoVT;
	pub use super::ipicture::IPictureVT;
	pub use super::ipropertystore::IPropertyStoreVT;
}
