#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::replace_handle_value;
use crate::prelude::Handle;

/// This trait is enabled with the `gdi` feature, and implements methods for any
/// [`HGDIOBJ`](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj)
/// handle, which is the base handle for
/// [GDI objects](https://learn.microsoft.com/en-us/windows/win32/sysinfo/gdi-objects).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait GdiObject: Handle {
	/// The type returned by
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) for this
	/// `GdiObject`.
	type SelectRet;

	/// Converts the generic pointer, returned from the raw
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) call, into
	/// the specific return type.
	///
	/// # Safety
	///
	/// The object must be the correct type.
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	unsafe fn convert_sel_ret(v: *mut std::ffi::c_void) -> Self::SelectRet;

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
