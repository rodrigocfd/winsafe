#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::{bool_to_winresult, GMEM_INVALID_HANDLE};
use crate::prelude::Handle;

impl_handle! { HGLOBAL: "kernel";
	/// Handle to a
	/// [global memory block](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc).
	/// Originally just a `HANDLE`.
}

impl kernel_Hglobal for HGLOBAL {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HGLOBAL`](crate::HGLOBAL).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hglobal: Handle {
	/// [`GlobalAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalFree`](crate::prelude::kernel_Hglobal::GlobalFree)
	/// call.
	#[must_use]
	fn GlobalAlloc(flags: co::GMEM, num_bytes: u64) -> WinResult<HGLOBAL> {
		unsafe { kernel::ffi::GlobalAlloc(flags.0, num_bytes).as_mut() }
			.map(|ptr| HGLOBAL(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalFlags`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalflags)
	/// method.
	#[must_use]
	fn GlobalFlags(self) -> WinResult<co::GMEM> {
		match unsafe { kernel::ffi::GlobalFlags(self.as_ptr()) } {
			GMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(co::GMEM(flags)),
		}
	}

	/// [`GlobalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// method.
	fn GlobalFree(self) -> WinResult<()> {
		match unsafe { kernel::ffi::GlobalFree(self.as_ptr()).as_mut() } {
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		}
	}

	/// [`GlobalLock`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
	/// method.
	///
	/// Calls [`HGLOBAL::GlobalSize`](crate::prelude::kernel_Hglobal::GlobalSize)
	/// to retrieve the size of the memory block.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalUnlock`](crate::prelude::kernel_Hglobal::GlobalUnlock)
	/// call.
	#[must_use]
	fn GlobalLock<'a>(self) -> WinResult<&'a mut [u8]> {
		let mem_sz = self.GlobalSize()?;
		unsafe { kernel::ffi::GlobalLock(self.as_ptr()).as_mut() }
			.map(|ptr| unsafe {
				std::slice::from_raw_parts_mut(ptr as *mut _ as *mut _, mem_sz as _)
			})
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalReAlloc`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalrealloc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalFree`](crate::prelude::kernel_Hglobal::GlobalFree)
	/// call.
	#[must_use]
	fn GlobalReAlloc(self,
		num_bytes: u64, flags: co::GMEM) -> WinResult<HGLOBAL>
	{
		unsafe {
			kernel::ffi::GlobalReAlloc(self.as_ptr(), num_bytes, flags.0).as_mut()
		}.map(|ptr| HGLOBAL(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalSize`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalsize)
	/// method.
	#[must_use]
	fn GlobalSize(self) -> WinResult<u64> {
		match unsafe { kernel::ffi::GlobalSize(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}

	/// [`GlobalUnlock`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// method.
	fn GlobalUnlock(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel::ffi::GlobalUnlock(self.as_ptr()) })
	}
}
