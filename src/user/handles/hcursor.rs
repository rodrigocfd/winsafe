#![allow(non_snake_case)]

use crate::co;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user;

impl_handle! { HCURSOR: "user";
	/// Handle to a
	/// [cursor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hcursor).
}

impl UserHcursor for HCURSOR {}

/// [`HCURSOR`](crate::HCURSOR) methods from `user` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait UserHcursor: Handle {
	/// [`CopyCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copycursor)
	/// method. Originally a macro.
	///
	/// **Note:** Must be paired with an
	/// [`HCURSOR::DestroyCursor`](crate::prelude::UserHcursor::DestroyCursor)
	/// call.
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
