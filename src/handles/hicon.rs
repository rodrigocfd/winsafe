#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::ffi::user32;

handle_type! {
	/// Handle to an
	/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
	HICON
}

impl HICON {
	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	pub fn DestroyIcon(&self) -> Result<(), co::ERROR> {
		match unsafe { user32::DestroyIcon(self.0) } {
			0 => Err(co::ERROR::GetLastError()),
			_ => Ok(()),
		}
	}
}