#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::ffi::{kernel32, user32};
use crate::funcs::GetLastError;
use crate::handles::{HINSTANCE, HWND};
use crate::structs::{ATOM, MSG, WNDCLASSEX};
use crate::Utf16;

/// [`PeekMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function
pub fn PeekMessage(lpMsg: &mut MSG, hWnd: HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: co::PM) -> bool
{
	unsafe {
		user32::PeekMessageW(
			lpMsg as *mut MSG as *mut c_void,
			hWnd.as_ptr(),
			wMsgFilterMin,
			wMsgFilterMax,
			wRemoveMsg.into(),
		) != 0
	}
}

/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
pub fn PostQuitMessage(nExitCode: i32) {
	unsafe { user32::PostQuitMessage(nExitCode) }
}

/// [`RegisterClassEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
pub fn RegisterClassEx(lpwcx: &WNDCLASSEX) -> Result<ATOM, co::ERROR> {
	match unsafe {
		user32::RegisterClassExW(lpwcx as *const WNDCLASSEX as *const c_void)
	} {
		0 => Err(GetLastError()),
		atom => Ok(ATOM::from(atom)),
	}
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(dwErrCode: co::ERROR) {
	unsafe { kernel32::SetLastError(dwErrCode.into()) }
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(lpMsg: &MSG) -> bool {
	unsafe {
		user32::TranslateMessage(lpMsg as *const MSG as *const c_void) != 0
	}
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(
	lpClassName: &str, hInstance: HINSTANCE) -> Result<(), co::ERROR>
{
	match unsafe {
		user32::UnregisterClassW(
			Utf16::from_str(lpClassName).as_ptr(),
			hInstance.as_ptr(),
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}