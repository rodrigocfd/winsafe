#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{GetLastError, SysResult};
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
	/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn LocalFree(&mut self) -> SysResult<()> {
		let ret = match unsafe {
			kernel::ffi::LocalFree(self.as_ptr()).as_mut() }
		{
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		};
		*self = Self::INVALID;
		ret
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
