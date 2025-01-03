#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, ffi_types::*, privs::*};
use crate::prelude::*;

/// [`CopyFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
///
/// # Related functions
///
/// * [`DeleteFile`](crate::DeleteFile)
/// * [`MoveFile`](crate::MoveFile)
/// * [`MoveFileEx`](crate::MoveFileEx)
/// * [`ReplaceFile`](crate::ReplaceFile)
pub fn CopyFile(
	existing_file: &str,
	new_file: &str,
	fail_if_exists: bool,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::CopyFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
				fail_if_exists as _,
			)
		},
	)
}

/// [`CreateDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createdirectoryw)
/// function.
pub fn CreateDirectory(
	path_name: &str,
	security_attributes: Option<&SECURITY_ATTRIBUTES>,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::CreateDirectoryW(
				WString::from_str(path_name).as_ptr(),
				security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
			)
		},
	)
}

/// [`DeleteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
///
/// # Related functions
///
/// * [`CopyFile`](crate::CopyFile)
/// * [`MoveFile`](crate::MoveFile)
/// * [`MoveFileEx`](crate::MoveFileEx)
/// * [`ReplaceFile`](crate::ReplaceFile)
pub fn DeleteFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::DeleteFileW(WString::from_str(file_name).as_ptr()) },
	)
}

/// [`ExitProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
/// function.
pub fn ExitProcess(exit_code: u32) {
	unsafe { ffi::ExitProcess(exit_code) }
}

/// [`ExitThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
/// function.
pub fn ExitThread(exit_code: u32) {
	unsafe { ffi::ExitThread(exit_code) }
}

/// [`ExpandEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let expanded = w::ExpandEnvironmentStrings(
///     "Os %OS%, home %HOMEPATH% and temp %TEMP%",
/// )?;
///
/// println!("{}", expanded);
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn ExpandEnvironmentStrings(src: &str) -> SysResult<String> {
	let wsrc = WString::from_str(src);
	let mut buf_sz = match unsafe {
		ffi::ExpandEnvironmentStringsW(wsrc.as_ptr(), std::ptr::null_mut(), 0)
	} {
		0 => return Err(GetLastError()),
		n => n,
	}; // includes terminating null count

	loop {
		let mut buf = WString::new_alloc_buf(buf_sz as _);
		let required_sz = match unsafe {
			ffi::ExpandEnvironmentStringsW(wsrc.as_ptr(), buf.as_mut_ptr(), buf_sz)
		} {
			0 => return Err(GetLastError()),
			n => n,
		}; // plus terminating null count

		if required_sz == buf_sz {
			return Ok(buf.to_string());
		}

		buf_sz = required_sz; // includes terminating null count; set the new buffer size to try again
	}
}

/// [`FileTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-filetimetosystemtime)
/// function.
///
/// # Related functions
///
/// * [`GetLocalTime`](crate::GetLocalTime)
/// * [`GetSystemTime`](crate::GetSystemTime)
/// * [`SystemTimeToFileTime`](crate::SystemTimeToFileTime)
/// * [`SystemTimeToTzSpecificLocalTime`](crate::SystemTimeToTzSpecificLocalTime)
#[must_use]
pub fn FileTimeToSystemTime(ft: &FILETIME) -> SysResult<SYSTEMTIME> {
	let mut st = SYSTEMTIME::default();
	bool_to_sysresult(
		unsafe {
			ffi::FileTimeToSystemTime(ft as *const _ as _, &mut st as *mut _ as _)
		},
	).map(|_| st)
}

/// [`FlushProcessWriteBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
/// function.
pub fn FlushProcessWriteBuffers() {
	unsafe { ffi::FlushProcessWriteBuffers() }
}

/// [`FormatMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
/// function.
///
/// You don't need to call this function: all error types implement the
/// [`FormattedError`](crate::prelude::FormattedError) trait which will
/// automatically call `FormatMessage`.
#[must_use]
pub unsafe fn FormatMessage(
	flags: co::FORMAT_MESSAGE,
	source: Option<*mut std::ffi::c_void>,
	message_id: u32,
	lang_id: LANGID,
	args: Option<&[*mut std::ffi::c_void]>,
) -> SysResult<String>
{
	let mut ptr_buf = std::ptr::null_mut() as *mut u16;

	let nchars = match ffi::FormatMessageW(
		flags.raw(),
		source.unwrap_or(std::ptr::null_mut()),
		message_id,
		u16::from(lang_id) as _,
		&mut ptr_buf as *mut *mut _ as _, // pass pointer to pointer
		0,
		args.map_or(std::ptr::null_mut(), |arr| arr.as_ptr() as _),
	) as _ {
		0 => Err(GetLastError()),
		nchars => Ok(nchars),
	}?;

	let final_wstr = WString::from_wchars_count(ptr_buf, nchars as _);
	let _ = LocalFreeGuard::new(HLOCAL::from_ptr(ptr_buf as _)); // free returned pointer
	let final_str = final_wstr.to_string();
	Ok(final_str)
}

/// [`GetBinaryType`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
/// function.
#[must_use]
pub fn GetBinaryType(application_name: &str) -> SysResult<co::SCS> {
	let mut binary_type = co::SCS::default();
	bool_to_sysresult(
		unsafe {
			ffi::GetBinaryTypeW(
				WString::from_str(application_name).as_ptr(),
				binary_type.as_mut(),
			)
		},
	).map(|_| binary_type)
}

/// [`GetCommandLine`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getcommandlinew)
/// function.
///
/// For an example, see [`CommandLineToArgv`](crate::CommandLineToArgv).
#[must_use]
pub fn GetCommandLine() -> String {
	unsafe { WString::from_wchars_nullt(ffi::GetCommandLineW()) }
		.to_string()
}

/// [`GetComputerName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
#[must_use]
pub fn GetComputerName() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz = buf.buf_len() as u32;

	bool_to_sysresult(
		unsafe { ffi::GetComputerNameW(buf.as_mut_ptr(), &mut sz) },
	).map(|_| buf.to_string())
}

/// [`GetCurrentDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentdirectory)
/// function.
#[must_use]
pub fn GetCurrentDirectory() -> SysResult<String> {
	let mut buf_sz = match unsafe {
		ffi::GetCurrentDirectoryW(0, std::ptr::null_mut())
	} {
		0 => return Err(GetLastError()),
		n => n,
	}; // includes terminating null count

	loop {
		let mut buf = WString::new_alloc_buf(buf_sz as _);
		let returned_chars = match unsafe {
			ffi::GetCurrentDirectoryW(buf_sz, buf.as_mut_ptr())
		} {
			0 => return Err(GetLastError()),
			n => n,
		};

		if returned_chars < buf_sz {
			return Ok(buf.to_string());
		}

		buf_sz = returned_chars; // includes terminating null count; set the new buffer size to try again
	}
}

/// [`GetCurrentProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
/// function.
#[must_use]
pub fn GetCurrentProcessId() -> u32 {
	unsafe { ffi::GetCurrentProcessId() }
}

/// [`GetCurrentThreadId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)
/// function.
#[must_use]
pub fn GetCurrentThreadId() -> u32 {
	unsafe { ffi::GetCurrentThreadId() }
}

/// [`GetDriveType`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdrivetypew)
/// function.
#[must_use]
pub fn GetDriveType(root_path_name: Option<&str>) -> co::DRIVE {
	unsafe {
		co::DRIVE::from_raw(
			ffi::GetDriveTypeW(WString::from_opt_str(root_path_name).as_ptr()),
		)
	}
}

/// [`GetDiskFreeSpaceEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskfreespaceexw)
/// function.
pub fn GetDiskFreeSpaceEx(
	directory_name: Option<&str>,
	free_bytes_available_to_caller: Option<&mut u64>,
	total_number_of_bytes: Option<&mut u64>,
	total_number_of_free_bytes: Option<&mut u64>,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::GetDiskFreeSpaceExW(
				WString::from_opt_str(directory_name).as_ptr(),
				free_bytes_available_to_caller.map_or(std::ptr::null_mut(), |n| n),
				total_number_of_bytes.map_or(std::ptr::null_mut(), |n| n),
				total_number_of_free_bytes.map_or(std::ptr::null_mut(), |n| n),
			)
		},
	)
}

/// [`GetDiskSpaceInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskspaceinformationw)
/// function.
#[must_use]
pub fn GetDiskSpaceInformation(
	root_path: &str,
) -> SysResult<DISK_SPACE_INFORMATION>
{
	let mut disk_space_info = DISK_SPACE_INFORMATION::default();
	match unsafe {
		co::ERROR::from_raw(
			ffi::GetDiskSpaceInformationW(
				WString::from_str(root_path).as_ptr(),
				&mut disk_space_info as *mut _ as _,
			),
		)
	} {
		co::ERROR::SUCCESS
			| co::ERROR::MORE_DATA => Ok(disk_space_info),
		err => Err(err),
	}
}

/// [`GetEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getenvironmentstringsw)
/// function.
///
/// Returns the parsed strings, and automatically frees the retrieved
/// environment block with
/// [`FreeEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-freeenvironmentstringsw).
///
/// # Examples
///
/// Retrieving and printing the key/value pairs of all environment strings:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let env_vars = w::GetEnvironmentStrings()?;
/// for (k, v) in env_vars.iter() {
///     println!("{} = {}", k, v);
/// }
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn GetEnvironmentStrings() -> SysResult<Vec<(String, String)>> {
	ptr_to_sysresult(unsafe { ffi::GetEnvironmentStringsW() } as _)
		.map(|ptr| {
			let vec_entries = parse_multi_z_str(ptr as *mut _ as _);
			unsafe { ffi::FreeEnvironmentStringsW(ptr); }
			vec_entries.iter()
				.map(|env_str| {
					let mut pair = env_str.split("="); // assumes correctly formatted pairs
					let key = pair.next().unwrap();
					let val = pair.next().unwrap();
					(key.to_owned(), val.to_owned())
				})
				.collect()
		})
}

/// [`GetFileAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
/// function.
///
/// # Examples
///
/// Checking whether a file or folder exists:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let file_exists = w::GetFileAttributes("C:\\Temp\\test.txt").is_ok();
/// ```
///
/// Retrieving various information about a file or folder path:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let flags = w::GetFileAttributes("C:\\Temp\\test.txt")?;
///
/// let is_compressed = flags.has(co::FILE_ATTRIBUTE::COMPRESSED);
/// let is_directory  = flags.has(co::FILE_ATTRIBUTE::DIRECTORY);
/// let is_encrypted  = flags.has(co::FILE_ATTRIBUTE::ENCRYPTED);
/// let is_hidden     = flags.has(co::FILE_ATTRIBUTE::HIDDEN);
/// let is_temporary  = flags.has(co::FILE_ATTRIBUTE::TEMPORARY);
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn GetFileAttributes(file_name: &str) -> SysResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		ffi::GetFileAttributesW(WString::from_str(file_name).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(unsafe { co::FILE_ATTRIBUTE::from_raw(flags) }),
	}
}

/// [`GetFileAttributesEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesexw)
/// function.
///
/// This function uses `GET_FILEEX_INFO_LEVELS::GetFileExInfoStandard` flag,
/// which is the only available flag.
pub fn GetFileAttributesEx(file: &str) -> SysResult<WIN32_FILE_ATTRIBUTE_DATA> {
	let mut wfad = WIN32_FILE_ATTRIBUTE_DATA::default();
	bool_to_sysresult(
		unsafe {
			ffi::GetFileAttributesExW(
				WString::from_str(file).as_ptr(),
				0,
				&mut wfad as *mut _ as _,
			)
		},
	).map(|_| wfad)
}

/// [`GetFirmwareType`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwaretype)
/// function.
#[must_use]
pub fn GetFirmwareType() -> SysResult<co::FIRMWARE_TYPE> {
	let mut ft = u32::default();
	bool_to_sysresult(unsafe { ffi::GetFirmwareType(&mut ft) })
		.map(|_| unsafe { co::FIRMWARE_TYPE::from_raw(ft) })
}

/// [`GetLargePageMinimum`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum)
/// function.
#[must_use]
pub fn GetLargePageMinimum() -> usize {
	unsafe { ffi::GetLargePageMinimum() }
}

/// [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
///
/// This function is automatically called every time a
/// [`SysResult`](crate::SysResult) evaluates to `Err`, so it's unlikely that
/// you ever need to call it.
#[must_use]
pub fn GetLastError() -> co::ERROR {
	unsafe { co::ERROR::from_raw(ffi::GetLastError()) }
}

/// [`GetLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime)
/// function.
///
/// This function retrieves local time; for UTC time use
/// [`GetSystemTime`](crate::GetSystemTime).
///
/// # Related functions
///
/// * [`FileTimeToSystemTime`](crate::FileTimeToSystemTime)
/// * [`GetSystemTime`](crate::GetSystemTime)
/// * [`SystemTimeToFileTime`](crate::SystemTimeToFileTime)
/// * [`SystemTimeToTzSpecificLocalTime`](crate::SystemTimeToTzSpecificLocalTime)
#[must_use]
pub fn GetLocalTime() -> SYSTEMTIME {
	let mut st = SYSTEMTIME::default();
	unsafe { ffi::GetLocalTime(&mut st as *mut _ as _); }
	st
}

/// [`GetLogicalDrives`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives)
/// function.
#[must_use]
pub fn GetLogicalDrives() -> u32 {
	unsafe { ffi::GetLogicalDrives() }
}

/// [`GetLogicalDriveStrings`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrivestringsw)
/// function.
#[must_use]
pub fn GetLogicalDriveStrings() -> SysResult<Vec<String>> {
	let len = match unsafe {
		ffi::GetLogicalDriveStringsW(0, std::ptr::null_mut())
	} {
		0 => Err(GetLastError()),
		len => Ok(len),
	}?;

	let mut buf = WString::new_alloc_buf(len as usize + 1); // room for terminating null

	bool_to_sysresult(
		unsafe { ffi::GetLogicalDriveStringsW(len, buf.as_mut_ptr()) } as _,
	).map(|_| parse_multi_z_str(buf.as_ptr()))
}

/// [`GetLongPathName`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlongpathnamew)
/// function.
#[must_use]
pub fn GetLongPathName(short_path: &str) -> SysResult<String> {
	let short_path_w = WString::from_str(short_path);
	let path_sz = match unsafe {
		ffi::GetLongPathNameW(short_path_w.as_ptr(), std::ptr::null_mut(), 0)
	} {
		0 => return Err(GetLastError()),
		len => len,
	};

	let mut path_buf = WString::new_alloc_buf(path_sz as _);
	match unsafe {
		ffi::GetLongPathNameW(
			short_path_w.as_ptr(),
			path_buf.as_mut_ptr(),
			path_sz,
		)
	} {
		0 => Err(GetLastError()),
		_ => Ok(path_buf.to_string())
	}
}

/// [`GetNativeSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo)
/// function.
#[must_use]
pub fn GetNativeSystemInfo() -> SYSTEM_INFO {
	let mut si = SYSTEM_INFO::default();
	unsafe { ffi::GetNativeSystemInfo(&mut si as *mut _ as _); }
	si
}

/// [`GetPrivateProfileSection`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilesectionw)
/// function.
///
/// # Examples
///
/// Reading all key/value pairs of a section from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let pairs = w::GetPrivateProfileSection(
///     "MySection",
///     "C:\\Temp\\foo.ini",
/// )?;
///
/// for (key, val) in pairs.iter() {
///     println!("{} = {}", key, val);
/// }
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`GetPrivateProfileSectionNames`](crate::GetPrivateProfileSectionNames)
/// * [`GetPrivateProfileString`](crate::GetPrivateProfileString)
/// * [`WritePrivateProfileString`](crate::WritePrivateProfileString)
#[must_use]
pub fn GetPrivateProfileSection(
	section_name: &str,
	file_name: &str,
) -> SysResult<Vec<(String, String)>>
{
	let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		let returned_chars = unsafe { // char count without terminating null
			ffi::GetPrivateProfileSectionW(
				WString::from_str(section_name).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_str(file_name).as_ptr(),
			)
		} + 1 + 1; // plus terminating null count, plus weird extra count

		if GetLastError() == co::ERROR::FILE_NOT_FOUND {
			return Err(co::ERROR::FILE_NOT_FOUND);
		} else if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
			return Ok(
				parse_multi_z_str(buf.as_ptr())
					.iter()
					.map(|line| match line.split_once('=') {
						Some((key, val)) => (key.to_owned(), val.to_owned()),
						None => (String::new(), String::new()),
					})
					.collect(),
			);
		}

		buf_sz *= 2; // double the buffer size to try again
	}
}

/// [`GetPrivateProfileSectionNames`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilesectionnamesw)
/// function.
///
/// # Examples
///
/// Reading all section names from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let sections = w::GetPrivateProfileSectionNames(
///     Some("C:\\Temp\\foo.ini"),
/// )?;
///
/// for section in sections.iter() {
///     println!("{}", section);
/// }
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`GetPrivateProfileSection`](crate::GetPrivateProfileSection)
/// * [`GetPrivateProfileString`](crate::GetPrivateProfileString)
/// * [`WritePrivateProfileString`](crate::WritePrivateProfileString)
#[must_use]
pub fn GetPrivateProfileSectionNames(
	file_name: Option<&str>,
) -> SysResult<Vec<String>>
{
	let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		let returned_chars = unsafe { // char count without terminating null
			ffi::GetPrivateProfileSectionNamesW(
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_opt_str(file_name).as_ptr(),
			)
		} + 1 + 1; // plus terminating null count, plus weird extra count

		if GetLastError() == co::ERROR::FILE_NOT_FOUND {
			return Err(co::ERROR::FILE_NOT_FOUND);
		} else if (returned_chars as usize) < buf_sz { // to break, must have at least 1 char gap
			return Ok(parse_multi_z_str(buf.as_ptr()));
		}

		buf_sz *= 2; // double the buffer size to try again
	}
}

/// [`GetPrivateProfileString`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilestringw)
/// function.
///
/// # Examples
///
/// Reading from an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let val = w::GetPrivateProfileString(
///     "MySection",
///     "MyKey",
///     "C:\\Temp\\foo.ini",
/// )?.unwrap_or("not found!".to_owned());
///
/// println!("{}", val);
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`GetPrivateProfileSection`](crate::GetPrivateProfileSection)
/// * [`GetPrivateProfileSectionNames`](crate::GetPrivateProfileSectionNames)
/// * [`WritePrivateProfileString`](crate::WritePrivateProfileString)
#[must_use]
pub fn GetPrivateProfileString(
	section_name: &str,
	key_name: &str,
	file_name: &str,
) -> SysResult<Option<String>>
{
	let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
	loop {
		let mut buf = WString::new_alloc_buf(buf_sz);
		unsafe { // char count without terminating null
			ffi::GetPrivateProfileStringW(
				WString::from_str(section_name).as_ptr(),
				WString::from_str(key_name).as_ptr(),
				std::ptr::null_mut(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
				WString::from_str(file_name).as_ptr(),
			);
		}

		match GetLastError() {
			co::ERROR::SUCCESS => {
				return Ok(Some(buf.to_string()));
			},
			co::ERROR::MORE_DATA => {
				buf_sz *= 2; // double the buffer size to try again
			},
			co::ERROR::FILE_NOT_FOUND => {
				return Ok(None);
			},
			e => {
				return Err(e);
			},
		}
	}
}

/// [`GetStartupInfo`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getstartupinfow)
/// function.
#[must_use]
pub fn GetStartupInfo<'a, 'b>() -> STARTUPINFO<'a, 'b> {
	let mut si = STARTUPINFO::default();
	unsafe { ffi::GetStartupInfoW(&mut si as *mut _ as _); }
	si
}

/// [`GetSystemDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw)
/// function.
#[must_use]
pub fn GetSystemDirectory() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	let nchars = match unsafe {
		ffi::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buf_len() as _)
	} {
		0 => return Err(GetLastError()),
		n => n,
	} as usize;

	if nchars > buf.buf_len() {
		buf = WString::new_alloc_buf(nchars);
		if unsafe {
			ffi::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buf_len() as _)
		} == 0 {
			return Err(GetLastError());
		}
	}

	Ok(buf.to_string())
}

/// [`GetSystemFileCacheSize`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getsystemfilecachesize)
/// function.
///
/// Returns minimum and maximum size of file cache (in bytes), and enabled cache
/// limit flags, respectively.
#[must_use]
pub fn GetSystemFileCacheSize() -> SysResult<(usize, usize, co::FILE_CACHE)> {
	let (mut min, mut max) = (usize::default(), usize::default());
	let mut flags = co::FILE_CACHE::default();
	bool_to_sysresult(
		unsafe {
			ffi::GetSystemFileCacheSize(&mut min, &mut max, flags.as_mut())
		},
	).map(|_| (min, max, flags))
}

/// [`GetSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
/// function.
#[must_use]
pub fn GetSystemInfo() -> SYSTEM_INFO {
	let mut si = SYSTEM_INFO::default();
	unsafe { ffi::GetSystemInfo(&mut si as *mut _ as _); }
	si
}

/// [`GetSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
///
/// This function retrieves UTC time; for local time use
/// [`GetLocalTime`](crate::GetLocalTime).
///
/// # Related functions
///
/// * [`FileTimeToSystemTime`](crate::FileTimeToSystemTime)
/// * [`GetLocalTime`](crate::GetLocalTime)
/// * [`SystemTimeToFileTime`](crate::SystemTimeToFileTime)
/// * [`SystemTimeToTzSpecificLocalTime`](crate::SystemTimeToTzSpecificLocalTime)
#[must_use]
pub fn GetSystemTime() -> SYSTEMTIME {
	let mut st = SYSTEMTIME::default();
	unsafe { ffi::GetSystemTime(&mut st as *mut _ as _); }
	st
}

/// [`GetSystemTimeAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
#[must_use]
pub fn GetSystemTimeAsFileTime() -> FILETIME {
	let mut ft = FILETIME::default();
	unsafe { ffi::GetSystemTimeAsFileTime(&mut ft as *mut _ as _); }
	ft
}

/// [`GetSystemTimePreciseAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
#[must_use]
pub fn GetSystemTimePreciseAsFileTime() -> FILETIME {
	let mut ft = FILETIME::default();
	unsafe { ffi::GetSystemTimePreciseAsFileTime(&mut ft as *mut _ as _); }
	ft
}

/// [`GetSystemTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes)
/// function.
///
/// Returns idle, kernel and user times.
///
/// # Examples
///
/// Retrieving just the kernel time:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let (_, kernel_time, _) = w::GetSystemTimes()?;
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn GetSystemTimes() -> SysResult<(FILETIME, FILETIME, FILETIME)> {
	let mut idle_time = FILETIME::default();
	let mut kernel_time = FILETIME::default();
	let mut user_time = FILETIME::default();

	bool_to_sysresult(
		unsafe {
			ffi::GetSystemTimes(
				&mut idle_time as *mut _ as _,
				&mut kernel_time as *mut _ as _,
				&mut user_time as *mut _ as _,
			)
		},
	).map(|_| (idle_time, kernel_time, user_time))
}

/// [`GetTempFileName`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettempfilenamew)
/// function.
#[must_use]
pub fn GetTempFileName(
	path_name: &str,
	prefix: &str,
	unique: u32,
) -> SysResult<String>
{
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetTempFileNameW(
				WString::from_str(path_name).as_ptr(),
				WString::from_str(prefix).as_ptr(),
				unique,
				buf.as_mut_ptr(),
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetTempPath`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppathw)
/// function.
#[must_use]
pub fn GetTempPath() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe { ffi::GetTempPathW(buf.buf_len() as _, buf.as_mut_ptr()) } as _,
	).map(|_| buf.to_string())
}

/// [`GetTickCount64`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64)
/// function.
#[must_use]
pub fn GetTickCount64() -> u64 {
	unsafe { ffi::GetTickCount64() }
}

/// [`GetVolumeInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumeinformationw)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let mut name = String::default();
/// let mut serial_no = u32::default();
/// let mut max_comp_len = u32::default();
/// let mut sys_flags = co::FILE_VOL::default();
/// let mut sys_name = String::default();
///
/// w::GetVolumeInformation(
///     Some("C:\\"),
///     Some(&mut name),
///     Some(&mut serial_no),
///     Some(&mut max_comp_len),
///     Some(&mut sys_flags),
///     Some(&mut sys_name),
/// )?;
///
/// println!("Name: {}", name);
/// println!("Serial no: {:#010x}", serial_no);
/// println!("Max comp len: {}", max_comp_len);
/// println!("Sys flags: {:?}", sys_flags);
/// println!("Sys name: {}", sys_name);
/// # w::SysResult::Ok(())
/// ```
pub fn GetVolumeInformation(
	root_path_name: Option<&str>,
	name: Option<&mut String>,
	serial_number: Option<&mut u32>,
	max_component_len: Option<&mut u32>,
	file_system_flags: Option<&mut co::FILE_VOL>,
	file_system_name: Option<&mut String>,
) -> SysResult<()>
{
	let mut name_buf = match name {
		None => (WString::new(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};
	let mut sys_name_buf = match file_system_name {
		None => (WString::new(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};

	bool_to_sysresult(
		unsafe {
			ffi::GetVolumeInformationW(
				WString::from_opt_str(root_path_name).as_ptr(),
				match name {
					Some(_) => name_buf.0.as_mut_ptr(),
					None => std::ptr::null_mut(),
				},
				name_buf.1 as u32,
				serial_number.map_or(std::ptr::null_mut(), |n| n),
				max_component_len.map_or(std::ptr::null_mut(), |m| m),
				file_system_flags.map_or(std::ptr::null_mut(), |f| f.as_mut()),
				match file_system_name {
					Some(_) => sys_name_buf.0.as_mut_ptr(),
					None => std::ptr::null_mut(),
				},
				sys_name_buf.1 as u32,
			)
		},
	).map(|_| {
		if let Some(name) = name {
			*name = name_buf.0.to_string();
		}
		if let Some(sys_name) = file_system_name {
			*sys_name = sys_name_buf.0.to_string();
		}
	})
}

/// [`GetVolumePathName`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumepathnamew)
/// function.
#[must_use]
pub fn GetVolumePathName(file_name: &str) -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			ffi::GetVolumePathNameW(
				WString::from_str(file_name).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GlobalMemoryStatusEx`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex)
/// function.
#[must_use]
pub fn GlobalMemoryStatusEx() -> SysResult<MEMORYSTATUSEX> {
	let mut msx = MEMORYSTATUSEX::default();
	bool_to_sysresult(
		unsafe { ffi::GlobalMemoryStatusEx(&mut msx as *mut _ as _) },
	).map(|_| msx)
}

/// [`HIBYTE`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// macro.
///
/// # Related functions
///
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as _
}

/// Returns the high-order `u32` of an `u64`.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as _
}

/// [`HIWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// macro.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as _
}

/// [`IsDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)
/// function.
#[must_use]
pub fn IsDebuggerPresent() -> bool {
	unsafe { ffi::IsDebuggerPresent() != 0 }
}

/// [`IsNativeVhdBoot`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-isnativevhdboot)
/// function.
#[must_use]
pub fn IsNativeVhdBoot() -> SysResult<bool> {
	let mut is_native: BOOL = 0;
	bool_to_sysresult(unsafe { ffi::IsNativeVhdBoot(&mut is_native) })
		.map(|_| is_native != 0)
}

/// [`IsWindows10OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows10orgreater)
/// function.
#[must_use]
pub fn IsWindows10OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINTHRESHOLD.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WINTHRESHOLD.raw()) as _,
		0,
	)
}

/// [`IsWindows7OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows7orgreater)
/// function.
#[must_use]
pub fn IsWindows7OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN7.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WIN7.raw()) as _,
		0,
	)
}

/// [`IsWindows8OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8orgreater)
/// function.
#[must_use]
pub fn IsWindows8OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WIN8.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WIN8.raw()) as _,
		0,
	)
}

/// [`IsWindows8Point1OrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindows8point1orgreater)
/// function.
#[must_use]
pub fn IsWindows8Point1OrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_WINBLUE.raw()) as _,
		LOBYTE(co::WIN32::WINNT_WINBLUE.raw()) as _,
		0,
	)
}

/// [`IsWindowsServer`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsserver)
/// function.
#[must_use]
pub fn IsWindowsServer() -> SysResult<bool> {
	let mut osvi = OSVERSIONINFOEX::default();
	osvi.wProductType = co::VER_NT::WORKSTATION;
	let cond_mask = VerSetConditionMask(
		0, co::VER_MASK::PRODUCT_TYPE, co::VER_COND::EQUAL);
	VerifyVersionInfo(&mut osvi, co::VER_MASK::PRODUCT_TYPE, cond_mask)
		.map(|b| !b) // not workstation
}

/// [`IsWindowsVersionOrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsversionorgreater)
/// function.
#[must_use]
pub fn IsWindowsVersionOrGreater(
	major_version: u16,
	minor_version: u16,
	service_pack_major: u16,
) -> SysResult<bool>
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

/// [`IsWindowsVistaOrGreater`](https://learn.microsoft.com/en-us/windows/win32/api/versionhelpers/nf-versionhelpers-iswindowsvistaorgreater)
/// function.
#[must_use]
pub fn IsWindowsVistaOrGreater() -> SysResult<bool> {
	IsWindowsVersionOrGreater(
		HIBYTE(co::WIN32::WINNT_VISTA.raw()) as _,
		LOBYTE(co::WIN32::WINNT_VISTA.raw()) as _,
		0,
	)
}

/// [`LOBYTE`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632658(v=vs.85))
/// macro.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// Returns the low-order `u32` of an `u64`.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as _
}

/// [`LOWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// macro.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as _
}

/// Function analog to
/// [`MAKELONG`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}

/// Similar to [`MAKEDWORD`](crate::MAKEDWORD), but for `u64`.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEWORD`](crate::MAKEWORD)
#[must_use]
pub const fn MAKEQWORD(lo: u32, hi: u32) -> u64 {
	((lo as u64 & 0xffff_ffff) | ((hi as u64 & 0xffff_ffff) << 32)) as _
}

/// [`MAKEWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// macro.
///
/// # Related functions
///
/// * [`HIBYTE`](crate::HIBYTE)
/// * [`HIDWORD`](crate::HIDWORD)
/// * [`HIWORD`](crate::HIWORD)
/// * [`LOBYTE`](crate::LOBYTE)
/// * [`LODWORD`](crate::LODWORD)
/// * [`LOWORD`](crate::LOWORD)
/// * [`MAKEDWORD`](crate::MAKEDWORD)
/// * [`MAKEQWORD`](crate::MAKEQWORD)
#[must_use]
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
///
/// # Related functions
///
/// * [`CopyFile`](crate::CopyFile)
/// * [`DeleteFile`](crate::DeleteFile)
/// * [`MoveFileEx`](crate::MoveFileEx)
/// * [`ReplaceFile`](crate::ReplaceFile)
pub fn MoveFile(existing_file: &str, new_file: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			ffi::MoveFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
			)
		},
	)
}

/// [`MoveFileEx`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw)
/// function.
///
/// # Related functions
///
/// * [`CopyFile`](crate::CopyFile)
/// * [`DeleteFile`](crate::DeleteFile)
/// * [`MoveFile`](crate::MoveFile)
/// * [`ReplaceFile`](crate::ReplaceFile)
pub fn MoveFileEx(
	existing_file: &str,
	new_file: Option<&str>,
	flags: co::MOVEFILE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::MoveFileExW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_opt_str(new_file).as_ptr(),
				flags.raw(),
			)
		},
	)
}

/// [`MulDiv`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
#[must_use]
pub fn MulDiv(number: i32, numerator: i32, denominator: i32) -> i32 {
	unsafe { ffi::MulDiv(number, numerator, denominator) }
}

/// [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
///
/// If `multi_byte_str` doesn't have a terminating null, the resulting
/// `Vec<u16>` also won't include one.
///
/// # Related functions
///
/// * [`WideCharToMultiByte`](crate::WideCharToMultiByte)
#[must_use]
pub fn MultiByteToWideChar(
	code_page: co::CP,
	flags: co::MBC,
	multi_byte_str: &[u8],
) -> SysResult<Vec<u16>>
{
	let num_bytes = match unsafe {
		ffi::MultiByteToWideChar(
			code_page.raw() as _,
			flags.raw(),
			vec_ptr(multi_byte_str),
			multi_byte_str.len() as _,
			std::ptr::null_mut(),
			0,
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}?;

	let mut buf = vec![0u16; num_bytes as _];

	bool_to_sysresult(
		unsafe {
			ffi::MultiByteToWideChar(
				code_page.raw() as _,
				flags.raw(),
				vec_ptr(multi_byte_str),
				multi_byte_str.len() as _,
				buf.as_mut_ptr(),
				num_bytes as _,
			)
		},
	).map(|_| buf)
}

/// [`OutputDebugString`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringw)
/// function.
pub fn OutputDebugString(output_string: &str) {
	unsafe {
		ffi::OutputDebugStringW(WString::from_str(output_string).as_ptr())
	}
}

/// [`QueryPerformanceCounter`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let freq = w::QueryPerformanceFrequency()?;
/// let t0 = w::QueryPerformanceCounter()?;
///
/// // perform some operation...
///
/// let duration_ms =
///     ((w::QueryPerformanceCounter()? - t0) as f64 / freq as f64) * 1000.0;
///
/// println!("Operation lasted {:.2} ms", duration_ms);
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`QueryPerformanceFrequency`](crate::QueryPerformanceFrequency)
#[must_use]
pub fn QueryPerformanceCounter() -> SysResult<i64> {
	let mut perf_count = i64::default();
	bool_to_sysresult(unsafe { ffi::QueryPerformanceCounter(&mut perf_count) })
		.map(|_| perf_count)
}

/// [`QueryPerformanceFrequency`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// # Related functions
///
/// * [`QueryPerformanceCounter`](crate::QueryPerformanceCounter)
#[must_use]
pub fn QueryPerformanceFrequency() -> SysResult<i64> {
	let mut freq = i64::default();
	bool_to_sysresult(unsafe { ffi::QueryPerformanceFrequency(&mut freq) })
		.map(|_| freq)
}

/// [`QueryUnbiasedInterruptTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryunbiasedinterrupttime)
/// function.
#[must_use]
pub fn QueryUnbiasedInterruptTime() -> SysResult<u64> {
	let mut t = u64::default();
	bool_to_sysresult(unsafe { ffi::QueryUnbiasedInterruptTime(&mut t) })
		.map(|_| t)
}

/// [`ReplaceFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
/// function.
///
/// # Related functions
///
/// * [`CopyFile`](crate::CopyFile)
/// * [`DeleteFile`](crate::DeleteFile)
/// * [`MoveFile`](crate::MoveFile)
pub fn ReplaceFile(
	replaced: &str,
	replacement: &str,
	backup: Option<&str>,
	flags: co::REPLACEFILE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::ReplaceFileW(
				WString::from_str(replaced).as_ptr(),
				WString::from_str(replacement).as_ptr(),
				WString::from_opt_str(backup).as_ptr(),
				flags.raw(),
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			)
		},
	)
}

/// [`SetCurrentDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setcurrentdirectory)
/// function.
pub fn SetCurrentDirectory(path_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			ffi::SetCurrentDirectoryW(WString::from_str(path_name).as_ptr())
		},
	)
}

/// [`SetFileAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesw)
/// function.
pub fn SetFileAttributes(
	file_name: &str,
	attributes: co::FILE_ATTRIBUTE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::SetFileAttributesW(
				WString::from_str(file_name).as_ptr(),
				attributes.raw(),
			)
		},
	)
}

/// [`SetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(err_code: co::ERROR) {
	unsafe { ffi::SetLastError(err_code.raw()) }
}

/// [`SetThreadStackGuarantee`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadstackguarantee)
/// function.
///
/// Returns the size of the previous stack.
pub fn SetThreadStackGuarantee(stack_size_in_bytes: u32) -> SysResult<u32> {
	let mut sz = stack_size_in_bytes;
	bool_to_sysresult(unsafe { ffi::SetThreadStackGuarantee(&mut sz) })
		.map(|_| sz)
}

/// [`Sleep`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(milliseconds: u32) {
	unsafe { ffi::Sleep(milliseconds) }
}

/// [`SwitchToThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-switchtothread)
/// function.
pub fn SwitchToThread() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SwitchToThread() })
}

/// [`SystemTimeToFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
///
/// # Related functions
///
/// * [`FileTimeToSystemTime`](crate::FileTimeToSystemTime)
/// * [`GetLocalTime`](crate::GetLocalTime)
/// * [`GetSystemTime`](crate::GetSystemTime)
/// * [`SystemTimeToTzSpecificLocalTime`](crate::SystemTimeToTzSpecificLocalTime)
#[must_use]
pub fn SystemTimeToFileTime(st: &SYSTEMTIME) -> SysResult<FILETIME> {
	let mut ft = FILETIME::default();
	bool_to_sysresult(
		unsafe {
			ffi::SystemTimeToFileTime(st as *const _ as _, &mut ft as *mut _ as _)
		},
	).map(|_| ft)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
///
/// # Related functions
///
/// * [`FileTimeToSystemTime`](crate::FileTimeToSystemTime)
/// * [`GetLocalTime`](crate::GetLocalTime)
/// * [`GetSystemTime`](crate::GetSystemTime)
/// * [`SystemTimeToFileTime`](crate::SystemTimeToFileTime)
#[must_use]
pub fn SystemTimeToTzSpecificLocalTime(
	time_zone: Option<&TIME_ZONE_INFORMATION>,
	universal_time: &SYSTEMTIME,
) -> SysResult<SYSTEMTIME>
{
	let mut local_time = SYSTEMTIME::default();
	bool_to_sysresult(
		unsafe {
			ffi::SystemTimeToTzSpecificLocalTime(
				time_zone.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				universal_time as *const _ as _,
				&mut local_time as *mut _ as _,
			)
		},
	).map(|_| local_time)
}

/// [`VerifyVersionInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-verifyversioninfow)
/// function.
#[must_use]
pub fn VerifyVersionInfo(
	osvix: &mut OSVERSIONINFOEX,
	type_mask: co::VER_MASK,
	condition_mask: u64,
) -> SysResult<bool>
{
	match unsafe {
		ffi::VerifyVersionInfoW(
			osvix as *mut _ as _,
			type_mask.raw(),
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

/// [`VerSetConditionMask`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-versetconditionmask)
/// function.
#[must_use]
pub fn VerSetConditionMask(
	condition_mask: u64,
	type_mask: co::VER_MASK,
	condition: co::VER_COND,
) -> u64
{
	unsafe {
		ffi::VerSetConditionMask(condition_mask, type_mask.raw(), condition.raw())
	}
}

/// [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
///
/// If `wide_char_str` doesn't have a terminating null, the resulting `Vec<u8>`
/// also won't include one.
///
/// # Related functions
///
/// * [`MultiByteToWideChar`](crate::MultiByteToWideChar)
#[must_use]
pub fn WideCharToMultiByte(
	code_page: co::CP,
	flags: co::WC,
	wide_char_str: &[u16],
	default_char: Option<u8>,
	used_default_char: Option<&mut bool>,
) -> SysResult<Vec<u8>>
{
	let mut default_char_buf = default_char.unwrap_or_default();

	let num_bytes = match unsafe {
		ffi::WideCharToMultiByte(
			code_page.raw() as _,
			flags.raw(),
			vec_ptr(wide_char_str),
			wide_char_str.len() as _,
			std::ptr::null_mut(),
			0,
			&mut default_char_buf,
			std::ptr::null_mut(),
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}?;

	let mut u8_buf = vec![0u8; num_bytes as _];
	let mut bool_buf: BOOL = 0;

	bool_to_sysresult(
		unsafe {
			ffi::WideCharToMultiByte(
				code_page.raw() as _,
				flags.raw(),
				vec_ptr(wide_char_str),
				wide_char_str.len() as _,
				u8_buf.as_mut_ptr() as _,
				num_bytes as _,
				&mut default_char_buf,
				&mut bool_buf,
			)
		},
	).map(|_| {
		if let Some(used_default_char) = used_default_char {
			*used_default_char = bool_buf != 0;
		}
		u8_buf
	})
}

/// [`WritePrivateProfileString`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-writeprivateprofilestringw)
/// function.
///
/// # Examples
///
/// Writing value into an INI file:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// w::WritePrivateProfileString(
///     "MySection",
///     Some("MyKey"),
///     Some("new value"),
///     "C:\\Temp\\foo.ini",
/// )?;
/// # w::SysResult::Ok(())
/// ```
///
/// # Related functions
///
/// * [`GetPrivateProfileSection`](crate::GetPrivateProfileSection)
/// * [`GetPrivateProfileSectionNames`](crate::GetPrivateProfileSectionNames)
/// * [`GetPrivateProfileString`](crate::GetPrivateProfileString)
pub fn WritePrivateProfileString(
	section_name: &str,
	key_name: Option<&str>,
	new_val: Option<&str>,
	file_name: &str,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			ffi::WritePrivateProfileStringW(
				WString::from_str(section_name).as_ptr(),
				WString::from_opt_str(key_name).as_ptr(),
				WString::from_opt_str(new_val).as_ptr(),
				WString::from_str(file_name).as_ptr(),
			)
		},
	)
}
