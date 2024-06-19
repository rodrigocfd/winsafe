mod idispatch;
mod ipropertystore;
mod itypeinfo;

pub mod decl {
	pub use super::idispatch::IDispatch;
	pub use super::ipropertystore::IPropertyStore;
	pub use super::itypeinfo::ITypeInfo;
}

pub mod traits {
	pub use super::idispatch::oleaut_IDispatch;
	pub use super::ipropertystore::oleaut_IPropertyStore;
	pub use super::itypeinfo::oleaut_ITypeInfo;
}
