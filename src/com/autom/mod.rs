//! [Automation](https://docs.microsoft.com/en-us/windows/win32/api/_automat/)
//! COM interfaces, structs and constants.
//!
//! To enable the DirectShow COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.7", features = ["autom"] }
//! ```

pub(in crate::com) mod idispatch;
pub(in crate::com) mod itypeinfo;

pub use idispatch::IDispatch;
pub use itypeinfo::ITypeInfo;

pub(crate) mod prelude {
	pub use super::idispatch::IDispatchT;
	pub use super::itypeinfo::ITypeInfoT;
}

/// [Automation](https://docs.microsoft.com/en-us/windows/win32/api/_automat/)
/// COM virtual tables.
pub mod vt {
	pub use super::idispatch::IDispatchVT;
	pub use super::itypeinfo::ITypeInfoVT;
}
