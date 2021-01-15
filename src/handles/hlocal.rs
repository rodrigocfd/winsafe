#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::funcs_priv::ptr_as_opt;
use crate::funcs::GetLastError;

handle_type! {
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	/// Exposes methods.
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
