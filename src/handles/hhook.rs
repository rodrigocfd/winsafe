#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::aliases::HOOKPROC;
use crate::co;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HINSTANCE;
use crate::internal_defs::ptr_as_opt;

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
		hmod: HINSTANCE, dwThreadId: u32) -> Result<HHOOK, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::SetWindowsHookExW(
					idHook.into(),
					lpfn as *const c_void,
					hmod.as_ptr(),
					dwThreadId,
				)
			}
		) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}
}