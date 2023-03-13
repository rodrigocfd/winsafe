#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::guard::LocalFreeGuard;
use crate::kernel::privs::{LMEM_INVALID_HANDLE, ptr_to_sysresult};
use crate::prelude::Handle;

impl_handle! { HLOCAL;
	/// Handle to a
	/// [local memory block](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
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
		ptr_to_sysresult(
			unsafe { kernel::ffi::LocalAlloc(flags.0, num_bytes) },
			|ptr| LocalFreeGuard::new(HLOCAL(ptr)),
		)
	}

	/// [`LocalFlags`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localflags)
	/// method.
	#[must_use]
	fn LocalFlags(&self) -> SysResult<co::LMEM> {
		match unsafe { kernel::ffi::LocalFlags(self.as_ptr()) } {
			LMEM_INVALID_HANDLE => Err(GetLastError()),
			flags => Ok(co::LMEM(flags)),
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
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::LocalReAlloc(self.as_ptr(), num_bytes, flags.0)
			},
			|ptr| { *self = unsafe { Self::from_ptr(ptr) } },
		)
	}

	/// [`LocalSize`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	#[must_use]
	fn LocalSize(&self) -> SysResult<usize> {
		match unsafe { kernel::ffi::LocalSize(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
