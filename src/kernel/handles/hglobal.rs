#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{
	bool_to_sysresult, GMEM_INVALID_HANDLE, replace_handle_value,
};
use crate::prelude::Handle;

impl_handle! { HGLOBAL;
	/// Handle to a
	/// [global memory block](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc).
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
pub trait kernel_Hglobal: Handle {
	/// [`GlobalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc)
	/// static method.
	#[must_use]
	fn GlobalAlloc(
		flags: co::GMEM, num_bytes: usize) -> SysResult<HglobalGuard>
	{
		unsafe { kernel::ffi::GlobalAlloc(flags.0, num_bytes).as_mut() }
			.map(|ptr| HglobalGuard { handle: HGLOBAL(ptr) })
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalflags)
	/// method.
	#[must_use]
	fn GlobalFlags(&self) -> SysResult<co::GMEM> {
		match unsafe { kernel::ffi::GlobalFlags(self.as_ptr()) } {
			GMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(co::GMEM(flags)),
		}
	}

	/// [`GlobalLock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
	/// method.
	///
	/// Calls [`HGLOBAL::GlobalSize`](crate::prelude::kernel_Hglobal::GlobalSize)
	/// to retrieve the size of the memory block.
	///
	/// **Note:** Must be paired with an
	/// [`HGLOBAL::GlobalUnlock`](crate::prelude::kernel_Hglobal::GlobalUnlock)
	/// call.
	#[must_use]
	fn GlobalLock(&self) -> SysResult<&mut [u8]> {
		let mem_sz = self.GlobalSize()?;
		unsafe { kernel::ffi::GlobalLock(self.as_ptr()).as_mut() }
			.map(|ptr| unsafe {
				std::slice::from_raw_parts_mut(ptr as *mut _ as *mut _, mem_sz as _)
			})
			.ok_or_else(|| GetLastError())
	}

	/// [`GlobalReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalrealloc)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	#[must_use]
	fn GlobalReAlloc(&self,
		num_bytes: usize, flags: co::GMEM) -> SysResult<()>
	{
		unsafe {
			kernel::ffi::GlobalReAlloc(self.as_ptr(), num_bytes, flags.0).as_mut()
				.map(|ptr| {
					replace_handle_value(self, Self::from_ptr(ptr));
				})
		}.ok_or_else(|| GetLastError())
	}

	/// [`GlobalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalsize)
	/// method.
	#[must_use]
	fn GlobalSize(&self) -> SysResult<usize> {
		match unsafe { kernel::ffi::GlobalSize(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}

	/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// method.
	fn GlobalUnlock(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { kernel::ffi::GlobalUnlock(self.as_ptr()) })
	}
}

//------------------------------------------------------------------------------

handle_guard! { HglobalGuard: HGLOBAL;
	kernel::ffi::GlobalFree;
	/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) which automatically
	/// calls
	/// [`GlobalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// when the object goes out of scope.
}
