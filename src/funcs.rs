#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ATOM, CLSID, GUID, MSG, WNDCLASSEX};
use crate::{HINSTANCE, HWND, RECT};
use crate::co;
use crate::ComVtbl;
use crate::ffi::{comctl32, kernel32, ole32, user32};
use crate::Utf16;

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	lpRect: &mut RECT, dwStyle: co::WS,
	bMenu: bool, dwExStyle: co::WS_EX) -> Result<(), co::ERROR>
{
	match unsafe {
		user32::AdjustWindowRectEx(
			lpRect as *mut RECT as *mut c_void,
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
pub fn CoCreateInstance<VT: ComVtbl, IF: From<*const *const VT>>(
	rclsid: &CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX,
) -> IF {
	let mut ppv: *const *const VT = std::ptr::null();
	unsafe {
		ole32::CoCreateInstance(
			rclsid.as_ref() as *const GUID as *const c_void,
			pUnkOuter.unwrap_or(std::ptr::null_mut()),
			dwClsContext.into(),
			VT::IID().as_ref() as *const GUID as *const c_void,
			&mut ppv
				as *mut *const *const VT
				as *mut *const *const c_void,
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
pub fn DispatchMessage(lpMsg: &MSG) -> isize {
	unsafe { user32::DispatchMessageW(lpMsg as *const MSG as *const c_void) }
}

/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
pub fn GetLastError() -> co::ERROR {
	unsafe { co::ERROR::from(kernel32::GetLastError()) }
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(lpMsg: &MSG, hWnd: HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32) -> Result<bool, co::ERROR>
{
	match unsafe {
		user32::GetMessageW(
			lpMsg as *const MSG as *const c_void,
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

/// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
pub fn GetSystemMetrics(nIndex: co::SM) -> i32 {
	unsafe { user32::GetSystemMetrics(nIndex.into()) }
}

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { comctl32::InitCommonControls() }
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