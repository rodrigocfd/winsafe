#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::guard::{GlobalFreeGuard, GlobalUnlockGuard};
use crate::kernel::privs::{
	GMEM_INVALID_HANDLE, ptr_to_sysresult, ptr_to_sysresult_handle,
};
use crate::prelude::Handle;

impl_handle! { HGLOBAL;
	/// Handle to a
	/// [global memory block](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalalloc).
	/// Originally just a `HANDLE`.
	///
	/// The allocated memory block is accessible through the
	/// [`GlobalLock`](crate::prelude::kernel_Hglobal::GlobalLock) method.
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
		flags: Option<co::GMEM>, num_bytes: usize) -> SysResult<GlobalFreeGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::GlobalAlloc(flags.unwrap_or_default().raw(), num_bytes),
			).map(|h| GlobalFreeGuard::new(h))
		}
	}

	/// [`GlobalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalflags)
	/// method.
	#[must_use]
	fn GlobalFlags(&self) -> SysResult<co::GMEM> {
		match unsafe { kernel::ffi::GlobalFlags(self.ptr()) } {
			GMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(unsafe { co::GMEM::from_raw(flags) }),
		}
	}

	/// [`GlobalLock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globallock)
	/// method.
	///
	/// Calls
	/// [`HGLOBAL::GlobalSize`](crate::prelude::kernel_Hglobal::GlobalSize) to
	/// retrieve the size of the memory block.
	///
	/// Note that this method returns two objects: a reference to the memory
	/// block, and a guard which will call
	/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// automatically when the object goes out of scope, so keep the guard
	/// alive.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HGLOBAL};
	///
	/// let hglobal = HGLOBAL::GlobalAlloc(
	///     Some(co::GMEM::FIXED | co::GMEM::ZEROINIT),
	///     120,
	/// )?;
	///
	/// let (block, _guard) = hglobal.GlobalLock()?;
	///
	/// block[0] = 40;
	///
	/// // GlobalUnlock() called automatically
	///
	/// // GlobalFree() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn GlobalLock(&self) -> SysResult<(&mut [u8], GlobalUnlockGuard<'_, Self>)> {
		let mem_sz = self.GlobalSize()?;
		unsafe {
			ptr_to_sysresult(kernel::ffi::GlobalLock(self.ptr()))
				.map(|ptr| (
					std::slice::from_raw_parts_mut(ptr.cast(), mem_sz as _),
					GlobalUnlockGuard::new(self),
				),
			)
		}
	}

	/// [`GlobalReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalrealloc)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn GlobalReAlloc(&mut self,
		num_bytes: usize, flags: Option<co::GMEM>) -> SysResult<()>
	{
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::GlobalReAlloc(
					self.ptr(),
					num_bytes,
					flags.unwrap_or_default().raw(),
				)
			},
		).map(|h| { *self = h; })
	}

	/// [`GlobalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalsize)
	/// method.
	#[must_use]
	fn GlobalSize(&self) -> SysResult<usize> {
		match unsafe { kernel::ffi::GlobalSize(self.ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
