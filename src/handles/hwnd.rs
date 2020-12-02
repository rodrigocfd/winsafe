#![allow(non_snake_case)]

use crate::ffi::*;
use crate::co;
use crate::Utf16;

/// Handle to a
/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWND(*mut Void);

impl Default for HWND {
	fn default() -> Self {
		HWND(std::ptr::null_mut())
	}
}

impl HWND {
	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	pub fn MessageBox(self, lpText: &str, lpCaption: &str, uType: co::MB) -> co::DLGID {
		let dlgid = unsafe {
			user32::MessageBoxW(self.0, Utf16::from_str(lpText).as_ptr(),
				Utf16::from_str(lpCaption).as_ptr(), uType.into())
		};
		co::DLGID::from(dlgid)
	}
}