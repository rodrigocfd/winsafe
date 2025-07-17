#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

handle! { HHOOK;
	/// Handle to a
	/// [hook](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hhook).
}

impl HHOOK {
	/// [`CallNextHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex)
	/// function.
	pub fn CallNextHookEx(&self, code: co::WH, wparam: usize, lparam: isize) -> isize {
		unsafe { ffi::CallNextHookEx(self.ptr(), code.raw(), wparam, lparam) }
	}

	/// [`SetWindowsHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
	/// function.
	pub fn SetWindowsHookEx(
		hook_id: co::WH,
		proc: HOOKPROC,
		module: Option<&HINSTANCE>,
		thread_id: Option<u32>,
	) -> SysResult<HHOOK> {
		PtrRet(unsafe {
			ffi::SetWindowsHookExW(
				hook_id.raw(),
				proc as _,
				module.map_or(std::ptr::null_mut(), |h| h.ptr()),
				thread_id.unwrap_or_default(),
			)
		})
		.to_sysresult_handle()
	}

	/// [`UnhookWindowsHookEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex)
	/// function.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	pub fn UnhookWindowsHookEx(&mut self) -> SysResult<()> {
		let ret = BoolRet(unsafe { ffi::UnhookWindowsHookEx(self.ptr()) }).to_sysresult();
		*self = Self::INVALID;
		ret
	}
}
