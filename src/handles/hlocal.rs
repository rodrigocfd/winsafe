#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;

pub_struct_handle! {
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	HLOCAL
}

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(self) -> WinResult<()> {
		match unsafe { kernel32::LocalFree(self.ptr).as_mut() } {
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		}
	}

	/// [`LocalSize`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	pub fn LocalSize(self) -> WinResult<u64> {
		match unsafe { kernel32::LocalSize(self.ptr) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
