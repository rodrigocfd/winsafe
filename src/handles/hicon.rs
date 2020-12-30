#![allow(non_snake_case)]

use crate::co;
use crate::ffi::user32;
use crate::funcs::GetLastError;

handle_type! {
	/// Handle to an
	/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
	/// Exposes methods.
	HICON
}

impl HICON {
	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	pub fn DestroyIcon(&self) -> Result<(), co::ERROR> {
		match unsafe { user32::DestroyIcon(self.0) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}
