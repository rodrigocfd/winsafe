#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::SysResult;
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
use crate::prelude::Handle;
use crate::user::guard::DestroyCursorGuard;

impl_handle! { HCURSOR;
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
pub trait user_Hcursor: Handle {
	/// [`CopyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// method. Originally a macro.
	#[must_use]
	fn CopyCursor(&self) -> SysResult<DestroyCursorGuard> {
		unsafe {
			ptr_to_sysresult_handle(user::ffi::CopyIcon(self.as_ptr()))
				.map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`SetSystemCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// method.
	fn SetSystemCursor(&self, id: co::OCR) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::SetSystemCursor(self.as_ptr(), id.0) },
		)
	}
}
