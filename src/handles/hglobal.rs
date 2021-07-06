#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, GMEM_INVALID_HANDLE};

pub_struct_handle! {
	/// Handle to a
	/// [global memory block](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc).
	/// Originally just a `HANDLE`.
	HGLOBAL
}

impl HGLOBAL {
	/// [`GlobalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalFree`](crate::HGLOBAL::GlobalFree) call.
	pub fn GlobalAlloc(uFlags: co::GMEM, dwBytes: u64) -> WinResult<HGLOBAL> {
		unsafe { kernel32::GlobalAlloc(uFlags.0, dwBytes).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalFlags`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalflags)
	/// method.
	pub fn GlobalFlags(self) -> WinResult<co::GMEM> {
		match unsafe { kernel32::GlobalFlags(self.ptr) } {
			GMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(co::GMEM(flags)),
		}
	}

	/// [`GlobalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// method.
	pub fn GlobalFree(self) -> WinResult<()> {
		match unsafe { kernel32::GlobalFree(self.ptr).as_mut() } {
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		}
	}

	/// [`GlobalLock`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
	/// method.
	///
	/// Calls [`GlobalSize`](crate::HGLOBAL::GlobalSize) to retrieve the size of
	/// the memory block.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalUnlock`](crate::HGLOBAL::GlobalUnlock) call.
	pub fn GlobalLock<'a>(self) -> WinResult<&'a mut [u8]> {
		let memSz = self.GlobalSize()?;
		unsafe { kernel32::GlobalLock(self.ptr).as_mut() }
			.map(|ptr| unsafe {
				std::slice::from_raw_parts_mut(ptr as *mut _ as *mut _, memSz as _)
			})
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalReAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalrealloc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalFree`](crate::HGLOBAL::GlobalFree) call.
	pub fn GlobalReAlloc(self,
		dwBytes: u64, uFlags: co::GMEM) -> WinResult<HGLOBAL>
	{
		unsafe { kernel32::GlobalReAlloc(self.ptr, dwBytes, uFlags.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalSize`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalsize)
	/// method.
	pub fn GlobalSize(self) -> WinResult<u64> {
		match unsafe { kernel32::GlobalSize(self.ptr) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}

	/// [`GlobalUnlock`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// method.
	pub fn GlobalUnlock(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::GlobalUnlock(self.ptr) })
	}
}
