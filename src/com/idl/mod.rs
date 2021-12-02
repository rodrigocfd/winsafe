//! [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) COM
//! interfaces, structs and constants, which are shared among other COM modules.
//!
//! To enable the IDL COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.8", features = ["idl"] }
//! ```

pub(in crate::com) mod ipersist;
pub(in crate::com) mod ipicture;

pub use ipersist::IPersist;
pub use ipicture::IPicture;

pub(crate) mod prelude {
	pub use super::ipersist::IPersistT;
	pub use super::ipicture::IPictureT;
}

/// [IDL](https://docs.microsoft.com/en-us/windows/win32/api/_com/) COM
/// virtual tables.
pub mod vt {
	pub use super::ipersist::IPersistVT;
	pub use super::ipicture::IPictureVT;
}
