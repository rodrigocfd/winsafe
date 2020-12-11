#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::c_void;

use crate::{ATOM, CLSID, GUID, MSG, WNDCLASSEX};
use crate::{HINSTANCE, HWND, RECT};
use crate::{PPVtbl, Vtbl};
use crate::co;
use crate::ffi::{advapi32, comctl32, kernel32, ole32, user32};
use crate::internal_defs;
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
pub fn CoCreateInstance<VT: Vtbl, IF: From<PPVtbl<VT>>>(
	rclsid: &CLSID,
	pUnkOuter: Option<*mut c_void>,
	dwClsContext: co::CLSCTX,
) -> IF {
	let mut ppv: PPVtbl<VT> = std::ptr::null_mut();
	unsafe {
		ole32::CoCreateInstance(
			rclsid.as_ref() as *const GUID as *const c_void,
			pUnkOuter.unwrap_or(std::ptr::null_mut()),
			dwClsContext.into(),
			VT::IID().as_ref() as *const GUID as *const c_void,
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
pub fn DispatchMessage(lpMsg: &MSG) -> isize {
	unsafe { user32::DispatchMessageW(lpMsg as *const MSG as *const c_void) }
}

/// [`GetComputerName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
pub fn GetComputerName() -> Result<String, co::ERROR> {
	let mut buf = Utf16::new_alloc_buffer(internal_defs::MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz: u32 = buf.buffer_size() as u32;

	match unsafe { kernel32::GetComputerNameW(buf.as_mut_ptr(), &mut sz) } {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-freeenvironmentstringsw)
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
			let envstrs = internal_defs::parse_multi_z_str(p as *const u16);
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

/// [`GetUserName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
pub fn GetUserName() -> Result<String, co::ERROR> {
	let mut buf = Utf16::new_alloc_buffer(internal_defs::UNLEN + 1);
	let mut sz: u32 = buf.buffer_size() as u32;

	match unsafe { advapi32::GetUserNameW(buf.as_mut_ptr(), &mut sz) } {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
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