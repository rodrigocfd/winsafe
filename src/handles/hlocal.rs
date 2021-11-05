#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;

/// Handle to a
/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HLOCAL(pub(crate) *mut std::ffi::c_void);

impl_handle!(HLOCAL);

impl HLOCAL {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	pub fn LocalFree(self) -> WinResult<()> {
		match unsafe { kernel32::LocalFree(self.0).as_mut() } {
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		}
	}

	/// [`LocalSize`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	pub fn LocalSize(self) -> WinResult<u64> {
		match unsafe { kernel32::LocalSize(self.0) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
