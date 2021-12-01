//! [IDL](https://docs.microsoft.com/en-us/windows/win32/api/objidl/) COM
//! interfaces, structs and constants, which are shared among other COM modules.
//!
//! To enable the IDL COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.7", features = ["idl"] }
//! ```

pub(in crate::com) mod ipersist;

pub use ipersist::IPersist;

pub(crate) mod prelude {
	pub use super::ipersist::IPersistT;
}

/// [IDL](https://docs.microsoft.com/en-us/windows/win32/api/objidl/) COM
/// virtual tables.
pub mod vt {
	pub use super::ipersist::IPersistVT;
}
