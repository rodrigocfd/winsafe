#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

/// [`SHELLEXECUTEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shellexecuteinfow)
/// struct.
#[repr(C)]
pub struct SHELLEXECUTEINFO<'a, 'b, 'c, 'd, 'e> {
	cbSize: u32,
	pub fMask: co::SEE_MASK,
	pub hwnd: HWND,
	lpVerb: *mut u16,
	lpFile: *mut u16,
	lpParameters: *mut u16,
	lpDirectory: *mut u16,
	pub nShow: co::SW,
	pub hInstApp: HINSTANCE,
	pub lpIDList: *mut std::ffi::c_void,
	lpClass: *mut u16,
	pub hkeyClass: HKEY,
	dwHotKey: u32,
	hIcon_hMonitor: HANDLE, // union HICON and HMONITOR
	pub hProcess: HPROCESS,

	_lpVerb: PhantomData<&'a mut u16>,
	_lpFile: PhantomData<&'b mut u16>,
	_lpParameters: PhantomData<&'c mut u16>,
	_lpDirectory: PhantomData<&'d mut u16>,
	_lpClass: PhantomData<&'e mut u16>,
}

impl_default_with_size!(SHELLEXECUTEINFO, cbSize, 'a, 'b, 'c, 'd, 'e);

impl<'a, 'b, 'c, 'd, 'e> SHELLEXECUTEINFO<'a, 'b, 'c, 'd, 'e> {
	pub_fn_string_ptr_get_set!('a, lpVerb, set_lpVerb);
	pub_fn_string_ptr_get_set!('b, lpFile, set_lpFile);
	pub_fn_string_ptr_get_set!('c, lpParameters, set_lpParameters);
	pub_fn_string_ptr_get_set!('d, lpDirectory, set_lpDirectory);
	pub_fn_string_ptr_get_set!('e, lpClass, set_lpClass);

	/// Retrieves the `dwHotKey` field.
	#[must_use]
	pub const fn dwHotKey(&self) -> (co::VK, co::HOTKEYF) {
		unsafe {(
			co::VK::from_raw(LOWORD(self.dwHotKey)),
			co::HOTKEYF::from_raw(HIWORD(self.dwHotKey)),
		)}
	}

	/// Sets the `dwHotKey` field.
	pub fn set_dwHotKey(&mut self, val: (co::VK, co::HOTKEYF)) {
		self.dwHotKey = MAKEDWORD(val.0.raw(), val.1.raw())
	}

	/// Retrieves the `hIcon`/`hMonitor` union field.
	///
	/// # Safety
	///
	/// Both `hIcon` and `hMonitor` fields share the same memory space so both
	/// are returned, and you must choose which one you want.
	///
	/// # Examples
	///
	/// Retrieving `hIcon`:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let sei = w::SHELLEXECUTEINFO::default();
	///
	/// let (hicon, _) = unsafe { sei.hIcon_hMonitor() };
	/// ```
	#[must_use]
	pub unsafe fn hIcon_hMonitor(&self) -> (HICON, HMONITOR) {
		(HICON::from_ptr(self.hIcon_hMonitor),
			HMONITOR::from_ptr(self.hIcon_hMonitor))
	}

	/// Sets the `hIcon`/`hMonitor` union field as `hIcon`.
	///
	/// # Safety
	///
	/// Both `hIcon` and `hMonitor` fields share the same memory space, so when
	/// setting one, you replace the other.
	pub unsafe fn set_hIcon(&mut self, hicon: &HICON) {
		self.hIcon_hMonitor = hicon.ptr();
	}

	/// Sets the `hIcon`/`hMonitor` union field as `hMonitor`.
	///
	/// # Safety
	///
	/// Both `hIcon` and `hMonitor` fields share the same memory space, so when
	/// setting one, you replace the other.
	pub unsafe fn set_hMonitor(&mut self, hmonitor: &HMONITOR) {
		self.hIcon_hMonitor = hmonitor.ptr();
	}
}
