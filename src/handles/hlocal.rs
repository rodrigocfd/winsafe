#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::ptr_as_opt;

handle_type! {
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	HLOCAL
}

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(self) -> WinResult<()> {
		match ptr_as_opt(unsafe { kernel32::LocalFree(self.ptr) }) {
			Some(_) => Err(GetLastError()),
			None => Ok(()),
		}
	}
}
