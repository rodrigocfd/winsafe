#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{co, kernel};
use crate::kernel::decl::{
	DISK_SPACE_INFORMATION, FILETIME, HLOCAL, LANGID, LUID, MEMORYSTATUSEX,
	OSVERSIONINFOEX, SECURITY_DESCRIPTOR, SID, SID_IDENTIFIER_AUTHORITY,
	STARTUPINFO, SysResult, SYSTEM_INFO, SYSTEMTIME, TIME_ZONE_INFORMATION,
	WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::{
	FreeSidGuard, LocalFreeGuard, LocalFreeSidGuard, SidGuard,
};
use crate::kernel::privs::{
	bool_to_sysresult, INVALID_FILE_ATTRIBUTES, MAX_COMPUTERNAME_LENGTH,
	MAX_PATH, parse_multi_z_str, ptr_to_sysresult, SECURITY_DESCRIPTOR_REVISION,
};
use crate::prelude::{Handle, IntUnderlying, NativeStrConst};

/// [`AllocateAndInitializeSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-allocateandinitializesid)
/// function.
///
/// # Panics
///
/// Panics if `sub_authorities` has more than 8 elements.
///
/// # Examples
///
/// Create a well-known SID for the Everyone group:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{AllocateAndInitializeSid, co, SID_IDENTIFIER_AUTHORITY};
///
/// let sid_everyone = AllocateAndInitializeSid(
///     &SID_IDENTIFIER_AUTHORITY::WORLD,
///     &[
///         co::RID::SECURITY_WORLD,
///     ],
/// )?;
/// # Ok::<_, co::ERROR>(())
/// ```
///
/// Create a SID for the BUILTIN\Administrators group:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{AllocateAndInitializeSid, co, SID_IDENTIFIER_AUTHORITY};
///
/// let sid_builtin_administrators = AllocateAndInitializeSid(
///     &SID_IDENTIFIER_AUTHORITY::NT,
///     &[
///         co::RID::SECURITY_BUILTIN_DOMAIN,
///         co::RID::DOMAIN_ALIAS_ADMINS,
///     ],
/// )?;
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn AllocateAndInitializeSid(
	identifier_authority: &SID_IDENTIFIER_AUTHORITY,
	sub_authorities: &[co::RID],
) -> SysResult<FreeSidGuard>
{
	if sub_authorities.len() > 8 {
		panic!("You must specify at most 8 sub authorities.");
	}

	let mut psid = std::ptr::null_mut() as *mut SID;

	unsafe {
		bool_to_sysresult(
			kernel::ffi::AllocateAndInitializeSid(
				identifier_authority as *const _ as _,
				sub_authorities.len() as _,
				if sub_authorities.len() >= 1 { sub_authorities[0].raw() } else { 0 },
				if sub_authorities.len() >= 2 { sub_authorities[1].raw() } else { 0 },
				if sub_authorities.len() >= 3 { sub_authorities[2].raw() } else { 0 },
				if sub_authorities.len() >= 4 { sub_authorities[3].raw() } else { 0 },
				if sub_authorities.len() >= 5 { sub_authorities[4].raw() } else { 0 },
				if sub_authorities.len() >= 6 { sub_authorities[5].raw() } else { 0 },
				if sub_authorities.len() >= 7 { sub_authorities[6].raw() } else { 0 },
				if sub_authorities.len() >= 8 { sub_authorities[7].raw() } else { 0 },
				&mut psid as *mut _ as _,
			),
		).map(|_| FreeSidGuard::new(psid))
	}
}

/// [`ConvertSidToStringSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertsidtostringsidw)
/// function.
///
/// You don't need to call this function directly, because [`SID`](crate::SID)
/// implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
/// and [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html)
/// traits, which call it.
#[must_use]
pub fn ConvertSidToStringSid(sid: &SID) -> SysResult<String> {
	let mut pstr = std::ptr::null_mut() as *mut u16;
	bool_to_sysresult(
		unsafe {
			kernel::ffi::ConvertSidToStringSidW(sid as *const _ as _, &mut pstr)
		},
	)?;
	let name = WString::from_wchars_nullt(pstr).to_string();
	let _ = unsafe { LocalFreeGuard::new(HLOCAL::from_ptr(pstr as _)) }; // free returned pointer
	Ok(name)
}

/// [`ConvertStringSidToSid`](https://learn.microsoft.com/en-us/windows/win32/api/sddl/nf-sddl-convertstringsidtosidw)
/// function.
#[must_use]
pub fn ConvertStringSidToSid(str_sid: &str) -> SysResult<LocalFreeSidGuard> {
	let mut pbuf = std::ptr::null_mut() as *mut u8;
	unsafe {
		bool_to_sysresult(
			kernel::ffi::ConvertStringSidToSidW(
				WString::from_str(str_sid).as_ptr(),
				&mut pbuf,
			),
		).map(|_| LocalFreeSidGuard::new(HLOCAL::from_ptr(pbuf as _)))
	}
}

/// [`CopyFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-copyfilew)
/// function.
pub fn CopyFile(
	existing_file: &str, new_file: &str, fail_if_exists: bool) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::CopyFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
				fail_if_exists as _,
			)
		},
	)
}

/// [`CopySid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-copysid)
/// function.
#[must_use]
pub fn CopySid(src: &SID) -> SysResult<SidGuard> {
	let sid_sz = GetLengthSid(&src);
	let mut sid_buf = vec![0u8; sid_sz as _];

	unsafe {
		bool_to_sysresult(
			kernel::ffi::CopySid(
				sid_sz,
				sid_buf.as_mut_ptr(),
				src as *const _ as _,
			),
		).map(|_| SidGuard::new(sid_buf))
	}
}

/// [`CreateWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createwellknownsid)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CreateWellKnownSid};
///
/// let sid = CreateWellKnownSid(co::WELL_KNOWN_SID_TYPE::LocalSystem, None)?;
/// # Ok::<_, co::ERROR>(())
/// ```
#[must_use]
pub fn CreateWellKnownSid(
	well_known_sid: co::WELL_KNOWN_SID_TYPE,
	domain_sid: Option<&SID>,
) -> SysResult<SidGuard>
{
	let mut sid_sz = u32::default();

	unsafe {
		kernel::ffi::CreateWellKnownSid( // retrieve needed buffer sizes
			well_known_sid.raw(),
			domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
			std::ptr::null_mut(),
			&mut sid_sz,
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut sid_buf = vec![0u8; sid_sz as _];

	unsafe {
		bool_to_sysresult(
			kernel::ffi::CreateWellKnownSid(
				well_known_sid.raw(),
				domain_sid.map_or(std::ptr::null(), |s| s as *const _ as _),
				sid_buf.as_mut_ptr(),
				&mut sid_sz,
			),
		).map(|_| SidGuard::new(sid_buf))
	}
}

/// [`DeleteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew)
/// function.
pub fn DeleteFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			kernel::ffi::DeleteFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`DecryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-decryptfilew)
/// function.
pub fn DecryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			kernel::ffi::DecryptFileW(WString::from_str(file_name).as_ptr(), 0)
		},
	)
}

/// [`EncryptFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-encryptfilew)
/// function.
pub fn EncryptFile(file_name: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			kernel::ffi::EncryptFileW(WString::from_str(file_name).as_ptr())
		},
	)
}

/// [`EncryptionDisable`](https://learn.microsoft.com/en-us/windows/win32/api/winefs/nf-winefs-encryptiondisable)
/// function.
pub fn EncryptionDisable(dir_path: &str, disable: bool) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			kernel::ffi::EncryptionDisable(
				WString::from_str(dir_path).as_ptr(),
				disable as _,
			)
		},
	)
}

/// [`EqualDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equaldomainsid)
/// function.
#[must_use]
pub fn EqualDomainSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	let mut is_equal: BOOL = 0;
	bool_to_sysresult(
		unsafe {
			kernel::ffi::EqualDomainSid(
				sid1 as *const _ as _,
				sid2 as *const _ as _,
				&mut is_equal,
			)
		},
	).map(|_| is_equal != 0)
}

/// [`EqualPrefixSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalprefixsid)
/// function.
#[must_use]
pub fn EqualPrefixSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		kernel::ffi::EqualPrefixSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EqualSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-equalsid)
/// function.
#[must_use]
pub fn EqualSid(sid1: &SID, sid2: &SID) -> SysResult<bool> {
	match unsafe {
		kernel::ffi::EqualSid(sid1 as *const _ as _, sid2 as *const _ as _)
	} {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`ExitProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
/// function.
pub fn ExitProcess(exit_code: u32) {
	unsafe { kernel::ffi::ExitProcess(exit_code) }
}

/// [`ExitThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
/// function.
pub fn ExitThread(exit_code: u32) {
	unsafe { kernel::ffi::ExitThread(exit_code) }
}

/// [`ExpandEnvironmentStrings`](https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-expandenvironmentstringsw)
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
#[must_use]
pub fn ExpandEnvironmentStrings(src: &str) -> SysResult<String> {
	let wsrc = WString::from_str(src);
	let len = unsafe {
		kernel::ffi::ExpandEnvironmentStringsW(
			wsrc.as_ptr(),
			std::ptr::null_mut(),
			0,
		)
	};

	let mut buf = WString::new_alloc_buf(len as _);
	bool_to_sysresult(
		unsafe {
			kernel::ffi::ExpandEnvironmentStringsW(
				wsrc.as_ptr(),
				buf.as_mut_ptr(),
				len,
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`FileTimeToSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-filetimetosystemtime)
/// function.
pub fn FileTimeToSystemTime(
	file_time: &FILETIME, system_time: &mut SYSTEMTIME) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::FileTimeToSystemTime(
				file_time as *const _ as _,
				system_time as *mut _ as _,
			)
		},
	)
}

/// [`FlushProcessWriteBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
/// function.
pub fn FlushProcessWriteBuffers() {
	unsafe { kernel::ffi::FlushProcessWriteBuffers() }
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

	let nchars = match kernel::ffi::FormatMessageW(
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
			kernel::ffi::GetBinaryTypeW(
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
	WString::from_wchars_nullt(unsafe { kernel::ffi::GetCommandLineW() })
		.to_string()
}

/// [`GetComputerName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew)
/// function.
#[must_use]
pub fn GetComputerName() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_COMPUTERNAME_LENGTH + 1);
	let mut sz = buf.buf_len() as u32;

	bool_to_sysresult(
		unsafe { kernel::ffi::GetComputerNameW(buf.as_mut_ptr(), &mut sz) },
	).map(|_| buf.to_string())
}

/// [`GetCurrentDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcurrentdirectory)
/// function.
#[must_use]
pub fn GetCurrentDirectory() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetCurrentDirectoryW(
				buf.buf_len() as _,
				buf.as_mut_ptr(),
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetCurrentProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
/// function.
#[must_use]
pub fn GetCurrentProcessId() -> u32 {
	unsafe { kernel::ffi::GetCurrentProcessId() }
}

/// [`GetCurrentThreadId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid)
/// function.
#[must_use]
pub fn GetCurrentThreadId() -> u32 {
	unsafe { kernel::ffi::GetCurrentThreadId() }
}

/// [`GetDriveType`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdrivetypew)
/// function.
#[must_use]
pub fn GetDriveType(root_path_name: Option<&str>) -> co::DRIVE {
	unsafe {
		co::DRIVE::from_raw(
			kernel::ffi::GetDriveTypeW(
				WString::from_opt_str(root_path_name).as_ptr(),
			),
		)
	}
}

/// [`GetDiskSpaceInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskspaceinformationw)
/// function.
pub fn GetDiskSpaceInformation(
	root_path: &str,
	disk_space_info: &mut DISK_SPACE_INFORMATION,
) -> SysResult<()>
{
	match unsafe {
		co::ERROR::from_raw(
			kernel::ffi::GetDiskSpaceInformationW(
				WString::from_str(root_path).as_ptr(),
				disk_space_info as *mut _ as _,
			),
		)
	} {
		co::ERROR::SUCCESS
			| co::ERROR::MORE_DATA => Ok(()),
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
#[must_use]
pub fn GetEnvironmentStrings() -> SysResult<HashMap<String, String>> {
	ptr_to_sysresult(
		unsafe { kernel::ffi::GetEnvironmentStringsW() } as _,
	).map(|ptr| {
		let vec_env_strs = parse_multi_z_str(ptr as *mut _ as _);
		unsafe { kernel::ffi::FreeEnvironmentStringsW(ptr); }
		vec_env_strs.iter()
			.map(|env_str| {
				let mut pair = env_str.split("="); // assumes correctly formatted pairs
				let key = pair.next().unwrap();
				let val = pair.next().unwrap();
				(key.to_owned(), val.to_owned())
			})
			.collect()
	})
}

/// [`GetFirmwareType`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwaretype)
/// function.
#[must_use]
pub fn GetFirmwareType() -> SysResult<co::FIRMWARE_TYPE> {
	let mut ft = u32::default();
	bool_to_sysresult(unsafe { kernel::ffi::GetFirmwareType(&mut ft) })
		.map(|_| unsafe { co::FIRMWARE_TYPE::from_raw(ft) })
}

/// [`GetLargePageMinimum`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-getlargepageminimum)
/// function.
#[must_use]
pub fn GetLargePageMinimum() -> usize {
	unsafe { kernel::ffi::GetLargePageMinimum() }
}

/// [`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
/// function.
///
/// This function is automatically called every time a
/// [`SysResult`](crate::SysResult) evaluates to `Err`, so it's unlikely that
/// you ever need to call it.
#[must_use]
pub fn GetLastError() -> co::ERROR {
	unsafe { co::ERROR::from_raw(kernel::ffi::GetLastError()) }
}

/// [`GetLengthSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getlengthsid)
/// function.
#[must_use]
pub fn GetLengthSid(sid: &SID) -> u32 {
	unsafe { kernel::ffi::GetLengthSid(sid as *const _ as _) }
}

/// [`GetLogicalDrives`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives)
/// function.
#[must_use]
pub fn GetLogicalDrives() -> u32 {
	unsafe { kernel::ffi::GetLogicalDrives() }
}

/// [`GetLogicalDriveStrings`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrivestringsw)
/// function.
#[must_use]
pub fn GetLogicalDriveStrings() -> SysResult<Vec<String>> {
	let len = match unsafe {
		kernel::ffi::GetLogicalDriveStringsW(0, std::ptr::null_mut())
	} {
		0 => Err(GetLastError()),
		len => Ok(len),
	}?;

	let mut buf = WString::new_alloc_buf(len as usize + 1); // room for terminating null

	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetLogicalDriveStringsW(len, buf.as_mut_ptr())
		} as _,
	).map(|_| parse_multi_z_str(buf.as_ptr()))
}

/// [`GetFileAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileattributesw)
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
#[must_use]
pub fn GetFileAttributes(file_name: &str) -> SysResult<co::FILE_ATTRIBUTE> {
	const INVALID: u32 = INVALID_FILE_ATTRIBUTES as u32;
	match unsafe {
		kernel::ffi::GetFileAttributesW(WString::from_str(file_name).as_ptr())
	} {
		INVALID => Err(GetLastError()),
		flags => Ok(unsafe { co::FILE_ATTRIBUTE::from_raw(flags) }),
	}
}

/// [`GetLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlocaltime)
/// function.
///
/// This function retrieves local time; for UTC time use
/// [`GetSystemTime`](crate::GetSystemTime).
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetLocalTime, SYSTEMTIME};
///
/// let mut st = SYSTEMTIME::default();
/// GetLocalTime(&mut st);
/// ```
pub fn GetLocalTime(st: &mut SYSTEMTIME) {
	unsafe { kernel::ffi::GetLocalTime(st as *mut _ as _) }
}

/// [`GetNativeSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo)
/// function.
pub fn GetNativeSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel::ffi::GetNativeSystemInfo(si as *mut _ as _) }
}

/// [`GetSidLengthRequired`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getsidlengthrequired)
/// function.
#[must_use]
pub fn GetSidLengthRequired(sub_authority_count: u8) -> u32 {
	unsafe { kernel::ffi::GetSidLengthRequired(sub_authority_count) }
}

/// [`GetStartupInfo`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getstartupinfow)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetStartupInfo, STARTUPINFO};
///
/// let mut si = STARTUPINFO::default();
/// GetStartupInfo(&mut si);
/// ```
pub fn GetStartupInfo(si: &mut STARTUPINFO) {
	unsafe { kernel::ffi::GetStartupInfoW(si as *mut _ as _) }
}

/// [`GetSystemDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw)
/// function.
#[must_use]
pub fn GetSystemDirectory() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetSystemDirectoryW(buf.as_mut_ptr(), buf.buf_len() as _)
		} as _,
	).map(|_| buf.to_string())
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
			kernel::ffi::GetSystemFileCacheSize(&mut min, &mut max, flags.as_mut())
		},
	).map(|_| (min, max, flags))
}

/// [`GetSystemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsysteminfo)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetSystemInfo, SYSTEM_INFO};
///
/// let mut si = SYSTEM_INFO::default();
/// GetSystemInfo(&mut si);
/// ```
pub fn GetSystemInfo(si: &mut SYSTEM_INFO) {
	unsafe { kernel::ffi::GetSystemInfo(si as *mut _ as _) }
}

/// [`GetSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtime)
/// function.
///
/// This function retrieves UTC time; for local time use
/// [`GetLocalTime`](crate::GetLocalTime).
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetSystemTime, SYSTEMTIME};
///
/// let mut st = SYSTEMTIME::default();
/// GetSystemTime(&mut st);
/// ```
pub fn GetSystemTime(st: &mut SYSTEMTIME) {
	unsafe { kernel::ffi::GetSystemTime(st as *mut _ as _) }
}

/// [`GetSystemTimeAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeasfiletime)
/// function.
pub fn GetSystemTimeAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel::ffi::GetSystemTimeAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimePreciseAsFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime)
/// function.
pub fn GetSystemTimePreciseAsFileTime(ft: &mut FILETIME) {
	unsafe { kernel::ffi::GetSystemTimePreciseAsFileTime(ft as *mut _ as _) }
}

/// [`GetSystemTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getsystemtimes)
/// function.
pub fn GetSystemTimes(
	idle_time: &mut FILETIME,
	kernel_time: &mut FILETIME,
	user_time: &mut FILETIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetSystemTimes(
				idle_time as *mut _ as _,
				kernel_time as *mut _ as _,
				user_time as *mut _ as _,
			)
		},
	)
}

/// [`GetTempPath`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppathw)
/// function.
#[must_use]
pub fn GetTempPath() -> SysResult<String> {
	let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetTempPathW(buf.buf_len() as _, buf.as_mut_ptr())
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetTickCount64`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount64)
/// function.
#[must_use]
pub fn GetTickCount64() -> u64 {
	unsafe { kernel::ffi::GetTickCount64() }
}

/// [`GetUserName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamew)
/// function.
#[must_use]
pub fn GetUserName() -> SysResult<String> {
	let mut name_sz = u32::default();

	unsafe { kernel::ffi::GetUserNameW(std::ptr::null_mut(), &mut name_sz); }
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut name_buf = WString::new_alloc_buf(name_sz as _);

	bool_to_sysresult(
		unsafe { kernel::ffi::GetUserNameW(name_buf.as_mut_ptr(), &mut name_sz) },
	).map(|_| name_buf.to_string())
}

/// [`GetVolumeInformation`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumeinformationw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::{co, GetVolumeInformation};
///
/// let mut name = String::default();
/// let mut serial_no = u32::default();
/// let mut max_comp_len = u32::default();
/// let mut sys_flags = co::FILE_VOL::default();
/// let mut sys_name = String::default();
///
/// GetVolumeInformation(
///     Some("C:\\"),
///     Some(&mut name), Some(&mut serial_no), Some(&mut max_comp_len),
///     Some(&mut sys_flags), Some(&mut sys_name),
/// )?;
///
/// println!("Name: {}", name);
/// println!("Serial no: {:#010x}", serial_no);
/// println!("Max comp len: {}", max_comp_len);
/// println!("Sys flags: {:?}", sys_flags);
/// println!("Sys name: {}", sys_name);
/// # Ok::<_, co::ERROR>(())
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
		None => (WString::default(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};
	let mut sys_name_buf = match file_system_name {
		None => (WString::default(), 0),
		Some(_) => (WString::new_alloc_buf(MAX_PATH + 1), MAX_PATH + 1),
	};

	bool_to_sysresult(
		unsafe {
			kernel::ffi::GetVolumeInformationW(
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
			kernel::ffi::GetVolumePathNameW(
				WString::from_str(file_name).as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		} as _,
	).map(|_| buf.to_string())
}

/// [`GetWindowsAccountDomainSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-getwindowsaccountdomainsid)
/// function.
#[must_use]
pub fn GetWindowsAccountDomainSid(sid: &SID) -> SysResult<SidGuard> {
	let mut ad_sid_sz = u32::default();

	unsafe {
		kernel::ffi::GetWindowsAccountDomainSid(
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut ad_sid_sz,
		)
	};
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut ad_sid_buf = vec![0u8; ad_sid_sz as _];

	unsafe {
		bool_to_sysresult(
			kernel::ffi::GetWindowsAccountDomainSid(
				sid as *const _ as _,
				ad_sid_buf.as_mut_ptr(),
				&mut ad_sid_sz,
			),
		).map(|_| SidGuard::new(ad_sid_buf))
	}
}

/// [`GlobalMemoryStatusEx`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex)
/// function.
pub fn GlobalMemoryStatusEx(msx: &mut MEMORYSTATUSEX) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { kernel::ffi::GlobalMemoryStatusEx(msx as *mut _ as _) },
	)
}

/// [`HIBYTE`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632656(v=vs.85))
/// function. Originally a macro.
#[must_use]
pub const fn HIBYTE(v: u16) -> u8 {
	(v >> 8 & 0xff) as _
}

/// Returns the high-order `u32` of an `u64`.
#[must_use]
pub const fn HIDWORD(v: u64) -> u32 {
	(v >> 32 & 0xffff_ffff) as _
}

/// [`HIWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632657(v=vs.85))
/// function. Originally a macro.
#[must_use]
pub const fn HIWORD(v: u32) -> u16 {
	(v >> 16 & 0xffff) as _
}

/// [`InitializeSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor)
/// function.
///
/// You don't need to call this function directly, because
/// [`SECURITY_DESCRIPTOR`](crate::SECURITY_DESCRIPTOR) implements the
/// [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) trait,
/// which calls it.
#[must_use]
pub fn InitializeSecurityDescriptor() -> SysResult<SECURITY_DESCRIPTOR> {
	let mut sd = unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() };
	bool_to_sysresult(
		unsafe {
			kernel::ffi::InitializeSecurityDescriptor(
				&mut sd as *mut _ as _,
				SECURITY_DESCRIPTOR_REVISION,
			)
		},
	).map(|_| sd)
}

/// [`IsDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)
/// function.
#[must_use]
pub fn IsDebuggerPresent() -> bool {
	unsafe { kernel::ffi::IsDebuggerPresent() != 0 }
}

/// [`IsNativeVhdBoot`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-isnativevhdboot)
/// function.
#[must_use]
pub fn IsNativeVhdBoot() -> SysResult<bool> {
	let mut is_native: BOOL = 0;
	bool_to_sysresult(
		unsafe { kernel::ffi::IsNativeVhdBoot(&mut is_native) },
	).map(|_| is_native != 0)
}

/// [`IsValidSecurityDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsecuritydescriptor)
/// function.
#[must_use]
pub fn IsValidSecurityDescriptor(sd: &SECURITY_DESCRIPTOR) -> bool {
	unsafe { kernel::ffi::IsValidSecurityDescriptor(sd as *const _ as _) != 0 }
}

/// [`IsValidSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-isvalidsid)
/// function.
#[must_use]
pub fn IsValidSid(sid: &SID) -> SysResult<bool> {
	match unsafe { kernel::ffi::IsValidSid(sid as *const _ as _) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`IsWellKnownSid`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-iswellknownsid)
/// function.
#[must_use]
pub fn IsWellKnownSid(
	sid: &SID, well_known_sid: co::WELL_KNOWN_SID_TYPE) -> bool
{
	unsafe {
		kernel::ffi::IsWellKnownSid(sid as *const _ as _, well_known_sid.raw())
			!= 0
	}
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
/// function. Originally a macro.
#[must_use]
pub const fn LOBYTE(v: u16) -> u8 {
	(v & 0xff) as _
}

/// Returns the low-order `u32` of an `u64`.
#[must_use]
pub const fn LODWORD(v: u64) -> u32 {
	(v & 0xffff_ffff) as _
}

/// [`LookupAccountName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountnamew)
/// function.
///
/// Returns account's domain name, `SID` and type, respectively.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetUserName, LookupAccountName};
///
/// let user_name = GetUserName()?;
/// let (domain_name, sid, kind) = LookupAccountName(None, &user_name)?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[must_use]
pub fn LookupAccountName(
	system_name: Option<&str>,
	account_name: &str,
) -> SysResult<(String, SidGuard, co::SID_NAME_USE)>
{
	let mut sid_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		kernel::ffi::LookupAccountNameW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			WString::from_str(account_name).as_ptr(),
			std::ptr::null_mut(),
			&mut sid_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut sid_buf = vec![0u8; sid_sz as _];
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	unsafe {
		bool_to_sysresult(
			kernel::ffi::LookupAccountNameW(
				WString::from_opt_str(system_name).as_ptr(),
				WString::from_str(account_name).as_ptr(),
				sid_buf.as_mut_ptr(),
				&mut sid_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				sid_name_use.as_mut(),
			),
		).map(|_| (domain_buf.to_string(), SidGuard::new(sid_buf), sid_name_use))
	}
}

/// [`LookupAccountSid`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupaccountsidw)
/// function.
///
/// Returns account name, domain name and type, respectively.
#[must_use]
pub fn LookupAccountSid(
	system_name: Option<&str>,
	sid: &SID,
) -> SysResult<(String, String, co::SID_NAME_USE)>
{
	let mut account_sz = u32::default();
	let mut domain_sz = u32::default();
	let mut sid_name_use = co::SID_NAME_USE::default();

	unsafe {
		kernel::ffi::LookupAccountSidW( // retrieve needed buffer sizes
			WString::from_opt_str(system_name).as_ptr(),
			sid as *const _ as _,
			std::ptr::null_mut(),
			&mut account_sz,
			std::ptr::null_mut(),
			&mut domain_sz,
			sid_name_use.as_mut(),
		);
	}
	let get_size_err = GetLastError();
	if get_size_err != co::ERROR::INSUFFICIENT_BUFFER {
		return Err(get_size_err);
	}

	let mut account_buf = WString::new_alloc_buf(account_sz as _);
	let mut domain_buf = WString::new_alloc_buf(domain_sz as _);

	bool_to_sysresult(
		unsafe {
			kernel::ffi::LookupAccountSidW(
				WString::from_opt_str(system_name).as_ptr(),
				sid as *const _ as _,
				account_buf.as_mut_ptr(),
				&mut account_sz,
				domain_buf.as_mut_ptr(),
				&mut domain_sz,
				sid_name_use.as_mut(),
			)
		},
	).map(|_| (account_buf.to_string(), domain_buf.to_string(), sid_name_use))
}

/// [`LookupPrivilegeValue`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)
/// function.
#[must_use]
pub fn LookupPrivilegeValue(
	system_name: Option<&str>, name: co::SE_PRIV) -> SysResult<LUID>
{
	let mut luid = LUID::new(0, 0);
	bool_to_sysresult(
		unsafe {
			kernel::ffi::LookupPrivilegeValueW(
				WString::from_opt_str(system_name).as_ptr(),
				name.wstr().as_ptr(),
				&mut luid as *mut _ as _,
			)
		},
	).map(|_| luid)
}

/// [`LOWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632659(v=vs.85))
/// function. Originally a macro.
#[must_use]
pub const fn LOWORD(v: u32) -> u16 {
	(v & 0xffff) as _
}

/// Function that implements
/// [`MAKELONG`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632660(v=vs.85)),
/// [`MAKEWPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makewparam),
/// and
/// [`MAKELPARAM`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makelparam)
/// macros.
#[must_use]
pub const fn MAKEDWORD(lo: u16, hi: u16) -> u32 {
	((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}

/// Similar to [`MAKEDWORD`](crate::MAKEDWORD), but for `u64`.
#[must_use]
pub const fn MAKEQWORD(lo: u32, hi: u32) -> u64 {
	((lo as u64 & 0xffff_ffff) | ((hi as u64 & 0xffff_ffff) << 32)) as _
}

/// [`MAKEWORD`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms632663(v=vs.85))
/// function. Originally a macro.
#[must_use]
pub const fn MAKEWORD(lo: u8, hi: u8) -> u16 {
	(lo as u16 & 0xff) | ((hi as u16 & 0xff) << 8) as u16
}

/// [`MoveFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefilew)
/// function.
pub fn MoveFile(existing_file: &str, new_file: &str) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			kernel::ffi::MoveFileW(
				WString::from_str(existing_file).as_ptr(),
				WString::from_str(new_file).as_ptr(),
			)
		},
	)
}

/// [`MulDiv`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-muldiv)
/// function.
#[must_use]
pub fn MulDiv(number: i32, numerator: i32, denominator: i32) -> i32 {
	unsafe { kernel::ffi::MulDiv(number, numerator, denominator) }
}

/// [`MultiByteToWideChar`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
#[must_use]
pub fn MultiByteToWideChar(
	code_page: co::CP,
	flags: co::MBC,
	multi_byte_str: &[u8],
) -> SysResult<Vec<u16>>
{
	let num_bytes = match unsafe {
		kernel::ffi::MultiByteToWideChar(
			code_page.raw() as _,
			flags.raw(),
			multi_byte_str.as_ptr(),
			multi_byte_str.len() as _,
			std::ptr::null_mut(),
			0,
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}? + 1; // room for terminating null

	let mut buf = vec![0u16; num_bytes as _];

	bool_to_sysresult(
		unsafe {
			kernel::ffi::MultiByteToWideChar(
				code_page.raw() as _,
				flags.raw(),
				multi_byte_str.as_ptr(),
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
		kernel::ffi::OutputDebugStringW(WString::from_str(output_string).as_ptr())
	}
}

/// [`QueryPerformanceCounter`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
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
#[must_use]
pub fn QueryPerformanceCounter() -> SysResult<i64> {
	let mut perf_count = i64::default();
	bool_to_sysresult(
		unsafe { kernel::ffi::QueryPerformanceCounter(&mut perf_count) },
	).map(|_| perf_count)
}

/// [`QueryPerformanceFrequency`](https://learn.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter)
/// function.
///
/// Usually used with
/// [`QueryPerformanceCounter`](crate::QueryPerformanceCounter).
#[must_use]
pub fn QueryPerformanceFrequency() -> SysResult<i64> {
	let mut freq = i64::default();
	bool_to_sysresult(
		unsafe { kernel::ffi::QueryPerformanceFrequency(&mut freq) },
	).map(|_| freq)
}

/// [`ReplaceFile`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
/// function.
pub fn ReplaceFile(
	replaced: &str,
	replacement: &str,
	backup: Option<&str>,
	flags: co::REPLACEFILE,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::ReplaceFileW(
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
			kernel::ffi::SetCurrentDirectoryW(WString::from_str(path_name).as_ptr())
		},
	)
}

/// [`SetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
/// function.
pub fn SetLastError(err_code: co::ERROR) {
	unsafe { kernel::ffi::SetLastError(err_code.raw()) }
}

/// [`SetThreadStackGuarantee`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadstackguarantee)
/// function.
///
/// Returns the size of the previous stack.
pub fn SetThreadStackGuarantee(stack_size_in_bytes: u32) -> SysResult<u32> {
	let mut sz = stack_size_in_bytes;
	bool_to_sysresult(unsafe { kernel::ffi::SetThreadStackGuarantee(&mut sz) })
		.map(|_| sz)
}

/// [`Sleep`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep)
/// function.
pub fn Sleep(milliseconds: u32) {
	unsafe { kernel::ffi::Sleep(milliseconds) }
}

/// [`SwitchToThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-switchtothread)
/// function.
pub fn SwitchToThread() -> SysResult<()> {
	bool_to_sysresult(unsafe { kernel::ffi::SwitchToThread() })
}

/// [`SystemTimeToFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetofiletime)
/// function.
pub fn SystemTimeToFileTime(
	st: &SYSTEMTIME, ft: &mut FILETIME) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::SystemTimeToFileTime(
				st as *const _ as _,
				ft as *mut _ as _,
			)
		},
	)
}

/// [`SystemTimeToTzSpecificLocalTime`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-systemtimetotzspecificlocaltime)
/// function.
pub fn SystemTimeToTzSpecificLocalTime(
	time_zone: Option<&TIME_ZONE_INFORMATION>,
	universal_time: &SYSTEMTIME,
	local_time: &mut SYSTEMTIME,
) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			kernel::ffi::SystemTimeToTzSpecificLocalTime(
				time_zone.map_or(std::ptr::null(), |lp| lp as *const _ as _),
				universal_time as *const _ as _,
				local_time as *mut _ as _,
			)
		},
	)
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
		kernel::ffi::VerifyVersionInfoW(
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
	condition_mask: u64, type_mask: co::VER_MASK, condition: co::VER_COND) -> u64
{
	unsafe {
		kernel::ffi::VerSetConditionMask(
			condition_mask,
			type_mask.raw(),
			condition.raw(),
		)
	}
}

/// [`WideCharToMultiByte`](https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte)
/// function.
///
/// The resulting `Vec<u16>` includes a terminating null.
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
		kernel::ffi::WideCharToMultiByte(
			code_page.raw() as _,
			flags.raw(),
			wide_char_str.as_ptr(),
			wide_char_str.len() as _,
			std::ptr::null_mut(),
			0,
			&mut default_char_buf,
			std::ptr::null_mut(),
		)
	} {
		0 => Err(GetLastError()),
		num_bytes => Ok(num_bytes),
	}? + 1; // room for terminating null

	let mut u8_buf = vec![0u8; num_bytes as _];
	let mut bool_buf: BOOL = 0;

	bool_to_sysresult(
		unsafe {
			kernel::ffi::WideCharToMultiByte(
				code_page.raw() as _,
				flags.raw(),
				wide_char_str.as_ptr(),
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
