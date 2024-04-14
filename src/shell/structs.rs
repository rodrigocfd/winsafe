#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::prelude::*;

/// [`COMDLG_FILTERSPEC`](https://learn.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec)
/// struct.
#[repr(C)]
pub struct COMDLG_FILTERSPEC<'a, 'b> {
	pszName: *mut u16,
	pszSpec: *mut u16,

	_pszName: PhantomData<&'a mut u16>,
	_pszSpec: PhantomData<&'b mut u16>,
}

impl_default!(COMDLG_FILTERSPEC, 'a, 'b);

impl<'a, 'b> COMDLG_FILTERSPEC<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszName, set_pszName);
	pub_fn_string_ptr_get_set!('b, pszSpec, set_pszSpec);
}

/// [`NOTIFYICONDATA`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-notifyicondataw)
/// struct.
#[repr(C)]
pub struct NOTIFYICONDATA {
	cbSize: u32,
	pub hWnd: HWND,
	pub uID: u32,
	pub uFlags: co::NIF,
	pub uCallbackMessage: co::WM,
	pub hIcon: HICON,
	szTip: [u16; 128],
	pub dwState: co::NIS,
	pub dwStateMask: co::NIS,
	szInfo: [u16; 256],
	pub uVersion: u32, // union with uTimeout, which is deprecated
	szInfoTitle: [u16; 64],
	pub dwInfoFlags: co::NIIF,
	pub guidItem: GUID,
	pub hBalloonIcon: HICON,
}

impl_default_with_size!(NOTIFYICONDATA, cbSize);

impl NOTIFYICONDATA {
	pub_fn_string_arr_get_set!(szTip, set_szTip);
	pub_fn_string_arr_get_set!(szInfo, set_szInfo);
	pub_fn_string_arr_get_set!(szInfoTitle, set_szInfoTitle);
}

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

/// [`SHFILEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileinfow)
/// struct.
#[repr(C)]
pub struct SHFILEINFO {
	pub hIcon: HICON,
	pub iIcon: i32,
	dwAttributes: u32,
	szDisplayName: [u16; MAX_PATH],
	szTypeName: [u16; 80],
}

impl_default!(SHFILEINFO);

impl SHFILEINFO {
	pub_fn_string_arr_get_set!(szDisplayName, set_szDisplayName);
	pub_fn_string_arr_get_set!(szTypeName, set_szTypeName);
}

/// [`SHFILEOPSTRUCT`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileopstructw)
/// struct.
#[repr(C)]
pub struct SHFILEOPSTRUCT<'a, 'b, 'c> {
	pub hwnd: HWND,
	pub wFunc: co::FO,
	pFrom: *mut u16, // double-null terminated
	pTo: *mut u16, // double-null terminated
	pub fFlags: co::FOF,
	fAnyOperationsAborted: BOOL,
	hNameMappings: *mut std::ffi::c_void, // lots of stuff going here...
	lpszProgressTitle: *mut u16,

	_pFrom: PhantomData<&'a mut u16>,
	_pTo: PhantomData<&'b mut u16>,
	_lpszProgressTitle: PhantomData<&'c mut u16>,
}

impl_default!(SHFILEOPSTRUCT, 'a, 'b, 'c);

impl<'a, 'b, 'c> SHFILEOPSTRUCT<'a, 'b, 'c> {
	pub_fn_bool_get_set!(fAnyOperationsAborted, set_fAnyOperationsAborted);

	/// Retrieves the `pFrom` field.
	#[must_use]
	pub fn pFrom(&self) -> Option<Vec<String>> {
 		unsafe { self.pFrom.as_mut() }
			.map(|p| parse_multi_z_str(p))
	}

	/// Sets the `pFrom` field.
	///
	/// **Note:** You must create the string with
	/// [`WString::from_str_vec`](crate::WString::from_str_vec).
	pub fn set_pFrom(&mut self, val: Option<&'a mut WString>) {
		self.pFrom = val.map_or(std::ptr::null_mut(), |v| unsafe { v.as_mut_ptr() });
	}

	/// Retrieves the `pTo` field.
	#[must_use]
	pub fn pTo(&self) -> Option<Vec<String>> {
		unsafe { self.pTo.as_mut() }
		  .map(|p| parse_multi_z_str(p))
	}

	/// Sets the `pTo` field.
	///
	/// **Note:** You must create the string with
	/// [`WString::from_str_vec`](crate::WString::from_str_vec).
	pub fn set_pTo(&mut self, val: Option<&'b mut WString>) {
		self.pTo = val.map_or(std::ptr::null_mut(), |v| unsafe { v.as_mut_ptr() });
	}

	pub_fn_string_ptr_get_set!('c, lpszProgressTitle, set_lpszProgressTitle);
}

/// [`SHSTOCKICONINFO`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shstockiconinfo)
/// struct.
#[repr(C)]
pub struct SHSTOCKICONINFO {
	cbSize: u32,
	pub hIcon: HICON,
	pub iSysImageIndex: i32,
	pub iIcon: i32,
	szPath: [u16; MAX_PATH],
}

impl_default_with_size!(SHSTOCKICONINFO, cbSize);

impl SHSTOCKICONINFO {
	pub_fn_string_arr_get_set!(szPath, get_szPath);
}
