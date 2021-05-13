#![allow(non_snake_case)]

use crate::aliases::{HOOKPROC, WinResult};
use crate::co;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HINSTANCE;
use crate::privs::bool_to_winresult;

pub_struct_handle! {
	/// Handle to a
	/// [hook](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hhook).
	HHOOK
}

impl HHOOK {
	/// [`CallNextHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-callnexthookex)
	/// method.
	pub fn CallNextHookEx(self,
		nCode: co::WH, wParam: usize, lParam: isize) -> isize
	{
		unsafe { user32::CallNextHookEx(self.ptr, nCode.0, wParam, lParam) }
	}

	/// [`SetWindowsHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
	/// static method.
	pub fn SetWindowsHookEx(idHook: co::WH, lpfn: HOOKPROC,
		hmod: Option<HINSTANCE>, dwThreadId: Option<u32>) -> WinResult<HHOOK>
	{
		unsafe {
			user32::SetWindowsHookExW(
				idHook.0,
				lpfn as _,
				hmod.map_or(std::ptr::null_mut(), |h| h.ptr),
				dwThreadId.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`UnhookWindowsHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwindowshookex)
	/// method.
	pub fn UnhookWindowsHookEx(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::UnhookWindowsHookEx(self.ptr) })
	}
}
