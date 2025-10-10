use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variable parameter for:
///
/// * [`HINSTANCE::GetModuleHandleEx`](crate::HINSTANCE::GetModuleHandleEx)
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
/// * [`CLAIM_SECURITY_ATTRIBUTE_V1`](crate::CLAIM_SECURITY_ATTRIBUTE_V1)
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
/// * [`HINSTANCE::CreateDialogParam`](crate::HINSTANCE::CreateDialogParam)
/// * [`HINSTANCE::EnumResourceLanguages`](crate::HINSTANCE::EnumResourceLanguages)
/// * [`HINSTANCE::EnumResourceNames`](crate::HINSTANCE::EnumResourceNames)
/// * [`HINSTANCE::FindResource`](crate::HINSTANCE::FindResource)
/// * [`HINSTANCE::FindResourceEx`](crate::HINSTANCE::FindResourceEx)
/// * [`HINSTANCE::LoadAccelerators`](crate::HINSTANCE::LoadAccelerators)
/// * [`HINSTANCE::LoadMenu`](crate::HINSTANCE::LoadMenu)
/// * [`HUPDATERSRC::UpdateResource`](crate::HUPDATERSRC::UpdateResource)
/// * [`BmpIdbRes`](crate::BmpIdbRes)
/// * [`IconRes`](crate::IconRes)
/// * [`ResStrs`](crate::ResStrs)
#[derive(Clone)]
pub enum IdStr {
	/// A resource ID.
	Id(u16),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for IdStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use IdStr::*;
		match self {
			Id(rt) => write!(f, "ID: {}", rt),
			Str(str) => write!(f, "Str: {}", str),
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
			Self::Str(unsafe { WString::from_wchars_nullt(ptr) })
		}
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`AttachConsole`](crate::AttachConsole)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PidParent {
	/// Use the console of the specified process ID.
	Pid(u32),
	/// Use the console of the parent of the current process.
	Parent,
}

impl PidParent {
	/// Returns the `u32` value.
	#[must_use]
	pub const fn as_u32(&self) -> u32 {
		use PidParent::*;
		match self {
			Pid(pid) => *pid,
			Parent => ATTACH_PARENT_PROCESS,
		}
	}
}

/// Variant parameter for:
///
/// * [`POWERBROADCAST_SETTING`](crate::POWERBROADCAST_SETTING)
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
/// * [`PowerSetting::SystemAwayMode`](crate::PowerSetting::SystemAwayMode)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerSettingAwayMode {
	Exiting,
	Entering,
}

/// Variant parameter for:
///
/// * [`PowerSetting::LidSwitchStateChange`](crate::PowerSetting::LidSwitchStateChange)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PowerSettingLid {
	Closed,
	Opened,
}

/// A predefined resource identifier.
///
/// Variant parameter for:
///
/// * [`HINSTANCE::EnumResourceLanguages`](crate::HINSTANCE::EnumResourceLanguages)
/// * [`HINSTANCE::EnumResourceNames`](crate::HINSTANCE::EnumResourceNames)
/// * [`HINSTANCE::EnumResourceTypes`](crate::HINSTANCE::EnumResourceTypes)
/// * [`HINSTANCE::FindResource`](crate::HINSTANCE::FindResource)
/// * [`HINSTANCE::FindResourceEx`](crate::HINSTANCE::FindResourceEx)
/// * [`HUPDATERSRC`](crate::HUPDATERSRC::UpdateResource)
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for RtStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use RtStr::*;
		match self {
			Rt(rt) => write!(f, "RT: {}", rt),
			Str(str) => write!(f, "Str: {}", str),
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
			Self::Str(unsafe { WString::from_wchars_nullt(ptr) })
		}
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use RtStr::*;
		match self {
			Rt(id) => MAKEINTRESOURCE(id.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}
