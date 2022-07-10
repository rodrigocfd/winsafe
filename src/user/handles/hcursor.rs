#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;

impl_handle! { HCURSOR: "user";
	/// Handle to a
	/// [cursor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
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
	/// [`CopyCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// method. Originally a macro.
	///
	/// **Note:** Must be paired with an
	/// [`HCURSOR::DestroyCursor`](crate::prelude::user_Hcursor::DestroyCursor)
	/// call.
	#[must_use]
	fn CopyCursor(self) -> WinResult<HCURSOR> {
		unsafe { user::ffi::CopyIcon(self.as_ptr()).as_mut() }
			.map(|ptr| HCURSOR(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroycursor)
	/// method.
	fn DestroyCursor(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::DestroyCursor(self.as_ptr()) })
	}

	/// [`SetSystemCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsystemcursor)
	/// method.
	fn SetSystemCursor(self, id: co::OCR) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::SetSystemCursor(self.as_ptr(), id.0) },
		)
	}
}
