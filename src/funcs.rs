//! Win32 free functions.

#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{advapi32, BOOL, comctl32, comdlg32, HRESULT, kernel32, user32};
use crate::handles::{HINSTANCE, HWND};
use crate::privs::{
	bool_to_winresult,
	INVALID_FILE_ATTRIBUTES,
	MAX_PATH,
	parse_multi_z_str,
};
use crate::structs::{
	ATOM,
	CHOOSECOLOR,
	COLORREF,
	FILETIME,
	MEMORYSTATUSEX,
	MSG,
	OSVERSIONINFOEX,
	POINT,
	RECT,
	SYSTEMTIME,
	TIME_ZONE_INFORMATION,
	TRACKMOUSEEVENT,
	WNDCLASSEX,
};
use crate::WString;

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	lpRect: &mut RECT, dwStyle: co::WS,
	bMenu: bool, dwExStyle: co::WS_EX) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user32::AdjustWindowRectEx(
				lpRect as *mut _ as _,
				dwStyle.0,
				bMenu as _,
				dwExStyle.0,
			)
		},
	)
}

/// [`ChooseColor`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms646912(v=vs.85))
/// function.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, ChooseColor, CHOOSECOLOR};
///
/// let parent_hwnd: HWND; // initialized somewhere
///
/// let mut cc = CHOOSECOLOR::default();
/// let mut custom_colors = [COLORREF::new(255, 255, 255); 16];
///
/// cc.hwndOwner = parent_hwnd;
/// cc.Flags = co::CC::ANYCOLOR | co::CC::FULLOPEN | co::CC::RGBINIT;
/// cc.rgbResult = COLORREF::new(255, 0, 0); // color initially chosen
/// cc.set_lpCustColors(&mut custom_colors);
///
/// if ChooseColor(&mut cc).unwrap() {
///     println!("The color: {} {} {}",
///         cc.rgbResult.GetRValue(),
///         cc.rgbResult.GetGValue(),
///         cc.rgbResult.GetBValue(),
///     );
/// }
/// ```
pub fn ChooseColor(lpcc: &mut CHOOSECOLOR) -> WinResult<bool> {
	match unsafe { comdlg32::ChooseColorW(lpcc as *mut _ as _) } {
		0 => match CommDlgExtendedError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`ClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clipcursor)
/// method.
pub fn ClipCursor(lpRect: Option<&RECT>) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			user32::ClipCursor(
				lpRect.map_or(std::ptr::null(), |lp| lp as *const _ as _),
			)
		},
	)
}

/// [`CloseClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// function.
pub fn CloseClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::CloseClipboard() })
}

/// [`CommDlgExtendedError`](https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror)
/// function.
///
/// **Note:** The [`co::ERROR`](crate::co::ERROR) returned by this function
/// cannot be properly formatted by
/// [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) and
/// [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) traits,
/// thus showing a wrong description. The formatted text actually will belong to
/// the
/// [standard error code](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes)
/// of the same value. Do not rely on these descriptions.
pub fn CommDlgExtendedError() -> co::ERROR {
	co::ERROR(unsafe { comdlg32::CommDlgExtendedError() })
}

/// [`CopyFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
pub fn CopyFile(
	lpExistingFileName: &str, lpNewFileName: &str,
	bFailIfExists: bool) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::CopyFileW(
				WString::from_str(lpExistingFileName).as_ptr(),
				WString::from_str(lpNewFileName).as_ptr(),
				bFailIfExists as _,
			)
		},
	)
}

/// [`DecryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
pub fn DecryptFile(lpFileName: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::DecryptFileW(WString::from_str(lpFileName).as_ptr(), 0)
		},
	)
}

/// [`DeleteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
pub fn DeleteFile(lpFileName: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe { kernel32::DeleteFileW(WString::from_str(lpFileName).as_ptr()) },
	)
}

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
pub fn DispatchMessage(lpMsg: &MSG) -> isize {
	unsafe { user32::DispatchMessageW(lpMsg as *const _ as _) }
}

/// [`EmptyClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
/// function.
pub fn EmptyClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::EmptyClipboard() })
}

/// [`EncryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
pub fn EncryptFile(lpFileName: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::EncryptFileW(WString::from_str(lpFileName).as_ptr())
		},
	)
}

/// [`EncryptionDisable`](https://docs.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
pub fn EncryptionDisable(DirPath: &str, Disable: bool) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::EncryptionDisable(
				WString::from_str(DirPath).as_ptr(),
				Disable as _,
			)
		},
	)
}

/// [`ExpandEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
/// function.
pub fn ExpandEnvironmentStrings(lpSrc: &str) -> WinResult<String> {
	let wsrc = WString::from_str(lpSrc);
	let len = unsafe {
		kernel32::ExpandEnvironmentStringsW(
			wsrc.as_ptr(),
			std::ptr::null_mut(),
			0,
		)
	};

	let mut buf = WString::new_alloc_buffer(len as _);
	match unsafe {
		kernel32::ExpandEnvironmentStringsW(
			wsrc.as_ptr(),
			buf.as_mut_ptr(),
			len,
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`FileTimeToSystemTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-filetimetosystemtime)
/// function.
pub fn FileTimeToSystemTime(
	lpFileTime: &FILETIME, lpSystemTime: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::FileTimeToSystemTime(
				lpFileTime as *const _ as _,
				lpSystemTime as *mut _ as _,
			)
		},
	)
}

/// [`GetAsyncKeyState`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate)
/// function.
pub fn GetAsyncKeyState(vKey: co::VK) -> bool {
	unsafe { user32::GetAsyncKeyState(vKey.0 as _) != 0 }
}

/// [`GetBinaryType`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
/// function.
pub fn GetBinaryType(lpApplicationName: &str) -> WinResult<co::SCS> {
	let mut lpBinaryType = co::SCS::W_32BIT_BINARY;
	bool_to_winresult(
		unsafe {
			kernel32::GetBinaryTypeW(
				WString::from_str(lpApplicationName).as_ptr(),
				&mut lpBinaryType.0,
			)
		}
	).map(|_| lpBinaryType)
}

/// [`GetClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipcursor)
/// method.
pub fn GetClipCursor(lpRect: &mut RECT) -> WinResult<()> {
	bool_to_winresult(unsafe { user32::GetClipCursor(lpRect as *mut _ as _) })
}

/// [`GetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
/// function.
pub fn GetCursorPos() -> WinResult<POINT> {
	let mut lpPoint = POINT::default();
	bool_to_winresult(
		unsafe { user32::GetCursorPos(&mut lpPoint as *mut _ as _) },
	).map(|_| lpPoint)
}

/// [`GetDialogBaseUnits`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogbaseunits)
/// function.
pub fn GetDialogBaseUnits() -> i32 {
	unsafe { user32::GetDialogBaseUnits() }
}

/// [`GetDoubleClickTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdoubleclicktime)
/// function.
pub fn GetDoubleClickTime() -> u32 {
	unsafe { user32::GetDoubleClickTime() }
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
///
/// ```rust,ignore
/// use winsafe::GetEnvironmentStrings;
///
/// let env_vars = GetEnvironmentStrings().unwrap();
/// for (k, v) in env_vars.iter() {
///     println!("{} = {}", k, v);
/// }
/// ```
pub fn GetEnvironmentStrings() -> WinResult<HashMap<String, String>> {
	unsafe { kernel32::GetEnvironmentStringsW().as_mut() }
		.map(|ptr| {
			let vecEnvStrs = parse_multi_z_str(ptr as *mut _ as _);
			unsafe { kernel32::FreeEnvironmentStringsW(ptr); }

			let mut map = HashMap::with_capacity(vecEnvStrs.len());
			for envStr in vecEnvStrs {
				let pair: Vec<&str> = envStr.split("=").collect();
				map.insert(pair[0].to_owned(), pair[1].to_owned());
			}
			map
		})
		.ok_or_else(|| GetLastError())
}

/// [`GetFileAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
/// method.
///
/// # Examples
///
/// Checking whether a file or folder exists:
///
/// ```rust,ignore
/// use winsafe::{co, GetFileAttributes};
///
/// let file_exists = GetFileAttributes("C:\\Temp\\test.txt").is_ok();
/// ```
///
/// Retrieving various information about a file or folder path:
///
/// ```rust,ignore
/// use winsafe::{co, GetFileAttributes};
///
/// let flags = GetFileAttributes("C:\\Temp\\test.txt").unwrap();
///
/// let is_compressed = flags.has(co::FILE_ATTRIBUTE::COMPRESSED);
/// let is_directory  = flags.has(co::FILE_ATTRIBUTE::DIRECTORY);
/// let is_encrypted  = flags.has(co::FILE_ATTRIBUTE::ENCRYPTED);
/// let is_hidden     = flags.has(co::FILE_ATTRIBUTE::HIDDEN);
/// let is_temporary  = flags.has(co::FILE_ATTRIBUTE::TEMPORARY);
/// ```
pub fn GetFileAttributes(lpFileName: &str) -> WinResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		kernel32::GetFileAttributesW(WString::from_str(lpFileName).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(co::FILE_ATTRIBUTE(flags)),
	}
}

/// [`GetLargePageMinimum`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum)
/// function.
pub fn GetLargePageMinimum() -> u64 {
	unsafe { kernel32::GetLargePageMinimum() }
}

/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
///
/// This function is automatically called every time a
/// [`WinResult`](crate::WinResult) evaluates to `Err`, so it's unlikely that
/// you ever need to call it.
pub fn GetLastError() -> co::ERROR {
	co::ERROR(unsafe { kernel32::GetLastError() })
}

/// [`GetLogicalDriveStrings`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrivestringsw)
/// function.
pub fn GetLogicalDriveStrings() -> WinResult<Vec<String>> {
	match unsafe {
		kernel32::GetLogicalDriveStringsW(0, std::ptr::null_mut())
	} {
		0 => Err(GetLastError()),
		len => {
			let mut buf = WString::new_alloc_buffer(len as usize + 1);

			match unsafe {
				kernel32::GetLogicalDriveStringsW(len, buf.as_mut_ptr())
			} {
				0 => Err(GetLastError()),
				_ => Ok(parse_multi_z_str(unsafe { buf.as_ptr() })),
			}
		},
	}
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(lpMsg: &mut MSG, hWnd: Option<HWND>,
	wMsgFilterMin: u32, wMsgFilterMax: u32) -> WinResult<bool>
{
	match unsafe {
		user32::GetMessageW(
			lpMsg as *mut _ as _,
			hWnd.map_or(std::ptr::null_mut(), |h| h.ptr),
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
	unsafe { user32::GetQueueStatus(flags.0) }
}

/// [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
pub fn GetSysColor(nIndex: co::COLOR) -> COLORREF {
	COLORREF(unsafe { user32::GetSysColor(nIndex.0) })
}

/// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
pub fn GetSystemMetrics(nIndex: co::SM) -> i32 {
	unsafe { user32::GetSystemMetrics(nIndex.0) }
}

/// [`GetSystemTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
pub fn GetSystemTime(lpSystemTime: &mut SYSTEMTIME) {
	unsafe { kernel32::GetSystemTime(lpSystemTime as *mut _ as _) }
}

/// [`GetSystemTimeAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: &mut FILETIME) {
	unsafe {
		kernel32::GetSystemTimeAsFileTime(lpSystemTimeAsFileTime as *mut _ as _)
	}
}

/// [`GetSystemTimePreciseAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: &mut FILETIME) {
	unsafe {
		kernel32::GetSystemTimePreciseAsFileTime(
			lpSystemTimeAsFileTime as *mut _ as _,
		)
	}
}

/// [`GetTempPath`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppathw)
/// function.
pub fn GetTempPath() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel32::GetTempPathW(buf.buffer_size() as _, buf.as_mut_ptr()) }
	{
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetTickCount64`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64)
/// function.
pub fn GetTickCount64() -> u64 {
	unsafe { kernel32::GetTickCount64() }
}

/// [`GlobalMemoryStatusEx`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex)
/// function.
pub fn GlobalMemoryStatusEx(lpBuffer: &mut MEMORYSTATUSEX) -> WinResult<()> {
	bool_to_winresult(
		unsafe { kernel32::GlobalMemoryStatusEx(lpBuffer as *mut _ as _) },
	)
}

/// [`HIBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// function. Originally a macro.
pub const fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as _
}

/// Returns the high-order `u32` of an `u64`.
pub const fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as _
}

/// [`HIWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// function. Originally a macro.
pub const fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as _
}

/// [`HRESULT_FROM_WIN32`](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-hresult_from_win32)
/// function. Originally a macro.
pub const fn HRESULT_FROM_WIN32(hresult: HRESULT) -> co::ERROR {
	co::ERROR((hresult as u32) & 0xffff)
}

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { comctl32::InitCommonControls() }
}

/// [`IsGUIThread`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
pub fn IsGUIThread(bConvert: bool) -> WinResult<bool> {
	let r = unsafe { user32::IsGUIThread(bConvert as _) };
	if bConvert {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(co::ERROR(err as _)),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`IsWindows10OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows10orgreater)
/// function.
pub fn IsWindows10OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINTHRESHOLD.0) as _,
		LOBYTE(co::WIN32::WINNT_WINTHRESHOLD.0) as _,
		0,
	)
}

/// [`IsWindows7OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows7orgreater)
/// function.
pub fn IsWindows7OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN7.0) as _,
		LOBYTE(co::WIN32::WINNT_WIN7.0) as _,
		0,
	)
}

/// [`IsWindows8OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8orgreater)
/// function.
pub fn IsWindows8OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN8.0) as _,
		LOBYTE(co::WIN32::WINNT_WIN8.0) as _,
		0,
	)
}

/// [`IsWindows8Point1OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8point1orgreater)
/// function.
pub fn IsWindows8Point1OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINBLUE.0) as _,
		LOBYTE(co::WIN32::WINNT_WINBLUE.0) as _,
		0,
	)
}

/// [`IsWindowsServer`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsserver)
/// function.
pub fn IsWindowsServer() -> WinResult<bool> {
	let mut osvi = OSVERSIONINFOEX::default();
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
	let mut osvi = OSVERSIONINFOEX::default();
	let dwlConditionMask = VerSetConditionMask(
		VerSetConditionMask(
			VerSetConditionMask(0, co::VER_MASK::MAJORVERSION, co::VER_COND::GREATER_EQUAL),
			co::VER_MASK::MINORVERSION, co::VER_COND::GREATER_EQUAL,
		),
		co::VER_MASK::SERVICEPACKMAJOR, co::VER_COND::GREATER_EQUAL
	);

	osvi.dwMajorVersion = wMajorVersion as _;
	osvi.dwMinorVersion = wMinorVersion as _;
	osvi.wServicePackMajor = wServicePackMajor;

	VerifyVersionInfo(
		&mut osvi,
		co::VER_MASK::MAJORVERSION | co::VER_MASK::MINORVERSION | co::VER_MASK::SERVICEPACKMAJOR,
		dwlConditionMask,
	)
}

/// [`IsWindowsVistaOrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsvistaorgreater)
/// function.
pub fn IsWindowsVistaOrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_VISTA.0) as _,
		LOBYTE(co::WIN32::WINNT_VISTA.0) as _,
		0,
	)
}

/// [`LOBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// function. Originally a macro.
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// [`LockSetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
pub fn LockSetForegroundWindow(uLockCode: co::LSFW) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::LockSetForegroundWindow(uLockCode.0) },
	)
}

/// Returns the low-order `u32` of an `u64`.
pub const fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as _
}

/// [`LOWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// function. Originally a macro.
pub const fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as _
}

/// Function that implements
/// [`MAKELONG`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
pub const fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}

/// [`MAKEWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// function. Originally a macro.
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
pub fn MoveFile(
	lpExistingFileName: &str, lpNewFileName: &str) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::MoveFileW(
				WString::from_str(lpExistingFileName).as_ptr(),
				WString::from_str(lpNewFileName).as_ptr(),
			)
		},
	)
}

/// [`MulDiv`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
pub fn MulDiv(nNumber: i32, nNumerator: i32, nDenominator: i32) -> i32 {
	unsafe { kernel32::MulDiv(nNumber, nNumerator, nDenominator) }
}

/// [`MultiByteToWideChar`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
pub fn MultiByteToWideChar(
	CodePage: co::CP, dwFlags: co::MBC,
	lpMultiByteStr: &[u8]) -> WinResult<Vec<u16>> {

	match unsafe {
		kernel32::MultiByteToWideChar(
			CodePage.0,
			dwFlags.0,
			lpMultiByteStr.as_ptr(),
			lpMultiByteStr.len() as _,
			std::ptr::null_mut(),
			0,
		)
	} {
		0 => Err(GetLastError()),
		numBytes => {
			let mut destBuf: Vec<u16> = vec![0, numBytes as _];

			match unsafe {
				kernel32::MultiByteToWideChar(
					CodePage.0,
					dwFlags.0,
					lpMultiByteStr.as_ptr(),
					lpMultiByteStr.len() as _,
					destBuf.as_mut_ptr(),
					numBytes as _,
				)
			} {
				0 => Err(GetLastError()),
				_ => Ok(destBuf),
			}
		},
	}
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
pub fn PeekMessage(lpMsg: &mut MSG, hWnd: Option<HWND>,
	wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: co::PM) -> bool
{
	unsafe {
		user32::PeekMessageW(
			lpMsg as *mut _ as _,
			hWnd.map_or(std::ptr::null_mut(), |h| h.ptr),
			wMsgFilterMin,
			wMsgFilterMax,
			wRemoveMsg.0,
		) != 0
	}
}

/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
pub fn PostQuitMessage(nExitCode: co::ERROR) {
	unsafe { user32::PostQuitMessage(nExitCode.0 as _) }
}

/// [`RegisterClassEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
pub fn RegisterClassEx(lpwcx: &WNDCLASSEX) -> WinResult<ATOM> {
	match unsafe { user32::RegisterClassExW(lpwcx as *const _ as _) } {
		0 => Err(GetLastError()),
		atom => Ok(ATOM(atom)),
	}
}

/// [`ReleaseCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// function.
pub fn ReleaseCapture() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::ReleaseCapture() })
}

/// [`SetCaretBlinkTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretblinktime)
/// function.
pub fn SetCaretBlinkTime(uMSeconds: u32) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::SetCaretBlinkTime(uMSeconds) },
	)
}

/// [`SetCaretPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretpos)
/// function.
pub fn SetCaretPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetCaretPos(x, y) })
}

/// [`SetClipboardData`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
/// function.
pub fn SetClipboardData(uFormat: co::CF, hMem: *mut u8) -> WinResult<*mut u8> {
	unsafe { user32::SetClipboardData(uFormat.0, hMem as _).as_mut() }
		.map(|hMem| hMem as *mut _ as _)
		.ok_or_else(|| GetLastError())
}

/// [`SetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
/// function.
pub fn SetCursorPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetCursorPos(x, y) })
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(dwErrCode: co::ERROR) {
	unsafe { kernel32::SetLastError(dwErrCode.0) }
}

/// [`SetProcessDPIAware`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
pub fn SetProcessDPIAware() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetProcessDPIAware() })
}

/// [`ShowCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
/// function.
pub fn ShowCursor(bShow: bool) -> i32 {
	unsafe { user32::ShowCursor(bShow as _) }
}

/// [`Sleep`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(dwMilliseconds: u32) {
	unsafe { kernel32::Sleep(dwMilliseconds) }
}

/// [`SoundSentry`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-soundsentry)
/// function.
pub fn SoundSentry() -> bool {
	unsafe { user32::SoundSentry() != 0 }
}

/// [`SystemParametersInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow)
/// function.
///
/// The `pvParam` type varies according to the `uiAction`. If you set it wrong,
/// you're likely to cause a buffer overrun.
pub unsafe fn SystemParametersInfo<T>(
	uiAction: co::SPI,
	uiParam: u32,
	pvParam: &mut T,
	fWinIni: co::SPIF) -> WinResult<()>
{
	bool_to_winresult(
		user32::SystemParametersInfoW(
			uiAction.0,
			uiParam,
			pvParam as *mut _ as _,
			fWinIni.0,
		),
	)
}

/// [`SystemTimeToFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
pub fn SystemTimeToFileTime(
	lpSystemTime: &SYSTEMTIME, lpFileTime: &mut FILETIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::SystemTimeToFileTime(
				lpSystemTime as *const _ as _,
				lpFileTime as *mut _ as _,
			)
		},
	)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
pub fn SystemTimeToTzSpecificLocalTime(
	lpTimeZoneInformation: Option<&TIME_ZONE_INFORMATION>,
	lpUniversalTime: &SYSTEMTIME,
	lpLocalTime: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::SystemTimeToTzSpecificLocalTime(
				lpTimeZoneInformation.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				lpUniversalTime as *const _ as _,
				lpLocalTime as *mut _ as _,
			)
		},
	)
}

/// [`TrackMouseEvent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent)
/// function.
pub fn TrackMouseEvent(lpEventTrack: &mut TRACKMOUSEEVENT) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::TrackMouseEvent(lpEventTrack as *mut _ as _) },
	)
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(lpMsg: &MSG) -> bool {
	unsafe { user32::TranslateMessage(lpMsg as *const _ as _) != 0 }
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(
	lpClassName: &str, hInstance: HINSTANCE) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user32::UnregisterClassW(
				WString::from_str(lpClassName).as_ptr(),
				hInstance.ptr,
			)
		},
	)
}

/// [`VerifyVersionInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
pub fn VerifyVersionInfo(
	lpVersionInformation: &mut OSVERSIONINFOEX,
	dwTypeMask: co::VER_MASK,
	dwlConditionMask: u64) -> WinResult<bool>
{
	match unsafe {
		kernel32::VerifyVersionInfoW(
			lpVersionInformation as *mut _ as _,
			dwTypeMask.0,
			dwlConditionMask,
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
		kernel32::VerSetConditionMask(ConditionMask, TypeMask.0, Condition.0)
	}
}

/// [`WaitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage)
/// function.
pub fn WaitMessage() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::WaitMessage() })
}

/// [`WideCharToMultiByte`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
pub fn WideCharToMultiByte(
	CodePage: co::CP, dwFlags: co::WC,
	lpWideCharStr: &[u16], lpDefaultChar: Option<u8>,
	lpUsedDefaultChar: Option<&mut bool>) -> WinResult<Vec<u8>> {

	let mut lpDefaulCharBuf = lpDefaultChar.unwrap_or_default();

	match unsafe {
		kernel32::WideCharToMultiByte(
			CodePage.0,
			dwFlags.0,
			lpWideCharStr.as_ptr(),
			lpWideCharStr.len() as _,
			std::ptr::null_mut(),
			0,
			&mut lpDefaulCharBuf,
			std::ptr::null_mut(),
		)
	} {
		0 => Err(GetLastError()),
		numBytes => {
			let mut destBuf: Vec<u8> = vec![0; numBytes as _];
			let mut boolBuf: BOOL = 0;

			match unsafe {
				kernel32::WideCharToMultiByte(
					CodePage.0,
					dwFlags.0,
					lpWideCharStr.as_ptr(),
					lpWideCharStr.len() as _,
					destBuf.as_mut_ptr() as _,
					numBytes,
					&mut lpDefaulCharBuf,
					&mut boolBuf,
				)
			} {
				0 => Err(GetLastError()),
				_ => {
					if let Some(lp) = lpUsedDefaultChar {
						*lp = boolBuf != 0;
					}
					Ok(destBuf)
				},
			}
		},
	}
}
