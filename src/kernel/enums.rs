use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variable parameter for:
///
/// * [`HINSTANCE::GetModuleHandleEx`](crate::prelude::kernel_Hinstance::GetModuleHandleEx).
pub enum AddrStr {
	/// No value, will pass `NULL` to the call.
	None,
	/// An address in the module.
	Addr(*mut std::ffi::c_void),
	/// Name of the loaded module.
	Str(WString),
}

impl AddrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}
}

/// Variable parameter for:
///
/// * [`CLAIM_SECURITY_ATTRIBUTE_V1`](crate::CLAIM_SECURITY_ATTRIBUTE_V1).
pub enum ClaimSecurityAttr<'a> {
	Int64(&'a [i64]),
	Uint64(&'a [u64]),
	String(Vec<String>),
	Fbqn(&'a [CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE<'a>]),
	OctetString(&'a [CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE<'a>]),
}

/// A resource identifier.
///
/// Variable parameter for:
///
/// * [`HINSTANCE::CreateDialogParam`](crate::prelude::user_Hinstance::CreateDialogParam);
/// * [`HINSTANCE::EnumResourceLanguages`](crate::prelude::kernel_Hinstance::EnumResourceLanguages);
/// * [`HINSTANCE::EnumResourceNames`](crate::prelude::kernel_Hinstance::EnumResourceNames);
/// * [`HINSTANCE::FindResource`](crate::prelude::kernel_Hinstance::FindResource);
/// * [`HINSTANCE::FindResourceEx`](crate::prelude::kernel_Hinstance::FindResourceEx);
/// * [`HINSTANCE::LoadAccelerators`](crate::prelude::user_Hinstance::LoadAccelerators);
/// * [`HINSTANCE::LoadMenu`](crate::prelude::user_Hinstance::LoadMenu);
/// * [`HUPDATERSRC::UpdateResource`](crate::prelude::kernel_Hupdatersrc::UpdateResource);
/// * [`BmpIdbRes`](crate::BmpIdbRes);
/// * [`IconRes`](crate::IconRes);
/// * [`ResStrs`](crate::ResStrs).
#[derive(Clone)]
pub enum IdStr {
	/// A resource ID.
	Id(u16),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for IdStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Id(rt) => write!(f, "ID: {}", rt),
			Self::Str(str) => write!(f, "Str: {}", str),
		}
	}
}

impl IdStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Constructs the enum from a raw pointer.
	///
	/// # Safety
	///
	/// If string, be sure it is null-terminated, otherwise an invalid memory
	/// location might be read.
	#[must_use]
	pub unsafe fn from_ptr(ptr: *const u16) -> IdStr {
		if IS_INTRESOURCE(ptr) {
			Self::Id(ptr as _)
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
/// 
/// * [`MEMORY_BASIC_INFORMATION`](crate::MEMORY_BASIC_INFORMATION).
#[repr(C)]
pub enum MemoryBasicInformationAllocationProtect {
	None = 0,

	Execute = 0x10,
	ExecuteRead = 0x20,
	ExecuteReadWrite = 0x40,
	ExecuteWriteCopy = 0x80,
	NoAccess = 0x01,
	ReadOnly = 0x02,
	ReadWrite = 0x04,
	WriteCopy = 0x08,
	TargetsInvalidOrNoUpdate = 0x40000000,

	Guard = 0x100,
	NoCache = 0x200,
	WriteCombine = 0x400,
}

/// Variant parameter for:
/// 
/// * [`MEMORY_BASIC_INFORMATION`](crate::MEMORY_BASIC_INFORMATION).
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum MemoryBasicInformationState {
	Commit = 0x1000,
	Free = 0x10000,
	Reverse = 0x2000
}

/// Variant parameter for:
/// 
/// * [`MEMORY_BASIC_INFORMATION`](crate::MEMORY_BASIC_INFORMATION).
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum MemoryBasicInformationType {
	Image = 0x1000000,
	Mapped = 0x40000,
	Private = 0x20000,
}

/// Variant parameter for:
///
/// * [`POWERBROADCAST_SETTING`](crate::POWERBROADCAST_SETTING).
pub enum PowerSetting {
	AcDcPowerSource(co::SYSTEM_POWER_CONDITION),
	BatteryPercentageRemaining(u8),
	ConsoleDisplayState(co::MONITOR_DISPLAY_STATE),
	GlobalUserPresence(co::USER_ACTIVITY_PRESENCE),
	IdleBackgroundTask,
	MonitorPowerOn(bool),
	PowerSavingStatus(bool),
	PowerSchemePersonality(co::POWER_SAVINGS),
	SessionDisplayStatus(co::MONITOR_DISPLAY_STATE),
	SessionUserPresence(co::USER_ACTIVITY_PRESENCE),
	LidSwitchStateChange(PowerSettingLid),
	SystemAwayMode(PowerSettingAwayMode),
}

/// Variant parameter for:
///
/// * [`PowerSetting::SystemAwayMode`](crate::PowerSetting::SystemAwayMode).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerSettingAwayMode {
	Exiting,
	Entering,
}

/// Variant parameter for:
///
/// * [`PowerSetting::LidSwitchStateChange`](crate::PowerSetting::LidSwitchStateChange).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerSettingLid {
	Closed,
	Opened,
}

/// A predefined resource identifier.
///
/// Variant parameter for:
///
/// * [`HINSTANCE::EnumResourceLanguages`](crate::prelude::kernel_Hinstance::EnumResourceLanguages);
/// * [`HINSTANCE::EnumResourceNames`](crate::prelude::kernel_Hinstance::EnumResourceNames);
/// * [`HINSTANCE::EnumResourceTypes`](crate::prelude::kernel_Hinstance::EnumResourceTypes);
/// * [`HINSTANCE::FindResource`](crate::prelude::kernel_Hinstance::FindResource);
/// * [`HINSTANCE::FindResourceEx`](crate::prelude::kernel_Hinstance::FindResourceEx);
/// * [`HUPDATERSRC`](crate::prelude::kernel_Hupdatersrc::UpdateResource).
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for RtStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Rt(rt) => write!(f, "RT: {}", rt),
			Self::Str(str) => write!(f, "Str: {}", str),
		}
	}
}

impl RtStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Constructs the enum from a pointer to raw data.
	///
	/// # Safety
	///
	/// If string, be sure it is null-terminated, otherwise an invalid memory
	/// location might be read.
	#[must_use]
	pub unsafe fn from_ptr(ptr: *const u16) -> RtStr {
		if IS_INTRESOURCE(ptr) {
			Self::Rt(unsafe { co::RT::from_raw(ptr as _) })
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Rt(id) => MAKEINTRESOURCE(id.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}
