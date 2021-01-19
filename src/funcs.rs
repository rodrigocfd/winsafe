//! Win32 free functions.

#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{comctl32, kernel32, user32};
use crate::handles::{HINSTANCE, HWND};
use crate::privs::{const_void, mut_void, parse_multi_z_str, ptr_as_opt};
use crate::structs as s;
use crate::WString;

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	lpRect: &mut s::RECT, dwStyle: co::WS,
	bMenu: bool, dwExStyle: co::WS_EX) -> WinResult<()>
{
	match unsafe {
		user32::AdjustWindowRectEx(
			mut_void(lpRect), dwStyle.into(), bMenu as i32, dwExStyle.into(),
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
pub fn DispatchMessage(lpMsg: &s::MSG) -> isize {
	unsafe { user32::DispatchMessageW(const_void(lpMsg)) }
}

/// [`GetDialogBaseUnits`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogbaseunits)
/// function.
pub fn GetDialogBaseUnits() -> i32 {
	unsafe { user32::GetDialogBaseUnits() }
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
pub fn GetEnvironmentStrings() -> WinResult<HashMap<String, String>> {
	match ptr_as_opt(unsafe { kernel32::GetEnvironmentStringsW() }) {
		None => Err(GetLastError()),
		Some(p) => {
			let vecEnvStrs = parse_multi_z_str(p as *const u16);
			unsafe { kernel32::FreeEnvironmentStringsW(p); }

			let mut map = HashMap::with_capacity(vecEnvStrs.len());
			for envStr in vecEnvStrs {
				let pair: Vec<&str> = envStr.split("=").collect();
				map.insert(pair[0].to_owned(), pair[1].to_owned());
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
pub fn GetMessage(lpMsg: &mut s::MSG, hWnd: Option<HWND>,
	wMsgFilterMin: u32, wMsgFilterMax: u32) -> WinResult<bool>
{
	match unsafe {
		user32::GetMessageW(
			mut_void(lpMsg),
			match hWnd {
				Some(hWnd) => hWnd.ptr,
				None => std::ptr::null_mut(),
			},
			wMsgFilterMin, wMsgFilterMax,
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

/// [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
pub fn GetSysColor(nIndex: co::COLOR) -> s::COLORREF {
	s::COLORREF(unsafe { user32::GetSysColor(u32::from(nIndex) as i32) })
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

/// Returns the high-order `u32` of an `u64`.
pub fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as u32
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
pub fn IsGUIThread(bConvert: bool) -> WinResult<bool> {
	let r = unsafe { user32::IsGUIThread(bConvert as i32) };
	if bConvert {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(co::ERROR::from(err as u32)),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`IsWindows10OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows10orgreater)
pub fn IsWindows10OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINTHRESHOLD.into()) as u16,
		LOBYTE(co::WIN32::WINNT_WINTHRESHOLD.into()) as u16,
		0,
	)
}

/// [`IsWindows7OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows7orgreater)
pub fn IsWindows7OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN7.into()) as u16,
		LOBYTE(co::WIN32::WINNT_WIN7.into()) as u16,
		0,
	)
}

/// [`IsWindows8OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8orgreater)
/// function.
pub fn IsWindows8OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN8.into()) as u16,
		LOBYTE(co::WIN32::WINNT_WIN8.into()) as u16,
		0,
	)
}

/// [`IsWindows8Point1OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8point1orgreater)
/// function.
pub fn IsWindows8Point1OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINBLUE.into()) as u16,
		LOBYTE(co::WIN32::WINNT_WINBLUE.into()) as u16,
		0,
	)
}

/// [`IsWindowsServer`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsserver)
pub fn IsWindowsServer() -> WinResult<bool> {
	let mut osvi = s::OSVERSIONINFOEX::default();
	osvi.wProductType = co::VER_NT::WORKSTATION;
	let dwlConditionMask = VerSetConditionMask(
		0, co::VER_MASK::PRODUCT_TYPE, co::VER_COND::EQUAL);
	VerifyVersionInfo(&mut osvi, co::VER_MASK::PRODUCT_TYPE, dwlConditionMask)
		.map(|b| !b) // not workstation
}

/// [`IsWindowsVersionOrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsversionorgreater)
/// function.
pub fn IsWindowsVersionOrGreater(
	wMajorVersion: u16, wMinorVersion: u16,
	wServicePackMajor: u16) -> WinResult<bool>
{
	let mut osvi = s::OSVERSIONINFOEX::default();
	let dwlConditionMask = VerSetConditionMask(
		VerSetConditionMask(
			VerSetConditionMask(0, co::VER_MASK::MAJORVERSION, co::VER_COND::GREATER_EQUAL),
			co::VER_MASK::MINORVERSION, co::VER_COND::GREATER_EQUAL,
		),
		co::VER_MASK::SERVICEPACKMAJOR, co::VER_COND::GREATER_EQUAL
	);

	osvi.dwMajorVersion = wMajorVersion as u32;
	osvi.dwMinorVersion = wMinorVersion as u32;
	osvi.wServicePackMajor = wServicePackMajor;

	return VerifyVersionInfo(
		&mut osvi,
		co::VER_MASK::MAJORVERSION | co::VER_MASK::MINORVERSION | co::VER_MASK::SERVICEPACKMAJOR,
		dwlConditionMask,
	)
}

/// [`IsWindowsVistaOrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsvistaorgreater)
/// function.
pub fn IsWindowsVistaOrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_VISTA.into()) as u16,
		LOBYTE(co::WIN32::WINNT_VISTA.into()) as u16,
		0,
	)
}

/// [`LOBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// function. Originally a macro.
pub fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as u8
}

/// [`LockSetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
pub fn LockSetForegroundWindow(uLockCode: co::LSFW) -> WinResult<()> {
	match unsafe { user32::LockSetForegroundWindow(uLockCode.into()) } {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// Returns the low-order `u32` of an `u64`.
pub fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as u32
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

/// [`MulDiv`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
pub fn MulDiv(nNumber: i32, nNumerator: i32, nDenominator: i32) -> i32 {
	unsafe { kernel32::MulDiv(nNumber, nNumerator, nDenominator) }
}

/// [`OutputDebugString`](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)
/// function.
pub fn OutputDebugString(lpOutputString: &str) {
	unsafe {
		kernel32::OutputDebugStringW(WString::from_str(lpOutputString).as_ptr())
	}
}

/// [`PeekMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
pub fn PeekMessage(lpMsg: &mut s::MSG, hWnd: HWND,
	wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: co::PM) -> bool
{
	unsafe {
		user32::PeekMessageW(
			mut_void(lpMsg),
			hWnd.ptr,
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
pub fn RegisterClassEx(lpwcx: &s::WNDCLASSEX) -> WinResult<s::ATOM> {
	match unsafe {
		user32::RegisterClassExW(lpwcx as *const s::WNDCLASSEX as *const _)
	} {
		0 => Err(GetLastError()),
		atom => Ok(s::ATOM(atom)),
	}
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(dwErrCode: co::ERROR) {
	unsafe { kernel32::SetLastError(dwErrCode.into()) }
}

/// [`SetProcessDPIAware`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
pub fn SetProcessDPIAware() -> WinResult<()> {
	match unsafe { user32::SetProcessDPIAware() } {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`Sleep`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(dwMilliseconds: u32) {
	unsafe { kernel32::Sleep(dwMilliseconds) }
}

/// [`SystemParametersInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow)
/// function.
///
/// The `pvParam` type varies according to the `uiAction`. If you set it wrong,
/// you're likely to cause a buffer overrun.
pub unsafe fn SystemParametersInfo<T>(
	uiAction: co::SPI, uiParam: u32,
	pvParam: &mut T, fWinIni: co::SPIF) -> WinResult<()>
{
	match user32::SystemParametersInfoW(
		uiAction.into(), uiParam, mut_void(pvParam), fWinIni.into(),
	) {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(lpMsg: &s::MSG) -> bool {
	unsafe {
		user32::TranslateMessage(const_void(lpMsg)) != 0
	}
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(
	lpClassName: &str, hInstance: HINSTANCE) -> WinResult<()>
{
	match unsafe {
		user32::UnregisterClassW(
			WString::from_str(lpClassName).as_ptr(),
			hInstance.ptr,
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`VerifyVersionInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
pub fn VerifyVersionInfo(
	lpVersionInformation: &mut s::OSVERSIONINFOEX,
	dwTypeMask: co::VER_MASK, dwlConditionMask: u64) -> WinResult<bool>
{
	match unsafe {
		kernel32::VerifyVersionInfoW(
			mut_void(lpVersionInformation), dwTypeMask.into(), dwlConditionMask,
		)
	} {
		0 => match GetLastError() {
			co::ERROR::OLD_WIN_VERSION => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`VerSetConditionMask`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-versetconditionmask)
/// function.
pub fn VerSetConditionMask(
	ConditionMask: u64, TypeMask: co::VER_MASK, Condition: co::VER_COND) -> u64
{
	unsafe {
		kernel32::VerSetConditionMask(
			ConditionMask, TypeMask.into(), Condition.into(),
		)
	}
}
