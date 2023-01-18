#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{HINSTANCE, SysResult};
use crate::kernel::privs::{
	bool_to_sysresult, ptr_to_sysresult, replace_handle_value,
};
use crate::prelude::Handle;
use crate::user::decl::HOOKPROC;

impl_handle! { HHOOK;
	/// Handle to a
	/// [hook](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hhook).
}

impl user_Hhook for HHOOK {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HHOOK`](crate::HHOOK).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hhook: Handle {
	/// [`CallNextHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex)
	/// method.
	fn CallNextHookEx(&self,
		code: co::WH, wparam: usize, lparam: isize) -> isize
	{
		unsafe {
			user::ffi::CallNextHookEx(self.as_ptr(), code.0, wparam, lparam)
		}
	}

	/// [`SetWindowsHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
	/// static method.
	fn SetWindowsHookEx(
		hook_id: co::WH,
		proc: HOOKPROC,
		module: Option<&HINSTANCE>,
		thread_id: Option<u32>) -> SysResult<HHOOK>
	{
		ptr_to_sysresult(
			unsafe {
				user::ffi::SetWindowsHookExW(
					hook_id.0,
					proc as _,
					module.map_or(std::ptr::null_mut(), |h| h.0),
					thread_id.unwrap_or_default(),
				)
			},
			|ptr| HHOOK(ptr),
		)
	}

	/// [`UnhookWindowsHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn UnhookWindowsHookEx(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { user::ffi::UnhookWindowsHookEx(self.as_ptr()) },
		);
		replace_handle_value(self, Self::INVALID);
		ret
	}
}
