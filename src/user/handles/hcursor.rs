#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

handle! { HCURSOR;
	/// Handle to a
	/// [cursor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
}

impl user_Hcursor for HCURSOR {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HCURSOR`](crate::HCURSOR).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hcursor: Handle {
	/// [`CopyCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// macro.
	#[must_use]
	fn CopyCursor(&self) -> SysResult<DestroyCursorGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::CopyIcon(self.ptr())).map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`GetCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursor)
	/// function.
	#[must_use]
	fn GetCursor() -> Option<HCURSOR> {
		ptr_to_option_handle(unsafe { ffi::GetCursor() })
	}

	/// [`SetSystemCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// function.
	fn SetSystemCursor(&self, id: co::OCR) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetSystemCursor(self.ptr(), id.raw()) })
	}
}
