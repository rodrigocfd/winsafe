#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::user32;
use crate::privs::bool_to_winresult;

handle_type! {
	/// Handle to a
	/// [cursor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
	HCURSOR
}

impl HCURSOR {
	/// [`SetSystemCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// method.
	pub fn SetSystemCursor(self, id: co::OCR) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::SetSystemCursor(self.ptr, id.0) })
	}
}
