#![allow(non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::Handle;

impl_handle! { HLOCAL: "kernel";
	/// Handle to a
	/// [local memory block](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal).
}

impl KernelHlocal for HLOCAL {}

/// [`HLOCAL`](crate::HLOCAL) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHlocal: Handle {
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
	fn LocalSize(self) -> WinResult<u64> {
		match unsafe { kernel::ffi::LocalSize(self.as_ptr()) } {
			0 => Err(GetLastError()),
			sz => Ok(sz),
		}
	}
}
