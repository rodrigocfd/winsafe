#![allow(non_snake_case)]

use crate::co;
use crate::gdi;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::Handle;

/// Any
/// [`HGDIOBJ`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is a specific handle for
/// [GDI objects](https://docs.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait HandleGdi: Handle {
	/// [`DeleteObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// method.
	fn DeleteObject(self) -> WinResult<()> {
		match unsafe { gdi::ffi::DeleteObject(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()), // not really an error
				err => Err(err),
			},
			_ => Ok(()),
		}
	}
}
