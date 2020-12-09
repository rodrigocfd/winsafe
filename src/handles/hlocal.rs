#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::ffi::kernel32;
use crate::GetLastError;

handle_type! {
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	HLOCAL
}

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(&self) -> Result<(), co::ERROR> {
		match ptr_to_opt!(kernel32::LocalFree(self.0)) {
			Some(_) => Err(GetLastError()),
			None => Ok(()),
		}
	}
}