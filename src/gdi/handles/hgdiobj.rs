#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::prelude::Handle;

/// Any
/// [`HGDIOBJ`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is the base handle for
/// [GDI objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
///
/// This trait is enabled with the `gdi` feature.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait gdi_Hgdiobj: Handle {
	/// [`DeleteObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// method.
	fn DeleteObject(self) -> SysResult<()> {
		match unsafe { gdi::ffi::DeleteObject(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()), // not really an error
				err => Err(err),
			},
			_ => Ok(()),
		}
	}
}
