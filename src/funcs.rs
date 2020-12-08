#![allow(non_snake_case)]

use std::ffi::c_void;

use crate as w;
use crate::co;
use crate::ffi::{ole32, user32};

/// [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
/// function.
///
/// Returns an [`IUnknown`](crate::IUnknown)-derived COM interface object.
pub fn CoCreateInstance<V: w::ComVtbl, I: From<*const *const V>>(
	rclsid: &w::CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX,
) -> I {
	let mut ppv: *const *const V = std::ptr::null();
	unsafe {
		ole32::CoCreateInstance(
			rclsid.as_ref() as *const w::GUID as *const c_void,
			pUnkOuter.unwrap_or(std::ptr::null_mut()),
			dwClsContext.into(),
			V::IID().as_ref() as *const w::GUID as *const c_void,
			&mut ppv
				as *mut *const *const V
				as *mut *const *const c_void,
		);
	}
	I::from(ppv)
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

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessage)
/// function.
pub fn DispatchMessage(lpMsg: &w::MSG) -> isize {
	unsafe { user32::DispatchMessage(lpMsg as *const w::MSG as *const c_void) }
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(lpMsg: &w::MSG, hWnd: w::HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32) -> Result<bool, co::ERROR>
{
	match unsafe {
		user32::GetMessageW(
			lpMsg as *const w::MSG as *const c_void,
			hWnd.as_ptr(),
			wMsgFilterMin,
			wMsgFilterMax,
		)
	} {
		-1 => Err(co::ERROR::GetLastError()),
		0 => Ok(false),
		_ => Ok(true),
	}
}

/// [`RegisterClassEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
pub fn RegisterClassEx(wcx: &w::WNDCLASSEX) -> Result<w::ATOM, co::ERROR> {
	match unsafe {
		user32::RegisterClassExW(wcx as *const w::WNDCLASSEX as *const c_void)
	} {
		0 => Err(co::ERROR::GetLastError()),
		atom => Ok(w::ATOM::from(atom)),
	}
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(lpMsg: &w::MSG) -> bool {
	unsafe {
		user32::TranslateMessage(lpMsg as *const w::MSG as *const c_void) != 0
	}
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(
	lpClassName: &str, hInstance: w::HINSTANCE) -> Result<(), co::ERROR>
{
	match unsafe {
		user32::UnregisterClassW(
			w::Utf16::from_str(lpClassName).as_ptr(),
			hInstance.as_ptr(),
		)
	} {
		0 => Err(co::ERROR::GetLastError()),
		_ => Ok(()),
	}
}