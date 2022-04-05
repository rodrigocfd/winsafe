#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::kernel::decl::{HEVENT, HPIPE, HPROCESS, HTHREAD, MAKEQWORD};
use crate::kernel::privs::MAX_PATH;

/// [`ACL`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
#[derive(Default)]
pub struct ACL {
	pub AclRevision: u8,
	pub Sbz1: u8,
	pub AclSize: u16,
	pub AceCount: u16,
	pub Sbz2: u16,
}

/// [`BY_HANDLE_FILE_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
#[derive(Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub ftLastWriteTime: FILETIME,
	pub dwVolumeSerialNumber: u32,
	pub nFileSizeHigh: u32,
	pub nFileSizeLow: u32,
	pub nNumberOfLinks: u32,
	pub nFileIndexHigh: u32,
	pub nFileIndexLow: u32,
}

/// [`FILETIME`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

/// [`LANGID`](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifiers)
/// language identifier.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LANGID(pub(crate) u16);

impl From<LANGID> for u16 {
	fn from(v: LANGID) -> Self {
		v.0
	}
}

impl LANGID {
	/// [`LANGID`](crate::LANGID) composed of
	/// [`LANG::NEUTRAL`](crate::co::LANG::NEUTRAL) and
	/// [`SUBLANG::SYS_DEFAULT`](crate::co::SUBLANG::SYS_DEFAULT).
	pub const SYSTEM_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::SYS_DEFAULT);

	/// [`LANGID`](crate::LANGID) composed of
	/// [`LANG::NEUTRAL`](crate::co::LANG::NEUTRAL) and
	/// [`SUBLANG::DEFAULT`](crate::co::SUBLANG::DEFAULT).
	pub const USER_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::DEFAULT);

	/// Creates a new `LANGID`. Originally
	/// [`MAKELANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	pub const fn new(lang: co::LANG, sublang: co::SUBLANG) -> LANGID {
		Self((sublang.0 << 10) | lang.0)
	}

	/// Returns the primary language ID. Originally
	/// [`PRIMARYLANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-primarylangid)
	/// macro.
	pub const fn primary_lang_id(self) -> co::LANG {
		co::LANG(self.0 & 0x3ff)
	}

	/// Returns the sublanguage ID. Originally
	/// [`SUBLANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sublangid)
	/// macro.
	pub const fn sub_lang_id(self) -> co::SUBLANG {
		co::SUBLANG(self.0 >> 10)
	}
}

/// [`LCID`](https://docs.microsoft.com/en-us/windows/win32/intl/locale-identifiers)
/// locale identifier.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LCID(pub(crate) u32);

impl LCID {
	/// [`LCID`](crate::LCID) composed of
	/// [`LANGID::SYSTEM_DEFAULT`](crate::LANGID::SYSTEM_DEFAULT) and
	/// [`SORT::DEFAULT`](crate::co::SORT::DEFAULT).
	pub const SYSTEM_DEFAULT: Self = Self::new(LANGID::SYSTEM_DEFAULT, co::SORT::DEFAULT);

	/// [`LCID`](crate::LCID) composed of
	/// [`LANGID::USER_DEFAULT`](crate::LANGID::USER_DEFAULT) and
	/// [`SORT::DEFAULT`](crate::co::SORT::DEFAULT).
	pub const USER_DEFAULT: Self = Self::new(LANGID::USER_DEFAULT, co::SORT::DEFAULT);

	/// Creates a new `LCID`. Originally
	/// [`MAKELCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelcid)
	/// macro.
	pub const fn new(lang_id: LANGID, sort_id: co::SORT) -> LCID {
		Self(((sort_id.0 as u32) << 16) | lang_id.0 as u32)
	}

	/// Returns the language identifier. Originally
	/// [`LANGIDFROMLCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-langidfromlcid)
	/// macro.
	pub const fn lang_id(self) -> LANGID {
		LANGID(self.0 as _)
	}

	/// Returns the sort ID. Originally
	/// [`SORTIDFROMLCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sortidfromlcid)
	/// macro.
	pub const fn sort_id(self) -> co::SORT {
		co::SORT(((self.0 >> 16) & 0xf) as _)
	}
}

/// [`MEMORYSTATUSEX`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct MEMORYSTATUSEX {
	dwLength: u32,
	pub dwMemoryLoad: u32,
	pub ullTotalPhys: u64,
	pub ullAvailPhys: u64,
	pub ullTotalPageFile: u64,
	pub ullAvailPageFile: u64,
	pub ullTotalVirtual: u64,
	pub ullAvailVirtual: u64,
	pub ullAvailExtendedVirtual: u64,
}

impl_default_with_size!(MEMORYSTATUSEX, dwLength);

/// [`OSVERSIONINFOEX`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct OSVERSIONINFOEX {
	dwOSVersionInfoSize: u32,
	pub dwMajorVersion: u32,
	pub dwMinorVersion: u32,
	pub dwBuildNumber: u32,
	pub dwPlatformId: co::VER_PLATFORM,
	szCSDVersion: [u16; 128],
	pub wServicePackMajor: u16,
	pub wServicePackMinor: u16,
	pub wSuiteMask: co::VER_SUITE,
	pub wProductType: co::VER_NT,
	wReserved: u8,
}

impl_default_with_size!(OSVERSIONINFOEX, dwOSVersionInfoSize);

impl OSVERSIONINFOEX {
	pub_fn_string_arr_get_set!(szCSDVersion, set_szCSDVersion);
}

/// [`OVERLAPPED`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct OVERLAPPED {
	pub Internal: usize,
	pub InternalHigh: usize,
	pub Pointer: usize,
	pub hEvent: HEVENT,
}

impl_default!(OVERLAPPED);

/// [`PROCESS_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct PROCESS_INFORMATION {
	pub hProcess: HPROCESS,
	pub hThread: HTHREAD,
	pub dwProcessId: u32,
	pub dwThreadId: u32,
}

impl_default!(PROCESS_INFORMATION);

/// [`SECURITY_DESCRIPTOR`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct SECURITY_DESCRIPTOR {
	pub Revision: u8,
   pub Sbz1: u8,
   pub Control: u16,
   pub Owner: *mut std::ffi::c_void,
   pub Group: *mut std::ffi::c_void,
   pub Sacl: *mut ACL,
   pub Dacl: *mut ACL,
}

/// [`PROCESSENTRY32`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-processentry32w)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct PROCESSENTRY32 {
	dwSize: u32,
	cntUsage: u32,
	pub th32ProcessID: u32,
	th32DefaultHeapID: u64,
	th32ModuleID: u32,
	pub cntThreads: u32,
	pub th32ParentProcessID: u32,
	pub pcPriClassBase: i32,
	dwFlags: u32,
	szExeFile: [u16; MAX_PATH],
}

impl_default_with_size!(PROCESSENTRY32, dwSize);

impl PROCESSENTRY32 {
	pub_fn_string_arr_get_set!(szExeFile, set_szExeFile);
}

/// [`SECURITY_ATTRIBUTES`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct SECURITY_ATTRIBUTES<'a> {
	nLength: u32,
	lpSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
	bInheritHandle: i32,

	_lpSecurityDescriptor: PhantomData<&'a mut SECURITY_DESCRIPTOR>,
}

impl_default_with_size!(SECURITY_ATTRIBUTES, nLength, 'a);

impl<'a> SECURITY_ATTRIBUTES<'a> {
	pub_fn_ptr_get_set!('a, lpSecurityDescriptor, set_lpSecurityDescriptor, SECURITY_DESCRIPTOR);
	pub_fn_bool_get_set!(bInheritHandle, set_bInheritHandle);
}

/// [`STARTUPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct STARTUPINFO<'a, 'b> {
	cb: u32,
	lpReserved: *mut u16,
	lpDesktop: *mut u16,
	lpTitle: *mut u16,
	pub dwX: u32,
	pub dwY: u32,
	pub dwXSize: u32,
	pub dwYSize: u32,
	pub dwXCountChars: u32,
	pub dwYCountChars: u32,
	pub dwFillAttribute: u32,
	pub dwFlags: co::STARTF,
	wShowWindow: u16, // co::SW, should be 32-bit
	cbReserved2: u16,
	lpReserved2: *mut u8,
	pub hStdInput: HPIPE,
	pub hStdOutput: HPIPE,
	pub hStdError: HPIPE,

	_lpDesktop: PhantomData<&'a mut u16>,
	_lpTitle: PhantomData<&'b mut u16>,
}

impl_default_with_size!(STARTUPINFO, cb, 'a, 'b);

impl<'a, 'b> STARTUPINFO<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, lpDesktop, set_lpDesktop);
	pub_fn_string_ptr_get_set!('a, lpTitle, set_lpTitle);

	/// Returns the `wShowWindow` field.
	pub fn wShowWindow(&self) -> co::SW {
		co::SW(self.wShowWindow as _)
	}

	/// Sets the `wShowWindow` field.
	pub fn set_wShowWindow(&mut self, val: co::SW) {
		self.wShowWindow = val.0 as _;
	}
}

/// [`SYSTEM_INFO`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct SYSTEM_INFO {
	pub wProcessorArchitecture: co::PROCESSOR_ARCHITECTURE,
	wReserved: u16,
	pub dwPageSize: u32,
	pub lpMinimumApplicationAddress: *mut std::ffi::c_void,
	pub lpMaximumApplicationAddress: *mut std::ffi::c_void,
	pub dwActiveProcessorMask: usize,
	pub dwNumberOfProcessors: u32,
	pub dwProcessorType: co::PROCESSOR,
	pub dwAllocationGranularity: u32,
	pub wProcessorLevel: u16,
	pub wProcessorRevision: u16,
}

impl_default!(SYSTEM_INFO);

/// [`SYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct SYSTEMTIME {
	pub wYear: u16,
	pub wMonth: u16,
	pub wDayOfWeek: u16,
	pub wDay: u16,
	pub wHour: u16,
	pub wMinute: u16,
	pub wSecond: u16,
	pub wMilliseconds: u16,
}

/// [`TIME_ZONE_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
#[derive(Default)]
pub struct TIME_ZONE_INFORMATION {
	pub bias: i32,
	standardName: [u16; 32],
	pub standardDate: SYSTEMTIME,
	pub standardBias: i32,
	daylightName: [u16; 32],
	pub daylightDate: SYSTEMTIME,
	pub daylightBias: i32,
}

impl TIME_ZONE_INFORMATION {
	pub_fn_string_arr_get_set!(standardName, set_standardName);
	pub_fn_string_arr_get_set!(daylightName, set_daylightName);
}

/// [`WIN32_FIND_DATA`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-win32_find_dataw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[repr(C)]
pub struct WIN32_FIND_DATA {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub tLastWriteTime: FILETIME,
	nFileSizeHigh: u32,
	nFileSizeLow: u32,
	dwReserved0: u32,
	dwReserved1: u32,
	cFileName: [u16; MAX_PATH],
	cAlternateFileName: [u16; 14],
}

impl_default!(WIN32_FIND_DATA);

impl WIN32_FIND_DATA {
	pub_fn_string_arr_get_set!(cFileName, set_cFileName);
	pub_fn_string_arr_get_set!(cAlternateFileName, set_cAlternateFileName);

	/// Returns the nFileSizeHigh and nFileSizeLow fields.
	pub fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}
}
