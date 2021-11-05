#![allow(non_snake_case)]

use crate::aliases::{HOOKPROC, WinResult};
use crate::co;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HINSTANCE;
use crate::privs::bool_to_winresult;

/// Handle to a
/// [hook](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hhook).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HHOOK(pub(crate) *mut std::ffi::c_void);

impl_handle!(HHOOK);

impl HHOOK {
	/// [`CallNextHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex)
	/// method.
	pub fn CallNextHookEx(self,
		code: co::WH, wparam: usize, lparam: isize) -> isize
	{
		unsafe { user32::CallNextHookEx(self.0, code.0, wparam, lparam) }
	}

	/// [`SetWindowsHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
	/// static method.
	pub fn SetWindowsHookEx(
		hook_id: co::WH, proc: HOOKPROC,
		module: Option<HINSTANCE>, thread_id: Option<u32>) -> WinResult<HHOOK>
	{
		unsafe {
			user32::SetWindowsHookExW(
				hook_id.0,
				proc as _,
				module.map_or(std::ptr::null_mut(), |h| h.0),
				thread_id.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`UnhookWindowsHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex)
	/// method.
	pub fn UnhookWindowsHookEx(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::UnhookWindowsHookEx(self.0) })
	}
}
