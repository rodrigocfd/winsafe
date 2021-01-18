#![allow(non_snake_case)]

use crate::aliases::{HOOKPROC, WinResult};
use crate::co;
use crate::ffi::user32;
use crate::funcs_priv::ptr_as_opt;
use crate::funcs::GetLastError;
use crate::handles::HINSTANCE;

handle_type! {
	/// Handle to a
	/// [hook](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hhook).
	/// Exposes methods.
	HHOOK
}

impl HHOOK {
	/// [`SetWindowsHookEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw)
	/// static method.
	pub fn SetWindowsHookEx(idHook: co::WH, lpfn: HOOKPROC,
		hmod: HINSTANCE, dwThreadId: u32) -> WinResult<HHOOK>
	{
		match ptr_as_opt(
			unsafe {
				user32::SetWindowsHookExW(
					idHook.into(),
					lpfn as *const _,
					hmod.ptr,
					dwThreadId,
				)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}
}
