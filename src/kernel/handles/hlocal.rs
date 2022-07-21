#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::Handle;

impl_handle! { HLOCAL: "kernel";
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hlocal: Handle {
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// method.
	fn LocalFree(self) -> WinResult<()> {
		match unsafe { kernel::ffi::LocalFree(self.as_ptr()).as_mut() } {
			None => Ok(()),
			Some(_) => Err(GetLastError()),
		}
	}

	/// [`LocalSize`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localsize)
	/// method.
	#[must_use]
	fn LocalSize(self) -> WinResult<usize> {
		match unsafe { kernel::ffi::LocalSize(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
