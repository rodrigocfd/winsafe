#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::c_void;

use crate::co;
use crate::com::{PPVtbl, Vtbl};
use crate::ffi::{comctl32, kernel32, ole32, user32};
use crate::handles::{HINSTANCE, HWND};
use crate::internal_defs::parse_multi_z_str;
use crate::structs as s;
use crate::Utf16;

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	lpRect: &mut s::RECT, dwStyle: co::WS,
	bMenu: bool, dwExStyle: co::WS_EX) -> Result<(), co::ERROR>
{
	match unsafe {
		user32::AdjustWindowRectEx(
			lpRect as *mut s::RECT as *mut c_void,
			dwStyle.into(),
			bMenu as u32,
			dwExStyle.into(),
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM interface object.
///
/// # Examples
///
/// Instantiating an [`ITaskbarList`](crate::shell::ITaskbarList) object:
/// ```rust,ignore
/// let mut obj: shell::ITaskbarList = CoCreateInstance(
///   &shell::clsid::TaskbarList,
///   None,
///   co::CLSCTX::INPROC_SERVER,
/// );
/// ```
pub fn CoCreateInstance<VT: Vtbl, IF: From<PPVtbl<VT>>>(
	rclsid: &s::CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX,
) -> IF {
	let mut ppv: PPVtbl<VT> = std::ptr::null_mut();
	unsafe {
		ole32::CoCreateInstance(
			rclsid.as_ref() as *const s::GUID as *const c_void,
			pUnkOuter.unwrap_or(std::ptr::null_mut()),
			dwClsContext.into(),
			VT::IID().as_ref() as *const s::GUID as *const c_void,
			&mut ppv
				as *mut PPVtbl<VT>
				as *mut *mut *mut c_void,
		);
	}
	IF::from(ppv)
}

/// [`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
/// function.
///
/// Must be paired with a [`CoUninitialize`](crate::CoUninitialize) call.
pub fn CoInitializeEx(dwCoInit: co::COINIT) -> Result<co::ERROR, co::ERROR> {
	let err = co::ERROR::from(
		unsafe { ole32::CoInitializeEx(std::ptr::null(), dwCoInit.into()) }
	);
	match err {
		co::ERROR::S_OK
			| co::ERROR::S_FALSE
			| co::ERROR::RPC_E_CHANGED_MODE => Ok(err),
		err => Err(err),
	}
}

/// [`CoUninitialize`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// function.
///
/// Must be called after all COM interfaces have been released.
pub fn CoUninitialize() {
	unsafe { ole32::CoUninitialize() }
}

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
pub fn DispatchMessage(lpMsg: &s::MSG) -> isize {
	unsafe { user32::DispatchMessageW(lpMsg as *const s::MSG as *const c_void) }
}

/// [`GetEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getenvironmentstringsw)
/// function.
///
/// Returns the parsed strings, and automatically frees the retrieved
/// environment block with
/// [`FreeEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-freeenvironmentstringsw).
///
/// # Examples
///
/// Retrieving and printing the key/value pairs of all environment strings:
/// ```rust,ignore
/// let env_vars = GetEnvironmentStrings().unwrap();
/// for (k, v) in env_vars.iter() {
///   println!("{} = {}", k, v);
/// }
/// ```
pub fn GetEnvironmentStrings() -> Result<HashMap<String, String>, co::ERROR> {
	match ptr_as_opt!(kernel32::GetEnvironmentStringsW()) {
		None => Err(GetLastError()),
		Some(p) => {
			let envstrs = parse_multi_z_str(p as *const u16);
			unsafe { kernel32::FreeEnvironmentStringsW(p); }

			let mut map = HashMap::with_capacity(envstrs.len());
			for envstr in envstrs {
				let pair: Vec<&str> = envstr.split("=").collect();
				map.insert(String::from(pair[0]), String::from(pair[1]));
			}
			Ok(map)
		},
	}
}

/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
pub fn GetLastError() -> co::ERROR {
	unsafe { co::ERROR::from(kernel32::GetLastError()) }
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(lpMsg: &s::MSG, hWnd: HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32) -> Result<bool, co::ERROR>
{
	match unsafe {
		user32::GetMessageW(
			lpMsg as *const s::MSG as *const c_void,
			hWnd.as_ptr(),
			wMsgFilterMin,
			wMsgFilterMax,
		)
	} {
		-1 => Err(GetLastError()),
		0 => Ok(false),
		_ => Ok(true),
	}
}

/// [`GetQueueStatus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getqueuestatus)
/// function.
pub fn GetQueueStatus(flags: co::QS) -> u32 {
	unsafe { user32::GetQueueStatus(flags.into()) }
}

/// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
pub fn GetSystemMetrics(nIndex: co::SM) -> i32 {
	unsafe { user32::GetSystemMetrics(nIndex.into()) }
}

/// [`HIBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// function. Originally a macro.
pub fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as u8
}

/// [`HIWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// function. Originally a macro.
pub fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as u16
}

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { comctl32::InitCommonControls() }
}

/// [`IsGUIThread`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
pub fn IsGUIThread(bConvert: bool) -> Result<bool, co::ERROR> {
	let r = unsafe { user32::IsGUIThread(bConvert as u32) };
	if bConvert {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(co::ERROR::from(err)),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`LOBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// function. Originally a macro.
pub fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as u8
}

/// [`LOWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// function. Originally a macro.
pub fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as u16
}

/// Function that implements
/// [`MAKELONG`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
pub fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as u32
}

/// [`MAKEWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// function. Originally a macro.
pub fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`PeekMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
pub fn PeekMessage(lpMsg: &mut s::MSG, hWnd: HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: co::PM) -> bool
{
	unsafe {
		user32::PeekMessageW(
			lpMsg as *mut s::MSG as *mut c_void,
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
pub fn RegisterClassEx(lpwcx: &s::WNDCLASSEX) -> Result<s::ATOM, co::ERROR> {
	match unsafe {
		user32::RegisterClassExW(lpwcx as *const s::WNDCLASSEX as *const c_void)
	} {
		0 => Err(GetLastError()),
		atom => Ok(s::ATOM::from(atom)),
	}
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(dwErrCode: co::ERROR) {
	unsafe { kernel32::SetLastError(dwErrCode.into()) }
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(lpMsg: &s::MSG) -> bool {
	unsafe {
		user32::TranslateMessage(lpMsg as *const s::MSG as *const c_void) != 0
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