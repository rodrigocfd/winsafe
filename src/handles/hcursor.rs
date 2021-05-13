#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;

pub_struct_handle! {
	/// Handle to a
	/// [cursor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
	HCURSOR
}

impl HCURSOR {
	/// [`CopyCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// method. Originally a macro.
	///
	/// **Note:** Must be paired with a
	/// [`DestroyCursor`](crate::HCURSOR::DestroyCursor) call.
	pub fn CopyCursor(self) -> WinResult<HCURSOR> {
		unsafe { user32::CopyIcon(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroycursor)
	/// method.
	pub fn DestroyCursor(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DestroyCursor(self.ptr) })
	}

	/// [`SetSystemCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// method.
	pub fn SetSystemCursor(self, id: co::OCR) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::SetSystemCursor(self.ptr, id.0) })
	}
}
