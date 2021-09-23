//! Win32 free functions.

#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{
	advapi32,
	BOOL,
	comctl32,
	comdlg32,
	HRESULT,
	kernel32,
	shell32,
	user32,
	version,
};
use crate::handles::{HINSTANCE, HLOCAL, HWND};
use crate::privs::{
	bool_to_winresult,
	hr_to_winresult,
	INVALID_FILE_ATTRIBUTES,
	MAX_COMPUTERNAME_LENGTH,
	MAX_PATH,
	parse_multi_z_str,
	UNLEN,
};
use crate::structs::{
	ATOM,
	CHOOSECOLOR,
	COLORREF,
	DEVMODE,
	FILETIME,
	GUITHREADINFO,
	MEMORYSTATUSEX,
	MSG,
	NOTIFYICONDATA,
	OSVERSIONINFOEX,
	POINT,
	RECT,
	SHFILEINFO,
	SHFILEOPSTRUCT,
	SIZE,
	STARTUPINFO,
	SYSTEM_INFO,
	SYSTEMTIME,
	TASKDIALOGCONFIG,
	TIME_ZONE_INFORMATION,
	TRACKMOUSEEVENT,
	WNDCLASSEX,
};
use crate::various::WString;

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	rc: &mut RECT, style: co::WS,
	has_menu: bool, ex_style: co::WS_EX) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user32::AdjustWindowRectEx(
				rc as *mut _ as _,
				style.0,
				has_menu as _,
				ex_style.0,
			)
		},
	)
}

/// [`ChangeDisplaySettings`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw)
/// function.
pub fn ChangeDisplaySettings(
	dev_mode: &mut DEVMODE,
	flags: co::CDS) -> Result<co::DISP_CHANGE, co::DISP_CHANGE>
{
	let ret = unsafe {
		user32::ChangeDisplaySettingsW(dev_mode as *mut _ as _, flags.0)
	};
	if ret < 0 {
		Err(co::DISP_CHANGE(ret))
	} else {
		Ok(co::DISP_CHANGE(ret))
	}
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
/// if ChooseColor(&mut cc)? {
///     println!("The color: {} {} {}",
///         cc.rgbResult.GetRValue(),
///         cc.rgbResult.GetGValue(),
///         cc.rgbResult.GetBValue(),
///     );
/// }
/// ```
pub fn ChooseColor(cc: &mut CHOOSECOLOR) -> Result<bool, co::CDERR> {
	match unsafe { comdlg32::ChooseColorW(cc as *mut _ as _) } {
		0 => match CommDlgExtendedError() {
			co::CDERR::NoValue => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`ClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clipcursor)
/// function.
pub fn ClipCursor(rc: Option<&RECT>) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			user32::ClipCursor(
				rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
			)
		},
	)
}

/// [`CloseClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// function.
pub fn CloseClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::CloseClipboard() })
}

/// [`CommandLineToArgv`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
/// function.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{CommandLineToArgv, GetCommandLine};
///
/// let args = CommandLineToArgv(&GetCommandLine())?;
/// for arg in args.iter() {
///     println!("{}", arg);
/// }
/// ```
pub fn CommandLineToArgv(cmd_line: &str) -> WinResult<Vec<String>> {
	let mut num_args: i32 = 0;
	let lp_arr = unsafe {
		shell32::CommandLineToArgvW(
			WString::from_str(cmd_line).as_ptr(),
			&mut num_args,
		)
	};
	if lp_arr.is_null() {
		return Err(GetLastError());
	}

	let mut strs = Vec::with_capacity(num_args as _);
	for lp in unsafe { std::slice::from_raw_parts(lp_arr, num_args as _) }.iter() {
		strs.push(WString::from_wchars_nullt(*lp).to_string());
	}

	(HLOCAL { ptr: lp_arr as _ })
		.LocalFree()
		.map(|_| strs)
}

/// [`CommDlgExtendedError`](https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror)
/// function.
pub fn CommDlgExtendedError() -> co::CDERR {
	co::CDERR(unsafe { comdlg32::CommDlgExtendedError() })
}

/// [`CopyFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
pub fn CopyFile(
	existing_file: &str, new_file: &str,
	fail_if_exists: bool) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::CopyFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
				fail_if_exists as _,
			)
		},
	)
}

/// [`DecryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
pub fn DecryptFile(file_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::DecryptFileW(WString::from_str(file_name).as_ptr(), 0)
		},
	)
}

/// [`DeleteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
pub fn DeleteFile(file_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe { kernel32::DeleteFileW(WString::from_str(file_name).as_ptr()) },
	)
}

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
pub fn DispatchMessage(msg: &MSG) -> isize {
	unsafe { user32::DispatchMessageW(msg as *const _ as _) }
}

/// [`EmptyClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
/// function.
pub fn EmptyClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::EmptyClipboard() })
}

/// [`EncryptFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
pub fn EncryptFile(file_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::EncryptFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`EncryptionDisable`](https://docs.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
pub fn EncryptionDisable(dir_path: &str, disable: bool) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			advapi32::EncryptionDisable(
				WString::from_str(dir_path).as_ptr(),
				disable as _,
			)
		},
	)
}

/// [`EndMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endmenu)
/// function.
pub fn EndMenu() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::EndMenu() })
}

/// [`EnumDisplaySettingsEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsexw)
/// function
pub fn EnumDisplaySettingsEx(
	device_name: Option<&str>,
	mode_num: co::ENUM_SETTINGS,
	dev_mode: &mut DEVMODE,
	flags: co::EDS) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user32::EnumDisplaySettingsExW(
				device_name.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				mode_num.0,
				dev_mode as *mut _ as _,
				flags.0
			)
		},
	)
}

/// [`ExpandEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
/// function.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::ExpandEnvironmentStrings;
///
/// println("{}", ExpandEnvironmentStrings(
///     "Os %OS%, home %HOMEPATH%, temp %TEMP%",
/// )?;
/// ```
pub fn ExpandEnvironmentStrings(src: &str) -> WinResult<String> {
	let wsrc = WString::from_str(src);
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
	file_time: &FILETIME, system_time: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::FileTimeToSystemTime(
				file_time as *const _ as _,
				system_time as *mut _ as _,
			)
		},
	)
}

/// [`GetAsyncKeyState`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate)
/// function.
pub fn GetAsyncKeyState(virt_key: co::VK) -> bool {
	unsafe { user32::GetAsyncKeyState(virt_key.0 as _) != 0 }
}

/// [`GetBinaryType`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
/// function.
pub fn GetBinaryType(application_name: &str) -> WinResult<co::SCS> {
	let mut binary_type = co::SCS::W_32BIT_BINARY;
	bool_to_winresult(
		unsafe {
			kernel32::GetBinaryTypeW(
				WString::from_str(application_name).as_ptr(),
				&mut binary_type.0,
			)
		},
	).map(|_| binary_type)
}

/// [`GetClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipcursor)
/// function.
pub fn GetClipCursor() -> WinResult<RECT> {
	let mut rc = RECT::default();
	bool_to_winresult(unsafe { user32::GetClipCursor(&mut rc as *mut _ as _) })
		.map(|_| rc)
}

/// [`GetCommandLine`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getcommandlinew)
/// function.
///
/// For an example, see [`CommandLineToArgv`](crate::CommandLineToArgv).
pub fn GetCommandLine() -> String {
	WString::from_wchars_nullt(unsafe { kernel32::GetCommandLineW() }).to_string()
}

/// [`GetComputerName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
pub fn GetComputerName() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz = buf.buffer_size() as u32;
	bool_to_winresult(
		unsafe { kernel32::GetComputerNameW(buf.as_mut_ptr(), &mut sz) },
	).map(|_| buf.to_string())
}

/// [`GetCurrentDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentdirectory)
/// function.
pub fn GetCurrentDirectory() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel32::GetCurrentDirectoryW(buf.buffer_size() as _, buf.as_mut_ptr())
	} {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetCurrentProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
/// function.
pub fn GetCurrentProcessId() -> u32 {
	unsafe { kernel32::GetCurrentProcessId() }
}

/// [`GetCurrentThreadId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)
/// function.
pub fn GetCurrentThreadId() -> u32 {
	unsafe { kernel32::GetCurrentThreadId() }
}

/// [`GetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
/// function.
pub fn GetCursorPos() -> WinResult<POINT> {
	let mut pt = POINT::default();
	bool_to_winresult(
		unsafe { user32::GetCursorPos(&mut pt as *mut _ as _) },
	).map(|_| pt)
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
/// let env_vars = GetEnvironmentStrings()?;
/// for (k, v) in env_vars.iter() {
///     println!("{} = {}", k, v);
/// }
/// ```
pub fn GetEnvironmentStrings() -> WinResult<HashMap<String, String>> {
	unsafe { kernel32::GetEnvironmentStringsW().as_mut() }
		.map(|ptr| {
			let vec_env_strs = parse_multi_z_str(ptr as *mut _ as _);
			unsafe { kernel32::FreeEnvironmentStringsW(ptr); }

			let mut map = HashMap::with_capacity(vec_env_strs.len());
			for env_str in vec_env_strs {
				let pair: Vec<&str> = env_str.split("=").collect();
				map.insert(pair[0].to_owned(), pair[1].to_owned());
			}
			map
		})
		.ok_or_else(|| GetLastError())
}

/// [`GetFileAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
/// function.
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
/// let flags = GetFileAttributes("C:\\Temp\\test.txt")?;
///
/// let is_compressed = flags.has(co::FILE_ATTRIBUTE::COMPRESSED);
/// let is_directory  = flags.has(co::FILE_ATTRIBUTE::DIRECTORY);
/// let is_encrypted  = flags.has(co::FILE_ATTRIBUTE::ENCRYPTED);
/// let is_hidden     = flags.has(co::FILE_ATTRIBUTE::HIDDEN);
/// let is_temporary  = flags.has(co::FILE_ATTRIBUTE::TEMPORARY);
/// ```
pub fn GetFileAttributes(file_name: &str) -> WinResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		kernel32::GetFileAttributesW(WString::from_str(file_name).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(co::FILE_ATTRIBUTE(flags)),
	}
}

/// [`GetFileVersionInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfow)
/// function.
///
/// The passed buffer will be automatically allocated with
/// [`GetFileVersionInfoSize`](crate::GetFileVersionInfoSize).
pub fn GetFileVersionInfo(
	file_name: &str, data: &mut Vec<u8>) -> WinResult<()>
{
	data.resize(GetFileVersionInfoSize(file_name)? as _, 0);
	bool_to_winresult(
		unsafe {
			version::GetFileVersionInfoW(
				WString::from_str(file_name).as_ptr(),
				0,
				data.len() as _,
				data.as_mut_ptr() as _,
			)
		},
	)
}

/// [`GetFileVersionInfoSize`](https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-getfileversioninfosizew)
/// function.
pub fn GetFileVersionInfoSize(file_name: &str) -> WinResult<u32> {
	let mut dw_handle: u32 = 0;
	match unsafe {
		version::GetFileVersionInfoSizeW(
			WString::from_str(file_name).as_ptr(),
			&mut dw_handle,
		)
	} {
		0 => Err(GetLastError()),
		sz => Ok(sz)
	}
}

/// [`GetFirmwareType`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwaretype)
/// function.
pub fn GetFirmwareType() -> WinResult<co::FIRMWARE_TYPE> {
	let mut ft: u32 = 0;
	bool_to_winresult(unsafe { kernel32::GetFirmwareType(&mut ft) })
		.map(|_| co::FIRMWARE_TYPE(ft))
}

/// [`GetGUIThreadInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguithreadinfo)
/// function.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{GetGUIThreadInfo, GUITHREADINFO, HWND};
///
/// let hwnd: HWND; // initialized somewhere
///
/// let mut gti = GUITHREADINFO::default();
/// GetGUIThreadInfo(
///     hwnd.GetWindowThreadProcessId(),
///     &mut gti,
/// )?;
///
/// println!("Caret rect: {}", gti.rcCaret);
/// ```
pub fn GetGUIThreadInfo(
	thread_id: u32, gti: &mut GUITHREADINFO) -> WinResult<()>
{
	bool_to_winresult(
		unsafe { user32::GetGUIThreadInfo(thread_id, gti as *mut _ as _) }
	)
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

/// [`GetMenuCheckMarkDimensions`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenucheckmarkdimensions)
/// function.
pub fn GetMenuCheckMarkDimensions() -> SIZE {
	let dims = unsafe { user32::GetMenuCheckMarkDimensions() };
	SIZE::new(LOWORD(dims as _) as _, HIWORD(dims as _) as _)
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(
	msg: &mut MSG, hwnd: Option<HWND>,
	msg_filter_min: u32, msg_filter_max: u32) -> WinResult<bool>
{
	match unsafe {
		user32::GetMessageW(
			msg as *mut _ as _,
			hwnd.map_or(std::ptr::null_mut(), |h| h.ptr),
			msg_filter_min, msg_filter_max,
		)
	} {
		-1 => Err(GetLastError()),
		0 => Ok(false),
		_ => Ok(true),
	}
}

/// [`GetMessagePos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagepos)
/// function.
pub fn GetMessagePos() -> POINT {
	let xy = unsafe { user32::GetMessagePos() };
	POINT::new(LOWORD(xy) as _, HIWORD(xy) as _)
}

/// [`GetNativeSystemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo)
/// function.
pub fn GetNativeSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel32::GetNativeSystemInfo(si as *mut _ as _) }
}

/// [`GetQueueStatus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getqueuestatus)
/// function.
pub fn GetQueueStatus(flags: co::QS) -> u32 {
	unsafe { user32::GetQueueStatus(flags.0) }
}

/// [`GetStartupInfo`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getstartupinfow)
/// function.
pub fn GetStartupInfo(si: &mut STARTUPINFO) {
	unsafe { kernel32::GetStartupInfoW(si as *mut _ as _) }
}

/// [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
pub fn GetSysColor(index: co::COLOR) -> COLORREF {
	COLORREF(unsafe { user32::GetSysColor(index.0) })
}

/// [`GetSystemDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw)
/// function.
pub fn GetSystemDirectory() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel32::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buffer_size() as _)
	} {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetSystemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
/// function.
pub fn GetSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel32::GetSystemInfo(si as *mut _ as _) }
}

/// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
pub fn GetSystemMetrics(index: co::SM) -> i32 {
	unsafe { user32::GetSystemMetrics(index.0) }
}

/// [`GetSystemTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
pub fn GetSystemTime(st: &mut SYSTEMTIME) {
	unsafe { kernel32::GetSystemTime(st as *mut _ as _) }
}

/// [`GetSystemTimeAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
pub fn GetSystemTimeAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel32::GetSystemTimeAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimePreciseAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
pub fn GetSystemTimePreciseAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel32::GetSystemTimePreciseAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes)
/// function.
pub fn GetSystemTimes(
	idle_time: &mut FILETIME,
	kernel_time: &mut FILETIME,
	user_time: &mut FILETIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::GetSystemTimes(
				idle_time as *mut _ as _,
				kernel_time as *mut _ as _,
				user_time as *mut _ as _,
			)
		},
	)
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

/// [`GetUserName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
pub fn GetUserName() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(UNLEN + 1);
	let mut sz = buf.buffer_size() as u32;
	match unsafe { advapi32::GetUserNameW(buf.as_mut_ptr(), &mut sz) } {
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
pub fn GlobalMemoryStatusEx(msx: &mut MEMORYSTATUSEX) -> WinResult<()> {
	bool_to_winresult(
		unsafe { kernel32::GlobalMemoryStatusEx(msx as *mut _ as _) },
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
pub const fn HRESULT_FROM_WIN32(hr: HRESULT) -> co::ERROR {
	co::ERROR((hr as u32) & 0xffff)
}

/// [`InitCommonControls`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { comctl32::InitCommonControls() }
}

/// [`IsGUIThread`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
pub fn IsGUIThread(convert_to_gui_thread: bool) -> WinResult<bool> {
	let r = unsafe { user32::IsGUIThread(convert_to_gui_thread as _) };
	if convert_to_gui_thread {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(co::ERROR(err as _)),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`IsNativeVhdBoot`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-isnativevhdboot)
/// function.
pub fn IsNativeVhdBoot() -> WinResult<bool> {
	let mut is_native: BOOL = 0;
	match unsafe { kernel32::IsNativeVhdBoot(&mut is_native) } {
		0 => Err(GetLastError()),
		_ => Ok(is_native != 0),
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
	let cond_mask = VerSetConditionMask(
		0, co::VER_MASK::PRODUCT_TYPE, co::VER_COND::EQUAL);
	VerifyVersionInfo(&mut osvi, co::VER_MASK::PRODUCT_TYPE, cond_mask)
		.map(|b| !b) // not workstation
}

/// [`IsWindowsVersionOrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsversionorgreater)
/// function.
pub fn IsWindowsVersionOrGreater(
	major_version: u16, minor_version: u16,
	service_pack_major: u16) -> WinResult<bool>
{
	let mut osvi = OSVERSIONINFOEX::default();
	let cond_mask = VerSetConditionMask(
		VerSetConditionMask(
			VerSetConditionMask(0, co::VER_MASK::MAJORVERSION, co::VER_COND::GREATER_EQUAL),
			co::VER_MASK::MINORVERSION, co::VER_COND::GREATER_EQUAL,
		),
		co::VER_MASK::SERVICEPACKMAJOR, co::VER_COND::GREATER_EQUAL
	);

	osvi.dwMajorVersion = major_version as _;
	osvi.dwMinorVersion = minor_version as _;
	osvi.wServicePackMajor = service_pack_major;

	VerifyVersionInfo(
		&mut osvi,
		co::VER_MASK::MAJORVERSION | co::VER_MASK::MINORVERSION | co::VER_MASK::SERVICEPACKMAJOR,
		cond_mask,
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

/// [`IsWow64Message`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswow64message)
/// function.
pub fn IsWow64Message() -> bool {
	return unsafe { user32::IsWow64Message() != 0}
}

/// [`LOBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// function. Originally a macro.
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// [`LockSetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
pub fn LockSetForegroundWindow(lock_code: co::LSFW) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::LockSetForegroundWindow(lock_code.0) },
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

/// Similar to [`MAKEDWORD`](crate::MAKEDWORD), but for `u64`.
pub const fn MAKEQWORD(lo: u32, hi: u32) -> u64 {
	((lo as u64 & 0xffff_ffff) | ((hi as u64 & 0xffff_ffff) << 32)) as _
}

/// [`MAKEWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// function. Originally a macro.
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
pub fn MoveFile(existing_file: &str, new_file: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			kernel32::MoveFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
			)
		},
	)
}

/// [`MulDiv`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
pub fn MulDiv(number: i32, numerator: i32, denominator: i32) -> i32 {
	unsafe { kernel32::MulDiv(number, numerator, denominator) }
}

/// [`MultiByteToWideChar`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
pub fn MultiByteToWideChar(
	code_page: co::CP, flags: co::MBC,
	multi_byte_str: &[u8]) -> WinResult<Vec<u16>>
{
	match unsafe {
		kernel32::MultiByteToWideChar(
			code_page.0,
			flags.0,
			multi_byte_str.as_ptr(),
			multi_byte_str.len() as _,
			std::ptr::null_mut(),
			0,
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => {
			let num_bytes = num_bytes as usize + 1; // add room for terminating null
			let mut dest_buf: Vec<u16> = vec![0x0000; num_bytes as _];

			match unsafe {
				kernel32::MultiByteToWideChar(
					code_page.0,
					flags.0,
					multi_byte_str.as_ptr(),
					multi_byte_str.len() as _,
					dest_buf.as_mut_ptr(),
					num_bytes as _,
				)
			} {
				0 => Err(GetLastError()),
				_ => {
					unsafe { *dest_buf.get_unchecked_mut(num_bytes - 1) = 0x0000; } // terminating null
					Ok(dest_buf)
				},
			}
		},
	}
}

/// [`OutputDebugString`](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)
/// function.
pub fn OutputDebugString(output_string: &str) {
	unsafe {
		kernel32::OutputDebugStringW(WString::from_str(output_string).as_ptr())
	}
}

/// [`PeekMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
pub fn PeekMessage(
	msg: &mut MSG, hwnd: Option<HWND>,
	msg_filter_min: u32, msg_filter_max: u32, remove_msg: co::PM) -> bool
{
	unsafe {
		user32::PeekMessageW(
			msg as *mut _ as _,
			hwnd.map_or(std::ptr::null_mut(), |h| h.ptr),
			msg_filter_min,
			msg_filter_max,
			remove_msg.0,
		) != 0
	}
}

/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
pub fn PostQuitMessage(exit_code: i32) {
	unsafe { user32::PostQuitMessage(exit_code) }
}

/// [`QueryPerformanceCounter`](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{QueryPerformanceCounter, QueryPerformanceFrequency};
///
/// let freq = QueryPerformanceFrequency()?;
/// let start = QueryPerformanceCounter()?;
///
/// // perform some operation...
///
/// let duration_ms =
///     ((QueryPerformanceCounter()? - t0) as f64 / freq as f64) * 1000.0;
///
/// println!("Operation lasted {:.2} ms", duration_ms);
/// ```
pub fn QueryPerformanceCounter() -> WinResult<i64> {
	let mut perf_count: i64 = 0;
	bool_to_winresult(
		unsafe { kernel32::QueryPerformanceCounter(&mut perf_count) },
	).map(|_| perf_count)
}

/// [`QueryPerformanceFrequency`](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
pub fn QueryPerformanceFrequency() -> WinResult<i64> {
	let mut freq: i64 = 0;
	bool_to_winresult(
		unsafe { kernel32::QueryPerformanceFrequency(&mut freq) },
	).map(|_| freq)
}

/// [`RegisterClassEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
pub fn RegisterClassEx(wcx: &WNDCLASSEX) -> WinResult<ATOM> {
	match unsafe { user32::RegisterClassExW(wcx as *const _ as _) } {
		0 => Err(GetLastError()),
		atom => Ok(ATOM(atom)),
	}
}

/// [`ReleaseCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// function.
pub fn ReleaseCapture() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::ReleaseCapture() })
}

/// [`ReplaceFileW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
/// function.
pub fn ReplaceFile(
	replaced: &str, replacement: &str,
	backup: Option<&str>, flags: co::REPLACEFILE) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::ReplaceFileW(
				WString::from_str(replaced).as_ptr(),
				WString::from_str(replacement).as_ptr(),
				WString::from_opt_str(backup).as_ptr(),
				flags.0,
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			)
		},
	)
}

/// [`SetCaretBlinkTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretblinktime)
/// function.
pub fn SetCaretBlinkTime(milliseconds: u32) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::SetCaretBlinkTime(milliseconds) },
	)
}

/// [`SetCaretPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretpos)
/// function.
pub fn SetCaretPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetCaretPos(x, y) })
}

/// [`SetClipboardData`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
/// function.
pub fn SetClipboardData(format: co::CF, hmem: *mut u8) -> WinResult<*mut u8> {
	unsafe { user32::SetClipboardData(format.0, hmem as _).as_mut() }
		.map(|hmem| hmem as *mut _ as _)
		.ok_or_else(|| GetLastError())
}

/// [`SetCurrentDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setcurrentdirectory)
/// function.
pub fn SetCurrentDirectory(path_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			kernel32::SetCurrentDirectoryW(WString::from_str(path_name).as_ptr())
		},
	)
}

/// [`SetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
/// function.
pub fn SetCursorPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetCursorPos(x, y) })
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(err_code: co::ERROR) {
	unsafe { kernel32::SetLastError(err_code.0) }
}

/// [`SetProcessDPIAware`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
pub fn SetProcessDPIAware() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::SetProcessDPIAware() })
}

/// [`SHAddToRecentDocs`](https://docs.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shaddtorecentdocs)
/// function.
///
/// **Note:** The `pv` type varies according to `uFlags`. If you set it wrong,
/// you're likely to cause a buffer overrun.
pub unsafe fn SHAddToRecentDocs<T>(flags: co::SHARD, pv: &T) {
	shell32::SHAddToRecentDocs(flags.0, pv as *const _ as _);
}

/// [`Shell_NotifyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyiconw)
/// function.
pub fn Shell_NotifyIcon(
	message: co::NIM, data: &mut NOTIFYICONDATA) -> WinResult<()>
{
	bool_to_winresult(
		unsafe { shell32::Shell_NotifyIconW(message.0, data as *mut _ as _) },
	)
}

/// [`SHGetFileInfo`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetfileinfow)
/// function.
///
/// **Note:** If you are returning an icon in the `hIcon` member of
/// [`SHFILEINFO`](crate::SHFILEINFO), it must be paired with an
/// [`HICON::DestroyIcon`](crate::HICON::DestroyIcon) call.
pub fn SHGetFileInfo(
	path: &str, file_attrs: co::FILE_ATTRIBUTE,
	shfi: &mut SHFILEINFO, flags: co::SHGFI) -> WinResult<u32>
{
	match unsafe {
		shell32::SHGetFileInfoW(
			WString::from_str(path).as_ptr(),
			file_attrs.0,
			shfi as *mut _ as _,
			std::mem::size_of::<SHFILEINFO>() as _,
			flags.0,
		)
	} {
		0 => Err(GetLastError()),
		n => Ok(n as _),
	}
}

/// [`SHFileOperation`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shfileoperationw)
/// function.
pub fn SHFileOperation(file_op: &mut SHFILEOPSTRUCT) -> WinResult<()> {
	match unsafe {
		shell32::SHFileOperationW(file_op as *mut _ as _)
	} {
		0 => Err(GetLastError()),
		_ => Ok(()),
	}
}

/// [`ShowCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
/// function.
pub fn ShowCursor(show: bool) -> i32 {
	unsafe { user32::ShowCursor(show as _) }
}

/// [`Sleep`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(milliseconds: u32) {
	unsafe { kernel32::Sleep(milliseconds) }
}

/// [`SoundSentry`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-soundsentry)
/// function.
pub fn SoundSentry() -> bool {
	unsafe { user32::SoundSentry() != 0 }
}

/// [`SystemParametersInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow)
/// function.
///
/// **Note:** The `pvParam` type varies according to `uiAction`. If you set it
/// wrong, you're likely to cause a buffer overrun.
pub unsafe fn SystemParametersInfo<T>(
	action: co::SPI,
	ui_param: u32,
	pv_param: &mut T,
	win_ini: co::SPIF) -> WinResult<()>
{
	bool_to_winresult(
		user32::SystemParametersInfoW(
			action.0,
			ui_param,
			pv_param as *mut _ as _,
			win_ini.0,
		),
	)
}

/// [`SystemTimeToFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
pub fn SystemTimeToFileTime(
	st: &SYSTEMTIME, ft: &mut FILETIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::SystemTimeToFileTime(
				st as *const _ as _,
				ft as *mut _ as _,
			)
		},
	)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
pub fn SystemTimeToTzSpecificLocalTime(
	time_zone: Option<&TIME_ZONE_INFORMATION>,
	universal_time: &SYSTEMTIME,
	local_time: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel32::SystemTimeToTzSpecificLocalTime(
				time_zone.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				universal_time as *const _ as _,
				local_time as *mut _ as _,
			)
		},
	)
}

/// [`TaskDialogIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialogindirect)
/// function.
///
/// Returns:
/// * the selected `co::DLGID` button;
/// * if `pRadioButtons` of [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) struct
/// was set, the `u16` control ID of one of the specified radio buttons;
/// otherwise zero.
///
/// If you don't need all customizations, consider the
/// [`TaskDialog`](crate::HWND::TaskDialog) method.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, gui, IconIdTdicon,
///     TASKDIALOG_BUTTON TASKDIALOGCONFIG, TaskDialogIndirect,
///     WString};
///
/// let wnd: gui::WindowMain; // initialized somewhere
///
/// let mut tdc = TASKDIALOGCONFIG::default();
/// tdc.hwndParent = wnd.hwnd();
/// tdc.dwCommonButtons = co::TDCBF::YES | co::TDCBF::NO;
/// tdc.set_hMainIcon(IconIdTdicon::Tdicon(co::TD_ICON::INFORMATION));
///
/// let mut title = WString::from_str("Title");
/// tdc.set_pszWindowTitle(Some(&mut title));
///
/// let mut header = WString::from_str("Header");
/// tdc.set_pszMainInstruction(Some(&mut header));
///
/// let mut body = WString::from_str("Body");
/// tdc.set_pszContent(Some(&mut body));
///
/// // A custom button to appear before Yes and No.
/// let mut btn1 = TASKDIALOG_BUTTON::default();
/// let mut btn1_text = WString::from_str("Hello");
/// btn1.set_pszButtonText(Some(&mut btn1_text));
/// btn1.set_nButtonID(333); // this ID is returned if user clicks this button
/// let btns_slice = &mut [btn1];
/// tdc.set_pButtons(Some(btns_slice));
///
/// TaskDialogIndirect(&tdc, None)?;
/// ```
pub fn TaskDialogIndirect(
	task_config: &TASKDIALOGCONFIG,
	verification_flag_checked: Option<&mut bool>) -> WinResult<(co::DLGID, u16)>
{
	let mut pn_button: i32 = 0;
	let mut pn_radio_button: i32 = 0;
	let mut pf_bool: BOOL = 0;

	hr_to_winresult(
		unsafe {
			comctl32::TaskDialogIndirect(
				task_config as *const _ as _,
				&mut pn_button,
				&mut pn_radio_button,
				verification_flag_checked.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut pf_bool),
			)
		},
	)?;

	if let Some(pf) = verification_flag_checked {
		*pf = pf_bool != 0;
	}
	Ok((co::DLGID(pn_button as _), pn_radio_button as _))
}

/// [`TrackMouseEvent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent)
/// function.
pub fn TrackMouseEvent(tme: &mut TRACKMOUSEEVENT) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user32::TrackMouseEvent(tme as *mut _ as _) },
	)
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(msg: &MSG) -> bool {
	unsafe { user32::TranslateMessage(msg as *const _ as _) != 0 }
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(class_name: &str, hinst: HINSTANCE) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			user32::UnregisterClassW(
				WString::from_str(class_name).as_ptr(),
				hinst.ptr,
			)
		},
	)
}

/// [`VarQueryValue`](https://docs.microsoft.com/en-us/windows/win32/api/winver/nf-winver-verqueryvaluew)
/// function.
///
/// **Note:** The returned reference type varies according to `lpSubBlock`. If
/// you set it wrong, you're likely to cause a buffer overrun.
///
/// # Examples
///
/// Reading version information from resource:
///
/// ```rust,ignore
/// use winsafe::{HINSTANCE, VS_FIXEDFILEINFO};
/// use winsafe::{GetFileVersionInfo, VarQueryValue};
///
/// let exe_name = HINSTANCE::NULL.GetModuleFileName()?;
/// let mut res_buf = Vec::default();
/// GetFileVersionInfo(&exe_name, &mut res_buf)?;
///
/// let vsffi = unsafe {
///     VarQueryValue::<VS_FIXEDFILEINFO>(&res_buf, "\\")?
/// };
/// let ver = vsffi.dwFileVersion();
/// println!("Version {}.{}.{}.{}",
///     ver[0], ver[1], ver[2], ver[3]);
/// ```
pub unsafe fn VarQueryValue<'a, T>(
	block: &'a [u8], sub_block: &str) -> WinResult<&'a T>
{
	let mut lp_lp_buffer = std::ptr::null();
	let mut pu_len = 0;
	bool_to_winresult(
		version::VerQueryValueW(
			block.as_ptr() as _,
			WString::from_str(sub_block).as_ptr(),
			&mut lp_lp_buffer as *mut _ as _,
			&mut pu_len,
		),
	).map(|_| &*(lp_lp_buffer as *const T))
}

/// [`VerifyVersionInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
pub fn VerifyVersionInfo(
	osvix: &mut OSVERSIONINFOEX,
	type_mask: co::VER_MASK,
	condition_mask: u64) -> WinResult<bool>
{
	match unsafe {
		kernel32::VerifyVersionInfoW(
			osvix as *mut _ as _,
			type_mask.0,
			condition_mask,
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
	condition_mask: u64, type_mask: co::VER_MASK, condition: co::VER_COND) -> u64
{
	unsafe {
		kernel32::VerSetConditionMask(condition_mask, type_mask.0, condition.0)
	}
}

/// [`WaitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage)
/// function.
pub fn WaitMessage() -> WinResult<()> {
	bool_to_winresult(unsafe { user32::WaitMessage() })
}

/// [`WideCharToMultiByte`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
pub fn WideCharToMultiByte(
	code_page: co::CP, flags: co::WC,
	wide_char_str: &[u16], default_char: Option<u8>,
	used_default_char: Option<&mut bool>) -> WinResult<Vec<u8>> {

	let mut default_char_buf = default_char.unwrap_or_default();

	match unsafe {
		kernel32::WideCharToMultiByte(
			code_page.0,
			flags.0,
			wide_char_str.as_ptr(),
			wide_char_str.len() as _,
			std::ptr::null_mut(),
			0,
			&mut default_char_buf,
			std::ptr::null_mut(),
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => {
			let num_bytes = num_bytes as usize + 1; // add room for terminating null
			let mut dest_buf: Vec<u8> = vec![0x00; num_bytes as _];
			let mut bool_buf: BOOL = 0;

			match unsafe {
				kernel32::WideCharToMultiByte(
					code_page.0,
					flags.0,
					wide_char_str.as_ptr(),
					wide_char_str.len() as _,
					dest_buf.as_mut_ptr() as _,
					num_bytes as _,
					&mut default_char_buf,
					&mut bool_buf,
				)
			} {
				0 => Err(GetLastError()),
				_ => {
					if let Some(lp) = used_default_char {
						*lp = bool_buf != 0;
					}
					unsafe { *dest_buf.get_unchecked_mut(num_bytes - 1) = 0x00; } // terminating null
					Ok(dest_buf)
				},
			}
		},
	}
}
