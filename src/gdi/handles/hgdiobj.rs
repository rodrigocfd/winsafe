#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::replace_handle_value;
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
pub trait gdi_Hgdiobj: Handle {
	/// [`DeleteObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DeleteObject(&self) -> SysResult<()> {
		let ret = match unsafe { gdi::ffi::DeleteObject(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()), // not really an error
				err => Err(err),
			},
			_ => Ok(()),
		};
		replace_handle_value(self, Self::INVALID);
		ret
	}
}
