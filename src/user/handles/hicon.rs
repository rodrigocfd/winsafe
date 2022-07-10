#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user;

impl_handle! { HICON: "user";
	/// Handle to an
	/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
}

impl user_Hicon for HICON {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HICON`](crate::HICON).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Hicon: Handle {
	/// [`CopyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HICON::DestroyIcon`](crate::prelude::user_Hicon::DestroyIcon) call.
	#[must_use]
	fn CopyIcon(self) -> WinResult<HICON> {
		unsafe { user::ffi::CopyIcon(self.as_ptr()).as_mut() }
			.map(|ptr| HICON(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	fn DestroyIcon(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::DestroyIcon(self.as_ptr()) })
	}
}
