#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::co;
use crate::ffi_types::BOOL;
use crate::kernel;
use crate::kernel::decl::{MEMORYSTATUSEX, OSVERSIONINFOEX, STARTUPINFO,
	SYSTEM_INFO, TIME_ZONE_INFORMATION, WinResult, WString};
use crate::kernel::privs::{bool_to_winresult, INVALID_FILE_ATTRIBUTES,
	MAX_COMPUTERNAME_LENGTH, MAX_PATH, parse_multi_z_str};
use crate::kernel::structs::{FILETIME, SYSTEMTIME};

/// [`CopyFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn CopyFile(
	existing_file: &str, new_file: &str,
	fail_if_exists: bool) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::CopyFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
				fail_if_exists as _,
			)
		},
	)
}

/// [`DeleteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn DeleteFile(file_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			kernel::ffi::DeleteFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`ExpandEnvironmentStrings`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::ExpandEnvironmentStrings;
///
/// let expanded = ExpandEnvironmentStrings(
///     "Os %OS%, home %HOMEPATH% and temp %TEMP%",
/// )?;
///
/// println!("{}", expanded);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn ExpandEnvironmentStrings(src: &str) -> WinResult<String> {
	let wsrc = WString::from_str(src);
	let len = unsafe {
		kernel::ffi::ExpandEnvironmentStringsW(
			wsrc.as_ptr(),
			std::ptr::null_mut(),
			0,
		)
	};

	let mut buf = WString::new_alloc_buffer(len as _);
	match unsafe {
		kernel::ffi::ExpandEnvironmentStringsW(
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn FileTimeToSystemTime(
	file_time: &FILETIME, system_time: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::FileTimeToSystemTime(
				file_time as *const _ as _,
				system_time as *mut _ as _,
			)
		},
	)
}

/// [`GetBinaryType`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetBinaryType(application_name: &str) -> WinResult<co::SCS> {
	let mut binary_type = co::SCS::default();
	bool_to_winresult(
		unsafe {
			kernel::ffi::GetBinaryTypeW(
				WString::from_str(application_name).as_ptr(),
				&mut binary_type.0,
			)
		},
	).map(|_| binary_type)
}

/// [`GetCommandLine`](https://docs.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getcommandlinew)
/// function.
///
/// For an example, see [`CommandLineToArgv`](crate::CommandLineToArgv).
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetCommandLine() -> String {
	WString::from_wchars_nullt(unsafe { kernel::ffi::GetCommandLineW() })
		.to_string()
}

/// [`GetComputerName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetComputerName() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz = buf.buffer_size() as u32;

	bool_to_winresult(
		unsafe { kernel::ffi::GetComputerNameW(buf.as_mut_ptr(), &mut sz) },
	).map(|_| buf.to_string())
}

/// [`GetCurrentDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentdirectory)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetCurrentDirectory() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel::ffi::GetCurrentDirectoryW(
			buf.buffer_size() as _,
			buf.as_mut_ptr(),
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetCurrentProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetCurrentProcessId() -> u32 {
	unsafe { kernel::ffi::GetCurrentProcessId() }
}

/// [`GetCurrentThreadId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetCurrentThreadId() -> u32 {
	unsafe { kernel::ffi::GetCurrentThreadId() }
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::GetEnvironmentStrings;
///
/// let env_vars = GetEnvironmentStrings()?;
/// for (k, v) in env_vars.iter() {
///     println!("{} = {}", k, v);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetEnvironmentStrings() -> WinResult<HashMap<String, String>> {
	unsafe { kernel::ffi::GetEnvironmentStringsW().as_mut() }
		.map(|ptr| {
			let vec_env_strs = parse_multi_z_str(ptr as *mut _ as _);
			unsafe { kernel::ffi::FreeEnvironmentStringsW(ptr); }

			let mut map = HashMap::with_capacity(vec_env_strs.len());
			for env_str in vec_env_strs {
				let pair: Vec<&str> = env_str.split("=").collect();
				map.insert(pair[0].to_owned(), pair[1].to_owned());
			}
			map
		})
		.ok_or_else(|| GetLastError())
}

/// [`GetFirmwareType`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwaretype)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetFirmwareType() -> WinResult<co::FIRMWARE_TYPE> {
	let mut ft = u32::default();
	bool_to_winresult(unsafe { kernel::ffi::GetFirmwareType(&mut ft) })
		.map(|_| co::FIRMWARE_TYPE(ft))
}

/// [`GetLargePageMinimum`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetLargePageMinimum() -> u64 {
	unsafe { kernel::ffi::GetLargePageMinimum() }
}

/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
///
/// This function is automatically called every time a
/// [`WinResult`](crate::WinResult) evaluates to `Err`, so it's unlikely that
/// you ever need to call it.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetLastError() -> co::ERROR {
	co::ERROR(unsafe { kernel::ffi::GetLastError() })
}

/// [`GetLogicalDriveStrings`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrivestringsw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetLogicalDriveStrings() -> WinResult<Vec<String>> {
	match unsafe {
		kernel::ffi::GetLogicalDriveStringsW(0, std::ptr::null_mut())
	} {
		0 => Err(GetLastError()),
		len => {
			let mut buf = WString::new_alloc_buffer(len as usize + 1);

			match unsafe {
				kernel::ffi::GetLogicalDriveStringsW(len, buf.as_mut_ptr())
			} {
				0 => Err(GetLastError()),
				_ => Ok(parse_multi_z_str(unsafe { buf.as_ptr() })),
			}
		},
	}
}

/// [`GetFileAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
/// function.
///
/// # Examples
///
/// Checking whether a file or folder exists:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, GetFileAttributes};
///
/// let file_exists = GetFileAttributes("C:\\Temp\\test.txt").is_ok();
/// ```
///
/// Retrieving various information about a file or folder path:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, GetFileAttributes};
///
/// let flags = GetFileAttributes("C:\\Temp\\test.txt")?;
///
/// let is_compressed = flags.has(co::FILE_ATTRIBUTE::COMPRESSED);
/// let is_directory  = flags.has(co::FILE_ATTRIBUTE::DIRECTORY);
/// let is_encrypted  = flags.has(co::FILE_ATTRIBUTE::ENCRYPTED);
/// let is_hidden     = flags.has(co::FILE_ATTRIBUTE::HIDDEN);
/// let is_temporary  = flags.has(co::FILE_ATTRIBUTE::TEMPORARY);
/// # Ok::<_, co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetFileAttributes(file_name: &str) -> WinResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		kernel::ffi::GetFileAttributesW(WString::from_str(file_name).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(co::FILE_ATTRIBUTE(flags)),
	}
}

/// [`GetNativeSystemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetNativeSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel::ffi::GetNativeSystemInfo(si as *mut _ as _) }
}

/// [`GetStartupInfo`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getstartupinfow)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetStartupInfo(si: &mut STARTUPINFO) {
	unsafe { kernel::ffi::GetStartupInfoW(si as *mut _ as _) }
}

/// [`GetSystemTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemTime(st: &mut SYSTEMTIME) {
	unsafe { kernel::ffi::GetSystemTime(st as *mut _ as _) }
}

/// [`GetSystemTimeAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemTimeAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel::ffi::GetSystemTimeAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimePreciseAsFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemTimePreciseAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel::ffi::GetSystemTimePreciseAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemTimes(
	idle_time: &mut FILETIME,
	kernel_time: &mut FILETIME,
	user_time: &mut FILETIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::GetSystemTimes(
				idle_time as *mut _ as _,
				kernel_time as *mut _ as _,
				user_time as *mut _ as _,
			)
		},
	)
}

/// [`GetTempPath`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppathw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetTempPath() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel::ffi::GetTempPathW(buf.buffer_size() as _, buf.as_mut_ptr()) }
	{
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetSystemDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemDirectory() -> WinResult<String> {
	let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
	match unsafe {
		kernel::ffi::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buffer_size() as _)
	} {
		0 => Err(GetLastError()),
		_ => Ok(buf.to_string()),
	}
}

/// [`GetSystemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel::ffi::GetSystemInfo(si as *mut _ as _) }
}

/// [`GetTickCount64`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GetTickCount64() -> u64 {
	unsafe { kernel::ffi::GetTickCount64() }
}

/// [`GlobalMemoryStatusEx`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn GlobalMemoryStatusEx(msx: &mut MEMORYSTATUSEX) -> WinResult<()> {
	bool_to_winresult(
		unsafe { kernel::ffi::GlobalMemoryStatusEx(msx as *mut _ as _) },
	)
}

/// [`HIBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// function. Originally a macro.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as _
}

/// Returns the high-order `u32` of an `u64`.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as _
}

/// [`HIWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// function. Originally a macro.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as _
}

/// [`IsNativeVhdBoot`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-isnativevhdboot)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsNativeVhdBoot() -> WinResult<bool> {
	let mut is_native: BOOL = 0;
	match unsafe { kernel::ffi::IsNativeVhdBoot(&mut is_native) } {
		0 => Err(GetLastError()),
		_ => Ok(is_native != 0),
	}
}

/// [`IsWindows10OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows10orgreater)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsWindows10OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINTHRESHOLD.0) as _,
		LOBYTE(co::WIN32::WINNT_WINTHRESHOLD.0) as _,
		0,
	)
}

/// [`IsWindows7OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows7orgreater)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsWindows7OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN7.0) as _,
		LOBYTE(co::WIN32::WINNT_WIN7.0) as _,
		0,
	)
}

/// [`IsWindows8OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8orgreater)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsWindows8OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN8.0) as _,
		LOBYTE(co::WIN32::WINNT_WIN8.0) as _,
		0,
	)
}

/// [`IsWindows8Point1OrGreater`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8point1orgreater)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsWindows8Point1OrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINBLUE.0) as _,
		LOBYTE(co::WIN32::WINNT_WINBLUE.0) as _,
		0,
	)
}

/// [`IsWindowsServer`](https://docs.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsserver)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn IsWindowsVistaOrGreater() -> WinResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_VISTA.0) as _,
		LOBYTE(co::WIN32::WINNT_VISTA.0) as _,
		0,
	)
}

/// [`LOBYTE`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// function. Originally a macro.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// Returns the low-order `u32` of an `u64`.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as _
}

/// [`LOWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// function. Originally a macro.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as _
}

/// Function that implements
/// [`MAKELONG`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}

/// Similar to [`MAKEDWORD`](crate::MAKEDWORD), but for `u64`.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn MAKEQWORD(lo: u32, hi: u32) -> u64 {
	((lo as u64 & 0xffff_ffff) | ((hi as u64 & 0xffff_ffff) << 32)) as _
}

/// [`MAKEWORD`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// function. Originally a macro.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn MoveFile(existing_file: &str, new_file: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			kernel::ffi::MoveFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
			)
		},
	)
}

/// [`MulDiv`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn MulDiv(number: i32, numerator: i32, denominator: i32) -> i32 {
	unsafe { kernel::ffi::MulDiv(number, numerator, denominator) }
}

/// [`MultiByteToWideChar`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn MultiByteToWideChar(
	code_page: co::CP, flags: co::MBC,
	multi_byte_str: &[u8]) -> WinResult<Vec<u16>>
{
	match unsafe {
		kernel::ffi::MultiByteToWideChar(
			code_page.0 as _,
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
				kernel::ffi::MultiByteToWideChar(
					code_page.0 as _,
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn OutputDebugString(output_string: &str) {
	unsafe {
		kernel::ffi::OutputDebugStringW(WString::from_str(output_string).as_ptr())
	}
}

/// [`QueryPerformanceCounter`](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{QueryPerformanceCounter, QueryPerformanceFrequency};
///
/// let freq = QueryPerformanceFrequency()?;
/// let t0 = QueryPerformanceCounter()?;
///
/// // perform some operation...
///
/// let duration_ms =
///     ((QueryPerformanceCounter()? - t0) as f64 / freq as f64) * 1000.0;
///
/// println!("Operation lasted {:.2} ms", duration_ms);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn QueryPerformanceCounter() -> WinResult<i64> {
	let mut perf_count = i64::default();
	bool_to_winresult(
		unsafe { kernel::ffi::QueryPerformanceCounter(&mut perf_count) },
	).map(|_| perf_count)
}

/// [`QueryPerformanceFrequency`](https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn QueryPerformanceFrequency() -> WinResult<i64> {
	let mut freq = i64::default();
	bool_to_winresult(
		unsafe { kernel::ffi::QueryPerformanceFrequency(&mut freq) },
	).map(|_| freq)
}

/// [`ReplaceFile`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn ReplaceFile(
	replaced: &str, replacement: &str,
	backup: Option<&str>, flags: co::REPLACEFILE) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::ReplaceFileW(
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

/// [`SetCurrentDirectory`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setcurrentdirectory)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn SetCurrentDirectory(path_name: &str) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			kernel::ffi::SetCurrentDirectoryW(WString::from_str(path_name).as_ptr())
		},
	)
}

/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn SetLastError(err_code: co::ERROR) {
	unsafe { kernel::ffi::SetLastError(err_code.0) }
}

/// [`Sleep`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn Sleep(milliseconds: u32) {
	unsafe { kernel::ffi::Sleep(milliseconds) }
}

/// [`SystemTimeToFileTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn SystemTimeToFileTime(
	st: &SYSTEMTIME, ft: &mut FILETIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::SystemTimeToFileTime(
				st as *const _ as _,
				ft as *mut _ as _,
			)
		},
	)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn SystemTimeToTzSpecificLocalTime(
	time_zone: Option<&TIME_ZONE_INFORMATION>,
	universal_time: &SYSTEMTIME,
	local_time: &mut SYSTEMTIME) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			kernel::ffi::SystemTimeToTzSpecificLocalTime(
				time_zone.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				universal_time as *const _ as _,
				local_time as *mut _ as _,
			)
		},
	)
}

/// [`VerifyVersionInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn VerifyVersionInfo(
	osvix: &mut OSVERSIONINFOEX,
	type_mask: co::VER_MASK,
	condition_mask: u64) -> WinResult<bool>
{
	match unsafe {
		kernel::ffi::VerifyVersionInfoW(
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn VerSetConditionMask(
	condition_mask: u64, type_mask: co::VER_MASK, condition: co::VER_COND) -> u64
{
	unsafe {
		kernel::ffi::VerSetConditionMask(condition_mask, type_mask.0, condition.0)
	}
}

/// [`WideCharToMultiByte`](https://docs.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub fn WideCharToMultiByte(
	code_page: co::CP, flags: co::WC,
	wide_char_str: &[u16], default_char: Option<u8>,
	used_default_char: Option<&mut bool>) -> WinResult<Vec<u8>> {

	let mut default_char_buf = default_char.unwrap_or_default();

	match unsafe {
		kernel::ffi::WideCharToMultiByte(
			code_page.0 as _,
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
				kernel::ffi::WideCharToMultiByte(
					code_page.0 as _,
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
