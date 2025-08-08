#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

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

/// [`ACTCTX`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-actctxw)
/// struct.
#[repr(C)]
pub struct ACTCTX<'a, 'b, 'c, 'd> {
	cbSize: u32,
	pub dwFlags: co::ACTCTX_FLAG,
	lpSource: *mut u16,
	pub wProcessorArchitecture: co::PROCESSOR_ARCHITECTURE,
	pub wLangId: LANGID,
	lpAssemblyDirectory: *mut u16,
	lpResourceName: *mut u16,
	lpApplicationName: *mut u16,
	pub hModule: HINSTANCE,

	_lpSource: PhantomData<&'a mut u16>,
	_lpAssemblyDirectory: PhantomData<&'b mut u16>,
	_lpResourceName: PhantomData<&'c mut u16>,
	_lpApplicationName: PhantomData<&'d mut u16>,
}

impl_default!(ACTCTX, cbSize, 'a, 'b, 'c, 'd);

impl<'a, 'b, 'c, 'd> ACTCTX<'a, 'b, 'c, 'd> {
	pub_fn_string_ptr_get_set!('a, lpSource, set_lpSource);
	pub_fn_string_ptr_get_set!('b, lpAssemblyDirectory, set_lpAssemblyDirectory);
	pub_fn_string_ptr_get_set!('c, lpResourceName, set_lpResourceName);
	pub_fn_string_ptr_get_set!('d, lpApplicationName, set_lpApplicationName);
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
	nFileSizeHigh: u32,
	nFileSizeLow: u32,
	pub nNumberOfLinks: u32,
	nFileIndexHigh: u32,
	nFileIndexLow: u32,
}

impl BY_HANDLE_FILE_INFORMATION {
	/// Returns the nFileSizeHigh and nFileSizeLow fields.
	#[must_use]
	pub const fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}

	/// Sets the nFileSizeHigh and nFileSizeLow fields.
	pub const fn set_nFileSize(&mut self, val: u64) {
		self.nFileSizeHigh = HIDWORD(val);
		self.nFileSizeLow = LODWORD(val);
	}

	/// Returns the nFileIndexHigh and nFileIndexLow fields.
	#[must_use]
	pub const fn nFileIndex(&self) -> u64 {
		MAKEQWORD(self.nFileIndexLow, self.nFileIndexHigh)
	}

	/// Sets the nFileIndexHigh and nFileIndexLow fields.
	pub const fn set_nFileIndex(&mut self, val: u64) {
		self.nFileIndexHigh = HIDWORD(val);
		self.nFileIndexLow = LODWORD(val);
	}
}

/// [`CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_fqbn_value)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a> {
	pub Version: u64,
	Name: *mut u16,

	_Name: PhantomData<&'a mut u16>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE, 'a);

impl<'a> CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a> {
	pub_fn_string_ptr_get_set!('a, Name, set_Name);
}

/// [`CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_octet_string_value)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a> {
	pValue: *mut u8,
	ValueLength: u32,

	_pValue: PhantomData<&'a mut ()>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE, 'a);

impl<'a> CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a> {
	pub_fn_array_buf_get_set!('a, pValue, set_pValue, ValueLength, u8);
}

/// [`CLAIM_SECURITY_ATTRIBUTE_V1`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attribute_v1)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b> {
	Name: *mut u16,
	ValueType: co::CLAIM_SECURITY_ATTRIBUTE_TYPE,
	Reserved: u16,
	Flags: u32,
	ValueCount: u32,
	Values: CLAIM_SECURITY_ATTRIBUTE_V1_union0<'b>,

	_Name: PhantomData<&'a mut u16>,
}

#[repr(C)]
union CLAIM_SECURITY_ATTRIBUTE_V1_union0<'a> {
	pInt64: *mut i64, // pointers because these are all arrays with ValueCount items
	pUint64: *mut u64,
	ppString: *mut *mut u16,
	pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a>,
	pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTE_V1, 'a, 'b);

impl<'a, 'b> CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, Name, set_Name);

	/// Returns the low-word part of `Flags`.
	#[must_use]
	pub const fn FlagsLo(&self) -> co::CLAIM_SECURITY_ATTRIBUTE {
		unsafe { co::CLAIM_SECURITY_ATTRIBUTE::from_raw(LOWORD(self.Flags)) }
	}

	/// Sets the low-word part of `Flags`.
	pub const fn set_FlagsLo(&mut self, claim: co::CLAIM_SECURITY_ATTRIBUTE) {
		self.Flags = MAKEDWORD(claim.raw(), self.FlagsHi());
	}

	/// Returns the high-word part of `Flags`.
	#[must_use]
	pub const fn FlagsHi(&self) -> u16 {
		HIWORD(self.Flags)
	}

	/// Sets the high-word part of `Flags`.
	pub const fn set_FlagsHi(&mut self, flags: u16) {
		self.Flags = MAKEDWORD(self.FlagsLo().raw(), flags);
	}

	/// Returns the `Values` field.
	///
	/// # Panics
	///
	/// Panics if `ValueType` field is invalid.
	#[must_use]
	pub fn Values(&self) -> ClaimSecurityAttr<'_> {
		unsafe {
			match self.ValueType {
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::INT64 => ClaimSecurityAttr::Int64(
					std::slice::from_raw_parts(self.Values.pInt64, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::UINT64 => ClaimSecurityAttr::Uint64(
					std::slice::from_raw_parts(self.Values.pUint64, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::STRING => ClaimSecurityAttr::String(
					std::slice::from_raw_parts(self.Values.ppString, self.ValueCount as _)
						.iter()
						.map(|str_ptr| WString::from_wchars_nullt(*str_ptr).to_string())
						.collect(),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::FQBN => ClaimSecurityAttr::Fbqn(
					std::slice::from_raw_parts(self.Values.pFqbn, self.ValueCount as _),
				),
				co::CLAIM_SECURITY_ATTRIBUTE_TYPE::OCTET_STRING => ClaimSecurityAttr::OctetString(
					std::slice::from_raw_parts(self.Values.pOctetString, self.ValueCount as _),
				),
				_ => panic!("Invalid ValueType."),
			}
		}
	}
}

/// [`CONSOLE_READCONSOLE_CONTROL`](https://learn.microsoft.com/en-us/windows/console/console-readconsole-control)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct CONSOLE_READCONSOLE_CONTROL {
	pub nLength: u32,
	pub nInitialChars: u32,
	pub dwCtrlWakeupMask: u32,
	pub dwControlKeyState: u32,
}

/// [`DEV_BROADCAST_HDR`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_hdr)
/// struct.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DEV_BROADCAST_HDR {
	pub dbch_size: u32, // used by SvcCtlDeviceEvent (advapi) and wm::DeviceChange (user)
	pub dbch_devicetype: co::DBT_DEVTYP,
	dbch_reserved: u32,
}

/// [`DISK_SPACE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-disk_space_information)
/// struct.
#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct DISK_SPACE_INFORMATION {
	pub ActualTotalAllocationUnits: u64,
	pub ActualAvailableAllocationUnits: u64,
	pub ActualPoolUnavailableAllocationUnits: u64,
	pub CallerTotalAllocationUnits: u64,
	pub CallerAvailableAllocationUnits: u64,
	pub CallerPoolUnavailableAllocationUnits: u64,
	pub UsedAllocationUnits: u64,
	pub TotalReservedAllocationUnits: u64,
	pub VolumeStorageReserveAllocationUnits: u64,
	pub AvailableCommittedAllocationUnits: u64,
	pub PoolAvailableAllocationUnits: u64,
	pub SectorsPerAllocationUnit: u32,
	pub BytesPerSector: u32,
}

/// [`FILETIME`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
///
/// Can be converted to [`SYSTEMTIME`](crate::SYSTEMTIME) with
/// [`FileTimeToSystemTime`](crate::FileTimeToSystemTime) function.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

impl From<u64> for FILETIME {
	fn from(v: u64) -> Self {
		Self {
			dwLowDateTime: LODWORD(v),
			dwHighDateTime: HIDWORD(v),
		}
	}
}

impl From<FILETIME> for u64 {
	fn from(v: FILETIME) -> Self {
		MAKEQWORD(v.dwLowDateTime, v.dwHighDateTime)
	}
}

impl FILETIME {
	/// Returns a new `FILETIME` with the milliseconds difference.
	#[must_use]
	pub const fn add_ms(self, ms: i64) -> Self {
		let self64 = MAKEQWORD(self.dwLowDateTime, self.dwHighDateTime) as i64;
		let new_self64 = self64 + (ms * 10_000);
		Self {
			dwLowDateTime: LODWORD(new_self64 as _),
			dwHighDateTime: HIDWORD(new_self64 as _),
		}
	}

	/// Returns a new `FILETIME` with the seconds difference.
	#[must_use]
	pub const fn add_secs(self, secs: i64) -> Self {
		self.add_ms(secs * 1000)
	}

	/// Returns a new `FILETIME` with the minutes difference.
	#[must_use]
	pub const fn add_mins(self, mins: i64) -> Self {
		self.add_secs(mins * 60)
	}

	/// Returns a new `FILETIME` with the hours difference.
	#[must_use]
	pub const fn add_hours(self, hours: i64) -> Self {
		self.add_mins(hours * 60)
	}

	/// Returns a new `FILETIME` with the days difference.
	#[must_use]
	pub const fn add_days(self, days: i64) -> Self {
		self.add_hours(days * 24)
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

impl_default!(HEAPLIST32, dwSize);

newtype_num! { LANGID: u16;
	/// [`LANGID`](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifiers)
	/// language identifier.
}

impl std::fmt::Debug for LANGID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Primary [{:#04x} {}], Sublang [{:#04x} {}]",
			self.primary_lang_id(),
			self.primary_lang_id(),
			self.sub_lang_id(),
			self.sub_lang_id()
		)
	}
}
impl std::fmt::Display for LANGID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

impl LANGID {
	/// [`LANGID`](crate::LANGID) composed of
	/// [`LANG::NEUTRAL`](crate::co::LANG::NEUTRAL) and
	/// [`SUBLANG::NEUTRAL`](crate::co::SUBLANG::NEUTRAL).
	pub const NEUTRAL: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::NEUTRAL);

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
		Self((sublang.raw() << 10) | lang.raw())
	}

	/// Returns the primary language ID. Originally
	/// [`PRIMARYLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-primarylangid)
	/// macro.
	#[must_use]
	pub const fn primary_lang_id(self) -> co::LANG {
		unsafe { co::LANG::from_raw(self.0 & 0x3ff) }
	}

	/// Returns the sublanguage ID. Originally
	/// [`SUBLANGID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sublangid)
	/// macro.
	#[must_use]
	pub const fn sub_lang_id(self) -> co::SUBLANG {
		unsafe { co::SUBLANG::from_raw(self.0 >> 10) }
	}
}

newtype_num! { LCID: u32;
	/// [`LCID`](https://learn.microsoft.com/en-us/windows/win32/intl/locale-identifiers)
	/// locale identifier.
}

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
		Self(((sort_id.raw() as u32) << 16) | lang_id.raw() as u32)
	}

	/// Returns the language identifier. Originally
	/// [`LANGIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-langidfromlcid)
	/// macro.
	#[must_use]
	pub const fn lang_id(self) -> LANGID {
		unsafe { LANGID::from_raw(self.raw() as _) }
	}

	/// Returns the sort ID. Originally
	/// [`SORTIDFROMLCID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sortidfromlcid)
	/// macro.
	#[must_use]
	pub const fn sort_id(self) -> co::SORT {
		unsafe { co::SORT::from_raw(((self.raw() >> 16) & 0xf) as _) }
	}
}

/// [`LUID`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-luid)
/// identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LUID {
	LowPart: u32,
	HighPart: i32,
}

impl std::fmt::Display for LUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "LUID lo: {:#04x}, hi: {:#04x}", self.low_part(), self.high_part())
	}
}

impl LUID {
	pub const SYSTEM: Self = Self::from_parts(0x3e7, 0x0);
	pub const ANONYMOUS_LOGON: Self = Self::from_parts(0x3e6, 0x0);
	pub const LOCALSERVICE: Self = Self::from_parts(0x3e5, 0x0);
	pub const NETWORKSERVICE: Self = Self::from_parts(0x3e4, 0x0);
	pub const IUSER: Self = Self::from_parts(0x3e3, 0x0);
	pub const PROTECTED_TO_SYSTEM: Self = Self::from_parts(0x3e2, 0x0);

	/// Creates a new `LUID`.
	#[must_use]
	pub const fn from_parts(low_part: u32, high_part: i32) -> Self {
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

/// [`MEMORY_BASIC_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-memory_basic_information)
/// struct.
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
	pub BaseAddress: *mut std::ffi::c_void,
	pub AllocationBase: *mut std::ffi::c_void,
	pub AllocationProtect: co::PAGE,
	pub PartitionId: u16,
	pub RegionSize: usize,
	pub State: co::MEM_STATE,
	pub Protect: co::PAGE,
	pub Type: co::MEM_TYPE,
}

impl_default!(MEMORY_BASIC_INFORMATION);

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

impl_default!(MODULEENTRY32, dwSize);

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

impl_default!(MEMORYSTATUSEX, dwLength);

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

impl_default!(OSVERSIONINFOEX, dwOSVersionInfoSize);

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

/// [`POWERBROADCAST_SETTING`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-powerbroadcast_setting)
/// struct.
///
/// The `Data` field is dynamically allocated.
#[allow(dead_code)] // used by wm::PowerBroadcast in user
#[repr(C)]
pub struct POWERBROADCAST_SETTING {
	pub PowerSetting: co::POWER_SETTING,
	pub DataLength: u32,
	Data: [u8; 1],
}

impl POWERBROADCAST_SETTING {
	/// Returns the `Data` field according to `PowerSetting` identifier.
	///
	/// # Panics
	///
	/// Panics if `PowerSetting` identifier is invalid.
	///
	/// # Safety
	///
	/// Make sure the struct contains the correct size and data described by the
	/// `PowerSetting` identifier.
	#[must_use]
	pub const unsafe fn data(&self) -> PowerSetting {
		unsafe {
			match self.PowerSetting {
				co::POWER_SETTING::ACDC_POWER_SOURCE => {
					PowerSetting::AcDcPowerSource(co::SYSTEM_POWER_CONDITION::from_raw(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
					))
				},
				co::POWER_SETTING::BATTERY_PERCENTAGE_REMAINING => {
					PowerSetting::BatteryPercentageRemaining(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] as _,
					)
				},
				co::POWER_SETTING::CONSOLE_DISPLAY_STATE => {
					PowerSetting::ConsoleDisplayState(co::MONITOR_DISPLAY_STATE::from_raw(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
					))
				},
				co::POWER_SETTING::GLOBAL_USER_PRESENCE => {
					PowerSetting::GlobalUserPresence(co::USER_ACTIVITY_PRESENCE::from_raw(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
					))
				},
				co::POWER_SETTING::IDLE_BACKGROUND_TASK => PowerSetting::IdleBackgroundTask,
				co::POWER_SETTING::MONITOR_POWER_ON => PowerSetting::MonitorPowerOn(
					match std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] {
						0 => false,
						_ => true,
					},
				),
				co::POWER_SETTING::POWER_SAVING_STATUS => PowerSetting::PowerSavingStatus(
					match std::slice::from_raw_parts(self.Data.as_ptr() as *const u32, 1)[0] {
						0 => false,
						_ => true,
					},
				),
				co::POWER_SETTING::POWERSCHEME_PERSONALITY => PowerSetting::PowerSchemePersonality(
					std::slice::from_raw_parts(self.Data.as_ptr() as *const co::POWER_SAVINGS, 1)
						[0],
				),
				co::POWER_SETTING::SESSION_DISPLAY_STATUS => {
					PowerSetting::SessionDisplayStatus(co::MONITOR_DISPLAY_STATE::from_raw(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
					))
				},
				co::POWER_SETTING::SESSION_USER_PRESENCE => {
					PowerSetting::SessionUserPresence(co::USER_ACTIVITY_PRESENCE::from_raw(
						std::slice::from_raw_parts(self.Data.as_ptr() as *const _, 1)[0],
					))
				},
				co::POWER_SETTING::LIDSWITCH_STATE_CHANGE => PowerSetting::LidSwitchStateChange(
					match std::slice::from_raw_parts(self.Data.as_ptr() as *const u8, 1)[0] {
						0 => PowerSettingLid::Closed,
						_ => PowerSettingLid::Opened,
					},
				),
				co::POWER_SETTING::SYSTEM_AWAYMODE => PowerSetting::SystemAwayMode(
					match std::slice::from_raw_parts(self.Data.as_ptr() as *const u8, 1)[0] {
						0 => PowerSettingAwayMode::Exiting,
						_ => PowerSettingAwayMode::Entering,
					},
				),
				_ => panic!("Invalid co::POWER_SETTING."),
			}
		}
	}
}

/// [`PROCESS_HEAP_ENTRY`](https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-process_heap_entry)
/// struct.
#[repr(C)]
pub struct PROCESS_HEAP_ENTRY {
	pub lpData: *mut std::ffi::c_void,
	pub cbData: u32,
	pub cbOverhead: u8,
	pub iRegionIndex: u8,
	pub wFlags: co::PROCESS_HEAP,
	union0: PROCESS_HEAP_ENTRY_union0,
}

#[repr(C)]
union PROCESS_HEAP_ENTRY_union0 {
	Block: PROCESS_HEAP_ENTRY_Block,
	Region: PROCESS_HEAP_ENTRY_Region,
}

/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) `Block`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY_Block {
	pub hMem: *mut std::ffi::c_void,
	dwReserved: [u32; 3],
}

/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) `Region`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY_Region {
	pub dwCommittedSize: u32,
	pub dwUnCommittedSize: u32,
	pub lpFirstBlock: *mut std::ffi::c_void,
	pub lpLastBlock: *mut std::ffi::c_void,
}

impl_default!(PROCESS_HEAP_ENTRY);

impl PROCESS_HEAP_ENTRY {
	/// Retrieves the `Block` union field.
	#[must_use]
	pub const fn Block(&self) -> Option<&PROCESS_HEAP_ENTRY_Block> {
		if self.wFlags.has(co::PROCESS_HEAP::ENTRY_MOVEABLE) {
			Some(unsafe { &self.union0.Block })
		} else {
			None
		}
	}

	/// Retrieves the `Region` union field.
	#[must_use]
	pub const fn Region(&self) -> Option<&PROCESS_HEAP_ENTRY_Region> {
		if self.wFlags.has(co::PROCESS_HEAP::REGION) {
			Some(unsafe { &self.union0.Region })
		} else {
			None
		}
	}
}

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
	th32DefaultHeapID: usize,
	th32ModuleID: u32,
	pub cntThreads: u32,
	pub th32ParentProcessID: u32,
	pub pcPriClassBase: i32,
	dwFlags: u32,
	szExeFile: [u16; MAX_PATH],
}

impl_default!(PROCESSENTRY32, dwSize);

impl PROCESSENTRY32 {
	pub_fn_string_arr_get_set!(szExeFile, set_szExeFile);
}

/// [`PROCESSOR_NUMBER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-processor_number)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PROCESSOR_NUMBER {
	pub Group: u16,
	pub Number: u8,
	Reserved: u8,
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

impl_default!(SECURITY_ATTRIBUTES, nLength, 'a);

impl<'a> SECURITY_ATTRIBUTES<'a> {
	pub_fn_ptr_get_set!('a, lpSecurityDescriptor, set_lpSecurityDescriptor, SECURITY_DESCRIPTOR);
	pub_fn_bool_get_set!(bInheritHandle, set_bInheritHandle);
}

/// [`SECURITY_DESCRIPTOR`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)
/// struct.
///
/// This struct can be initialized with
/// [`InitializeSecurityDescriptor`](crate::InitializeSecurityDescriptor)
/// function.
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

impl_default!(STARTUPINFO, cb, 'a, 'b);

impl<'a, 'b> STARTUPINFO<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, lpDesktop, set_lpDesktop);
	pub_fn_string_ptr_get_set!('a, lpTitle, set_lpTitle);

	/// Returns the `wShowWindow` field.
	#[must_use]
	pub const fn wShowWindow(&self) -> co::SW {
		unsafe { co::SW::from_raw(self.wShowWindow as _) }
	}

	/// Sets the `wShowWindow` field.
	pub const fn set_wShowWindow(&mut self, val: co::SW) {
		self.wShowWindow = val.raw() as _;
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
///
/// Can be converted to [`FILETIME`](crate::FILETIME) with
/// [`SystemTimeToFileTime`](crate::SystemTimeToFileTime) function.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SYSTEMTIME {
	/// The year. The valid values for this member are 1,601 through 30,827.
	pub wYear: u16,
	/// The month. January = 1.
	pub wMonth: u16,
	/// The day of the week. Sunday = 0.
	pub wDayOfWeek: u16,
	/// The day of the month. The valid values for this member are 1 through 31.
	pub wDay: u16,
	/// The hour. The valid values for this member are 0 through 23.
	pub wHour: u16,
	/// The minute. The valid values for this member are 0 through 59.
	pub wMinute: u16,
	/// The second. The valid values for this member are 0 through 59.
	pub wSecond: u16,
	/// The millisecond. The valid values for this member are 0 through 999.
	pub wMilliseconds: u16,
}

impl std::fmt::Display for SYSTEMTIME {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
			self.wYear,
			self.wMonth,
			self.wDay,
			self.wHour,
			self.wMinute,
			self.wSecond,
			self.wMilliseconds
		)
	}
}

impl SYSTEMTIME {
	/// Returns a new `SYSTEMTIME` with the milliseconds difference.
	///
	/// Performs intermediate [`FILETIME`](crate::FILETIME) conversions.
	#[must_use]
	pub fn add_ms(self, ms: i64) -> SysResult<Self> {
		let ft = SystemTimeToFileTime(&self)?;
		FileTimeToSystemTime(&ft.add_ms(ms))
	}

	/// Returns a new `SYSTEMTIME` with the seconds difference.
	///
	/// Performs intermediate [`FILETIME`](crate::FILETIME) conversions.
	#[must_use]
	pub fn add_secs(self, secs: i64) -> SysResult<Self> {
		self.add_ms(secs * 1000)
	}

	/// Returns a new `SYSTEMTIME` with the minutes difference.
	///
	/// Performs intermediate [`FILETIME`](crate::FILETIME) conversions.
	#[must_use]
	pub fn add_mins(self, mins: i64) -> SysResult<Self> {
		self.add_secs(mins * 60)
	}

	/// Returns a new `SYSTEMTIME` with the hours difference.
	///
	/// Performs intermediate [`FILETIME`](crate::FILETIME) conversions.
	#[must_use]
	pub fn add_hours(self, hours: i64) -> SysResult<Self> {
		self.add_mins(hours * 60)
	}

	/// Returns a new `SYSTEMTIME` with the days difference.
	///
	/// Performs intermediate [`FILETIME`](crate::FILETIME) conversions.
	#[must_use]
	pub fn add_days(self, days: i64) -> SysResult<Self> {
		self.add_hours(days * 24)
	}
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

impl_default!(THREADENTRY32, dwSize);

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

/// [`WIN32_FILE_ATTRIBUTE_DATA`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-win32_file_attribute_data)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub ftLastWriteTime: FILETIME,
	nFileSizeHigh: u32,
	nFileSizeLow: u32,
}

impl WIN32_FILE_ATTRIBUTE_DATA {
	/// Returns the nFileSizeHigh and nFileSizeLow fields.
	#[must_use]
	pub const fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}

	/// Sets the nFileSizeHigh and nFileSizeLow fields.
	pub const fn set_nFileSize(&mut self, val: u64) {
		self.nFileSizeHigh = HIDWORD(val);
		self.nFileSizeLow = LODWORD(val);
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
	/// Returns the nFileSizeHigh and nFileSizeLow fields.
	#[must_use]
	pub const fn nFileSize(&self) -> u64 {
		MAKEQWORD(self.nFileSizeLow, self.nFileSizeHigh)
	}

	/// Sets the nFileSizeHigh and nFileSizeLow fields.
	pub const fn set_nFileSize(&mut self, val: u64) {
		self.nFileSizeHigh = HIDWORD(val);
		self.nFileSizeLow = LODWORD(val);
	}

	pub_fn_string_arr_get_set!(cFileName, set_cFileName);
	pub_fn_string_arr_get_set!(cAlternateFileName, set_cAlternateFileName);
}
