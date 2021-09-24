#![allow(non_snake_case)]

use crate::aliases::WinResult;

pub_struct_handle_closeable! {
	/// Handle to an
	/// [event](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw).
	/// Originally just a `HANDLE`.
	HEVENT
}

pub_struct_handle! {
	/// Handle to a
	/// [resource](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-findresourcew).
	/// Originally just a `HANDLE`.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	HRSRC
}

pub_struct_handle! {
	/// Handle to a resource
	/// [memory block](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadresource).
	/// Originally just an `HGLOBAL`.
	///
	/// For an example, see
	/// [`HINSTANCE::LockResource`](crate::HINSTANCE::LockResource).
	HRSRCMEM
}

pub_struct_handle! {
	/// Handle to an
	/// [tree view item](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	HTREEITEM
}
