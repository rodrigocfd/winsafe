#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::ffi_types::BOOL;
use crate::kernel::decl::WString;
use crate::kernel::privs::{MAX_PATH, parse_multi_z_str};
use crate::ole::decl::GUID;
use crate::user::decl::{HICON, HWND};

/// [`COMDLG_FILTERSPEC`](https://docs.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct COMDLG_FILTERSPEC<'a, 'b> {
	pszName: *mut u16,
	pszSpec: *mut u16,

	pszName_: PhantomData<&'a mut u16>,
	pszSpec_: PhantomData<&'b mut u16>,
}

impl_default!(COMDLG_FILTERSPEC, 'a, 'b);

impl<'a, 'b> COMDLG_FILTERSPEC<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszName, set_pszName);
	pub_fn_string_ptr_get_set!('b, pszSpec, set_pszSpec);
}

/// [`NOTIFYICONDATA`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-notifyicondataw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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

/// [`SHFILEINFO`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileinfow)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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

/// [`SHFILEOPSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-shfileopstructw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
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

	pFrom_: PhantomData<&'a mut usize>,
	pTo_: PhantomData<&'b mut usize>,
	lpszProgressTitle_: PhantomData<&'c mut usize>,
}

impl_default!(SHFILEOPSTRUCT, 'a, 'b, 'c);

impl<'a, 'b, 'c> SHFILEOPSTRUCT<'a, 'b, 'c> {
	pub_fn_bool_get_set!(fAnyOperationsAborted, set_fAnyOperationsAborted);

	/// Retrieves the `pFrom` field.
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
