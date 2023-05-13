#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::guard::{LocalFreeGuard, LocalUnlockGuard};
use crate::kernel::privs::{
	LMEM_INVALID_HANDLE, ptr_to_sysresult, ptr_to_sysresult_handle,
};
use crate::prelude::Handle;

impl_handle! { HLOCAL;
	/// Handle to a
	/// [local memory block](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
	///
	/// The allocated memory block is accessible through the
	/// [`LocalLock`](crate::prelude::kernel_Hlocal::LocalLock) method.
}

impl kernel_Hlocal for HLOCAL {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HLOCAL`](crate::HLOCAL).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hlocal: Handle {
	/// [`LocalAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localalloc)
	/// static method.
	#[must_use]
	fn LocalAlloc(
		flags: co::LMEM, num_bytes: usize) -> SysResult<LocalFreeGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::LocalAlloc(flags.raw(), num_bytes),
			).map(|h| LocalFreeGuard::new(h))
		}
	}

	/// [`LocalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localflags)
	/// method.
	#[must_use]
	fn LocalFlags(&self) -> SysResult<co::LMEM> {
		match unsafe { kernel::ffi::LocalFlags(self.ptr()) } {
			LMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(unsafe { co::LMEM::from_raw(flags) }),
		}
	}

	/// [`LocalLock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-locallock)
	/// method.
	///
	/// Calls
	/// [`HLOCAL::LocalSize`](crate::prelude::kernel_Hlocal::LocalSize) to
	/// retrieve the size of the memory block.
	///
	/// Note that this method returns two objects: a reference to the memory
	/// block, and a guard which will call
	/// [`LocalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localunlock)
	/// automatically when the object goes out of scope, so keep the guard
	/// alive.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HLOCAL};
	///
	/// let hlocal = HLOCAL::LocalAlloc(
	///     co::LMEM::FIXED | co::LMEM::ZEROINIT,
	///     120,
	/// )?;
	///
	/// let (block, _guard) = hlocal.LocalLock()?;
	///
	/// block[0] = 40;
	///
	/// // LocalUnlock() called automatically
	///
	/// // LocalFree() called automatically
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LocalLock(&self) -> SysResult<(&mut [u8], LocalUnlockGuard<'_, Self>)> {
		let mem_sz = self.LocalSize()?;
		unsafe {
			ptr_to_sysresult(kernel::ffi::LocalLock(self.ptr()))
				.map(|ptr| (
					std::slice::from_raw_parts_mut(
						ptr as *mut _ as *mut _, mem_sz as _),
					LocalUnlockGuard::new(self),
				),
			)
		}
	}

	/// [`LocalReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localrealloc)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn LocalReAlloc(&mut self,
		num_bytes: usize, flags: co::LMEM) -> SysResult<()>
	{
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::LocalReAlloc(self.ptr(), num_bytes, flags.raw())
			},
		).map(|h| { *self = h; })
	}

	/// [`LocalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	#[must_use]
	fn LocalSize(&self) -> SysResult<usize> {
		match unsafe { kernel::ffi::LocalSize(self.ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
