#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{bool_to_sysresult, invalidate_handle};
use crate::prelude::Handle;

impl_handle! { HCURSOR: "user";
	/// Handle to a
	/// [cursor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
}

impl user_Hcursor for HCURSOR {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HCURSOR`](crate::HCURSOR).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Hcursor: Handle {
	/// [`CopyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// method. Originally a macro.
	///
	/// **Note:** Must be paired with an
	/// [`HCURSOR::DestroyCursor`](crate::prelude::user_Hcursor::DestroyCursor)
	/// call.
	#[must_use]
	fn CopyCursor(&self) -> SysResult<HCURSOR> {
		unsafe { user::ffi::CopyIcon(self.as_ptr()).as_mut() }
			.map(|ptr| HCURSOR(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroycursor)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DestroyCursor(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { user::ffi::DestroyCursor(self.as_ptr()) },
		);
		invalidate_handle(self);
		ret
	}

	/// [`SetSystemCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// method.
	fn SetSystemCursor(&self, id: co::OCR) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::SetSystemCursor(self.as_ptr(), id.0) },
		)
	}
}
