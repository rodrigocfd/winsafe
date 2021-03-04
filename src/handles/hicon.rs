#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::user32;
use crate::privs::bool_to_winresult;

handle_type! {
	/// Handle to an
	/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
	HICON
}

impl HICON {
	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	pub fn DestroyIcon(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DestroyIcon(self.ptr) })
	}
}
