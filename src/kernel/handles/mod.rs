mod hevent;
mod hfile;
mod hfilemap;
mod hfilemapview;
mod hfindfile;
mod hglobal;
mod hheap;
mod hinstance;
mod hlocal;
mod hpipe;
mod hprocess;
mod hprocesslist;
mod hstd;
mod hthread;
mod hupdatesrc;

pub mod decl {
	pub use super::hevent::HEVENT;
	pub use super::hfile::HFILE;
	pub use super::hfilemap::HFILEMAP;
	pub use super::hfilemapview::HFILEMAPVIEW;
	pub use super::hfindfile::HFINDFILE;
	pub use super::hglobal::HGLOBAL;
	pub use super::hheap::HHEAP;
	pub use super::hinstance::HINSTANCE;
	pub use super::hlocal::HLOCAL;
	pub use super::hpipe::HPIPE;
	pub use super::hprocess::HPROCESS;
	pub use super::hprocesslist::HPROCESSLIST;
	pub use super::hstd::HSTD;
	pub use super::hthread::HTHREAD;
	pub use super::hupdatesrc::HUPDATERSRC;

	handle! { HRSRC;
		/// Handle to a
		/// [resource](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew).
		/// Originally just a `HANDLE`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	}

	handle! { HRSRCMEM;
		/// Handle to a resource
		/// [memory block](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource).
		/// Originally just an `HGLOBAL`.
		///
		/// For an example, see
		/// [`HINSTANCE::LockResource`](crate::prelude::kernel_Hinstance::LockResource).
	}
}

pub mod traits {
	pub use super::hevent::kernel_Hevent;
	pub use super::hfile::kernel_Hfile;
	pub use super::hfilemap::kernel_Hfilemap;
	pub use super::hfilemapview::kernel_Hfilemapview;
	pub use super::hfindfile::kernel_Hfindfile;
	pub use super::hglobal::kernel_Hglobal;
	pub use super::hheap::kernel_Hheap;
	pub use super::hinstance::kernel_Hinstance;
	pub use super::hlocal::kernel_Hlocal;
	pub use super::hpipe::kernel_Hpipe;
	pub use super::hprocess::kernel_Hprocess;
	pub use super::hprocesslist::kernel_Hprocesslist;
	pub use super::hstd::kernel_Hstd;
	pub use super::hthread::kernel_Hthread;
	pub use super::hupdatesrc::kernel_Hupdatersrc;
}
