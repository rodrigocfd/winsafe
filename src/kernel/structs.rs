#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::kernel::decl::{
	ConvertSidToStringSid, HEVENT, HINSTANCE, HPIPE, HPROCESS, HTHREAD,
	InitializeSecurityDescriptor, MAKEQWORD, WString,
};
use crate::kernel::privs::{MAX_MODULE_NAME32, MAX_PATH};

/// [`ACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct ACL {
	pub AclRevision: u8,
	pub Sbz1: u8,
	pub AclSize: u16,
	pub AceCount: u16,
	pub Sbz2: u16,
}

/// [`BY_HANDLE_FILE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information)
/// struct.
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

/// [`FILETIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

/// [`GUID`](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)
/// struct.
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct GUID {
	data1: u32,
	data2: u16,
	data3: u16,
	data4: u64,
}

impl std::fmt::Display for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
			self.data1, self.data2, self.data3,
			self.data4.swap_bytes() >> 48,
			self.data4.swap_bytes() & 0x0000_ffff_ffff_ffff,
		)
	}
}

impl Default for GUID {
	fn default() -> Self {
		Self::new("00000000-0000-0000-c000-000000000046") // IUnknown GUID
	}
}

impl GUID {
	/// Creates a new `GUID` from a representative hex string, which can be
	/// copied straight from standard `GUID` declarations.
	///
	/// # Panics
	///
	/// Panics if the string has an invalid format.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::GUID;
	///
	/// let g = GUID::new("00000000-0000-0000-c000-000000000046");
	/// ```
	#[must_use]
	pub const fn new(guid_str: &str) -> Self {
		if guid_str.len() != 36 {
			panic!("Bad number of GUID chars.");
		}

		let chs = guid_str.as_bytes();
		let p1 = Self::parse_block([chs[0], chs[1], chs[2], chs[3], chs[4],
			chs[5], chs[6], chs[7]]);
		let p2 = Self::parse_block([chs[9], chs[10], chs[11], chs[12]]);
		let p3 = Self::parse_block([chs[14], chs[15], chs[16], chs[17]]);
		let p4 = Self::parse_block([chs[19], chs[20], chs[21], chs[22]]);
		let p5 = Self::parse_block([chs[24], chs[25], chs[26], chs[27], chs[28],
			chs[29], chs[30], chs[31], chs[32], chs[33], chs[34], chs[35]]);

		Self {
			data1: p1 as _,
			data2: p2 as _,
			data3: p3 as _,
			data4: ((p4 << 48) | p5).swap_bytes(),
		}
	}

	const fn parse_block<const N: usize>(chars: [u8; N]) -> u64 {
		let mut res: u64 = 0;
		let mut idx: usize = 0;
		while idx < N {
			let ch = chars[idx];
			if !Self::valid_char(ch) {
				panic!("Bad GUID char.");
			}
			res += Self::char_to_num(ch) * 16_u64.pow((N - idx - 1) as _);
			idx += 1;
		}
		res
	}

	const fn valid_char(ch: u8) -> bool {
		(ch >= 48 && ch <= 57) // 0-9
			|| (ch >= 65 && ch <= 70) // A-F
			|| (ch >= 97 && ch <= 102) // a-f
	}

	const fn char_to_num(ch: u8) -> u64 {
		if ch >= 48 && ch <= 57 {
			ch as u64 - 48
		} else if ch >= 65 && ch <= 70 {
			ch as u64 - 65 + 10
		} else if ch >= 97 && ch <= 102 {
			ch as u64 - 97 + 10
		} else {
			panic!("Bad GUID char in conversion.");
		}
	}
}

/// [`HEAPLIST32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-heaplist32)
/// struct.
#[repr(C)]
pub struct HEAPLIST32 {
	dwSize: usize,
	pub th32ProcessID: u32,
	pub th32HeapID: usize,
	pub dwFlags: co::HF32,
}

impl_default_with_size!(HEAPLIST32, dwSize);

/// [`LANGID`](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifiers)
/// language identifier.
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
	/// [`MAKELANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	#[must_use]
	pub const fn new(lang: co::LANG, sublang: co::SUBLANG) -> Self {
		Self((sublang.0 << 10) | lang.0)
	}

	/// Returns the primary language ID. Originally
	/// [`PRIMARYLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-primarylangid)
	/// macro.
	#[must_use]
	pub const fn primary_lang_id(self) -> co::LANG {
		co::LANG(self.0 & 0x3ff)
	}

	/// Returns the sublanguage ID. Originally
	/// [`SUBLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sublangid)
	/// macro.
	#[must_use]
	pub const fn sub_lang_id(self) -> co::SUBLANG {
		co::SUBLANG(self.0 >> 10)
	}
}

/// [`LCID`](https://learn.microsoft.com/en-us/windows/win32/intl/locale-identifiers)
/// locale identifier.
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
	/// [`MAKELCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelcid)
	/// macro.
	#[must_use]
	pub const fn new(lang_id: LANGID, sort_id: co::SORT) -> Self {
		Self(((sort_id.0 as u32) << 16) | lang_id.0 as u32)
	}

	/// Returns the language identifier. Originally
	/// [`LANGIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-langidfromlcid)
	/// macro.
	#[must_use]
	pub const fn lang_id(self) -> LANGID {
		LANGID(self.0 as _)
	}

	/// Returns the sort ID. Originally
	/// [`SORTIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sortidfromlcid)
	/// macro.
	#[must_use]
	pub const fn sort_id(self) -> co::SORT {
		co::SORT(((self.0 >> 16) & 0xf) as _)
	}
}

/// [`LUID`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-luid)
/// identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LUID {
	LowPart: u32,
	HighPart: i32,
}

impl LUID {
	pub const SYSTEM: Self = Self::new(0x3e7, 0x0);
	pub const ANONYMOUS_LOGON: Self = Self::new(0x3e6, 0x0);
	pub const LOCALSERVICE: Self = Self::new(0x3e5, 0x0);
	pub const NETWORKSERVICE: Self = Self::new(0x3e4, 0x0);
	pub const IUSER: Self = Self::new(0x3e3, 0x0);
	pub const PROTECTED_TO_SYSTEM: Self = Self::new(0x3e2, 0x0);

	/// Creates a new `LUID`.
	#[must_use]
	pub const fn new(low_part: u32, high_part: i32) -> Self {
		Self { LowPart: low_part, HighPart: high_part }
	}

	/// Returns the low part.
	#[must_use]
	pub const fn low_part(&self) -> u32 {
		self.LowPart
	}

	/// Returns the high part.
	#[must_use]
	pub const fn high_part(&self) -> i32 {
		self.HighPart
	}
}

/// [`MODULEENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-moduleentry32w)
/// struct.
#[repr(C)]
pub struct MODULEENTRY32 {
	dwSize: u32,
	th32ModuleID: u32,
	pub th32ProcessID: u32,
	pub GlblcntUsage: u32,
	pub ProccntUsage: u32,
	pub modBaseAddr: *mut std::ffi::c_void,
	pub modBaseSize: u32,
	pub hModule: HINSTANCE,
	szModule: [u16; MAX_MODULE_NAME32 + 1],
	szExePath: [u16; MAX_PATH],
}

impl_default_with_size!(MODULEENTRY32, dwSize);

impl MODULEENTRY32 {
	pub_fn_string_arr_get_set!(szModule, set_szModule);
	pub_fn_string_arr_get_set!(szExePath, set_szExePath);
}

/// [`MEMORYSTATUSEX`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex)
/// struct.
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

/// [`OSVERSIONINFOEX`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
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

/// [`OVERLAPPED`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped)
/// struct.
#[repr(C)]
pub struct OVERLAPPED {
	pub Internal: usize,
	pub InternalHigh: usize,
	pub Pointer: usize,
	pub hEvent: HEVENT,
}

impl_default!(OVERLAPPED);

/// [`PROCESS_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information)
/// struct.
#[repr(C)]
pub struct PROCESS_INFORMATION {
	pub hProcess: HPROCESS,
	pub hThread: HTHREAD,
	pub dwProcessId: u32,
	pub dwThreadId: u32,
}

impl_default!(PROCESS_INFORMATION);

/// [`PROCESSENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-processentry32w)
/// struct.
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

/// [`SECURITY_ATTRIBUTES`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
/// struct.
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

/// [`SECURITY_DESCRIPTOR`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)
/// struct.
#[repr(C)]
pub struct SECURITY_DESCRIPTOR {
	pub Revision: u8,
	pub Sbz1: u8,
	pub Control: co::SE,
	pub Owner: *mut std::ffi::c_void,
	pub Group: *mut std::ffi::c_void,
	pub Sacl: *mut ACL,
	pub Dacl: *mut ACL,
}

impl Default for SECURITY_DESCRIPTOR {
	fn default() -> Self {
		InitializeSecurityDescriptor().unwrap()
	}
}

/// [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
/// struct.
/// 
/// Note that you cannot directly instantiate this struct, because the
/// `SubAuthority` field is dynamically allocated. There are two types of allocations:
/// 
/// * handled by the OS, which yields a [`FreeSidGuard`](crate::guard::FreeSidGuard);
/// * handled by WinSafe, which yields a [`SidGuard`](crate::guard::SidGuard).
#[repr(C)]
pub struct SID {
	pub Revision: u8,
	SubAuthorityCount: u8,
	pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
	SubAuthority: [co::RID; 1],
}

impl std::fmt::Display for SID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match ConvertSidToStringSid(self) {
			Ok(name) => write!(f, "{}", name),
			Err(err) => write!(f, "{}", err),
		}
	}
}

impl SID {
	/// Returns the `SubAuthorityCount` field.
	#[must_use]
	pub fn SubAuthorityCount(&self) -> u8 {
		self.SubAuthority().len() as _
	}

	/// Returns the `SubAuthority` field.
	#[must_use]
	pub fn SubAuthority(&self) -> &[co::RID] {
		unsafe {
			std::slice::from_raw_parts(
				self.SubAuthority.as_ptr(), self.SubAuthorityCount as _)
		}
	}
}

/// [`SID_IDENTIFIER_AUTHORITY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_identifier_authority)
/// struct.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SID_IDENTIFIER_AUTHORITY {
	pub Value: [u8; 6],
}

impl std::fmt::Display for SID_IDENTIFIER_AUTHORITY {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self.Value) // delegate to array Debug
	}
}

macro_rules! predef_sid_ident_au {
	($name:ident, $val:expr) => {
		/// Predefined `SID_IDENTIFIER_AUTHORITY`. Originally has `SECURITY`
		/// prefix and `AUTHORITY` suffix.
		pub const $name: Self = Self { Value: $val };
	};
}

impl SID_IDENTIFIER_AUTHORITY {
	predef_sid_ident_au!(NULL, [0, 0, 0, 0, 0, 0]);
	predef_sid_ident_au!(WORLD, [0, 0, 0, 0, 0, 1]);
	predef_sid_ident_au!(LOCAL, [0, 0, 0, 0, 0, 2]);
	predef_sid_ident_au!(CREATOR, [0, 0, 0, 0, 0, 3]);
	predef_sid_ident_au!(NON_UNIQUE, [0, 0, 0, 0, 0, 4]);
	predef_sid_ident_au!(RESOURCE_MANAGER, [0, 0, 0, 0, 0, 9]);
	predef_sid_ident_au!(NT, [0, 0, 0, 0, 0, 5]);
	predef_sid_ident_au!(APP_PACKAGE, [0, 0, 0, 0, 0, 15]);
	predef_sid_ident_au!(MANDATORY_LABEL, [0, 0, 0, 0, 0, 16]);
	predef_sid_ident_au!(SCOPED_POLICY_ID, [0, 0, 0, 0, 0, 17]);
	predef_sid_ident_au!(AUTHENTICATION, [0, 0, 0, 0, 0, 18]);
	predef_sid_ident_au!(PROCESS_TRUST, [0, 0, 0, 0, 0, 19]);
}

/// [`STARTUPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow)
/// struct.
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
	#[must_use]
	pub const fn wShowWindow(&self) -> co::SW {
		co::SW(self.wShowWindow as _)
	}

	/// Sets the `wShowWindow` field.
	pub fn set_wShowWindow(&mut self, val: co::SW) {
		self.wShowWindow = val.0 as _;
	}
}

/// [`SYSTEM_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
/// struct.
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

/// [`SYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime)
/// struct.
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

/// [`THREADENTRY32`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-threadentry32)
/// struct.
#[repr(C)]
pub struct THREADENTRY32 {
	dwSize: u32,
	cntUsage: u32,
	pub th32ThreadID: u32,
	pub th32OwnerProcessID: u32,
	pub tpBasePri: i32,
	tpDeltaPri: i32,
	dwFlags: u32,
}

impl_default_with_size!(THREADENTRY32, dwSize);

/// [`TIME_ZONE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information)
/// struct.
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

/// [`VALENT`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/ns-winreg-valentw)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct VALENT {
	pub ve_valuename: *mut u16,
	pub ve_valuelen: u32,
	pub ve_valueptr: usize,
	pub ve_type: co::REG,
}

impl_default!(VALENT);

impl VALENT {
	/// Returns a projection over `src`, delimited by `ve_valueptr` and
	/// `ve_valuelen` fields.
	pub unsafe fn buf_projection<'a>(&'a self, src: &'a [u8]) -> &'a [u8] {
		let proj_idx = self.ve_valueptr - src.as_ptr() as usize;
		let proj_past_idx = proj_idx + self.ve_valuelen as usize;
		&src[proj_idx..proj_past_idx]
	}
}

/// [`WIN32_FIND_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-win32_find_dataw)
/// struct.
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
	#[must_use]
	pub const fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}
}
