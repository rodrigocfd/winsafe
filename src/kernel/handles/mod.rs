mod haccesstoken;
mod handle;
mod hevent;
mod hfile;
mod hfilemap;
mod hfilemapview;
mod hfindfile;
mod hglobal;
mod hinstance;
mod hlocal;
mod hpipe;
mod hprocess;
mod hprocesslist;
mod hthread;
mod hupdatesrc;

pub mod decl {
	pub use super::haccesstoken::HACCESSTOKEN;
	pub use super::hevent::HEVENT;
	pub use super::hfile::HFILE;
	pub use super::hfilemap::HFILEMAP;
	pub use super::hfilemapview::HFILEMAPVIEW;
	pub use super::hfindfile::HFINDFILE;
	pub use super::hglobal::HGLOBAL;
	pub use super::hinstance::HINSTANCE;
	pub use super::hlocal::HLOCAL;
	pub use super::hpipe::HPIPE;
	pub use super::hprocess::HPROCESS;
	pub use super::hprocesslist::HPROCESSLIST;
	pub use super::hthread::HTHREAD;
	pub use super::hupdatesrc::HUPDATERSRC;

	impl_handle! { HRSRC: "kernel";
		/// Handle to a
		/// [resource](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew).
		/// Originally just a `HANDLE`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::KernelHinstance::LockResource).
	}

	impl_handle! { HRSRCMEM: "kernel";
		/// Handle to a resource
		/// [memory block](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource).
		/// Originally just an `HGLOBAL`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::KernelHinstance::LockResource).
	}
}

pub mod traits {
	pub use super::haccesstoken::KernelHaccesstoken;
	pub use super::handle::{Handle, HandleClose};
	pub use super::hfile::KernelHfile;
	pub use super::hfilemap::KernelHfilemap;
	pub use super::hfilemapview::KernelHfilemapview;
	pub use super::hfindfile::KernelHfindfile;
	pub use super::hglobal::KernelHglobal;
	pub use super::hinstance::KernelHinstance;
	pub use super::hlocal::KernelHlocal;
	pub use super::hpipe::KernelHpipe;
	pub use super::hprocess::KernelHprocess;
	pub use super::hprocesslist::KernelHprocesslist;
	pub use super::hthread::KernelHthread;
	pub use super::hupdatesrc::KernelHupdatersrc;
}
