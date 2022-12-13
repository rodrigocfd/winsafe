#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{bool_to_sysresult, invalidate_handle};
use crate::prelude::Handle;
use crate::user;

impl_handle! { HICON;
	/// Handle to an
	/// [icon](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
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
pub trait user_Hicon: Handle {
	/// [`CopyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HICON::DestroyIcon`](crate::prelude::user_Hicon::DestroyIcon) call.
	#[must_use]
	fn CopyIcon(&self) -> SysResult<HICON> {
		unsafe { user::ffi::CopyIcon(self.as_ptr()).as_mut() }
			.map(|ptr| HICON(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DestroyIcon(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { user::ffi::DestroyIcon(self.as_ptr()) },
		);
		invalidate_handle(self);
		ret
	}
}
