use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variable parameter for:
///
/// * [`HACCESSTOKEN::AdjustTokenPrivileges`](crate::prelude::advapi_Haccesstoken::AdjustTokenPrivileges).
pub enum DisabPriv<'a> {
	/// Disables all privileges.
	Disab,
	/// An array of privileges and its attributes.
	Privs(&'a TOKEN_PRIVILEGES)
}

/// Registry value types.
///
/// This is a high-level abstraction over the [`co::REG`](crate::co::REG)
/// constants, plus the value they carry.
#[derive(Clone, Debug)]
pub enum RegistryValue {
	/// Binary value, defined as [`REG::BINARY`](crate::co::REG::BINARY).
	Binary(Vec<u8>),
	/// An `u32` integer value, defined as [`REG::DWORD`](crate::co::REG::DWORD).
	Dword(u32),
	/// An `u64` integer value, defined as [`REG::QWORD`](crate::co::REG::QWORD).
	Qword(u64),
	/// String value, defined as [`REG::SZ`](crate::co::REG::SZ).
	Sz(String),
	/// String value that contains unexpanded references to environment
	/// variables, for example, `%PATH%`. To expand the environment variable
	/// references, use
	/// [`ExpandEnvironmentStrings`](crate::ExpandEnvironmentStrings).
	ExpandSz(String),
	/// Multiple strings, defined as [`REG::MULTI_SZ`](crate::co::REG::MULTI_SZ).
	MultiSz(Vec<String>),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl std::fmt::Display for RegistryValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Binary(b) => write!(
				f,
				"[REG_BINARY] {}",
				b.iter()
					.map(|n| format!("{:02x}", *n))
					.collect::<Vec<_>>()
					.join(" "),
			),
			Self::Dword(n) => write!(f, "[REG_DWORD] {}", *n),
			Self::Qword(n) => write!(f, "[REG_QWORD] {}", *n),
			Self::Sz(s) => write!(f, "[REG_SZ] \"{}\"", s),
			Self::ExpandSz(s) => write!(f, "[REG_EXPAND_SZ] \"{}\"", s),
			Self::MultiSz(v) => write!(
				f,
				"[REG_MULTI_SZ] {}",
				v.iter()
					.map(|s| format!("\"{}\"", s))
					.collect::<Vec<_>>()
					.join(", "),
			),
			Self::None => write!(f, "[REG_NONE]"),
		}
	}
}

impl RegistryValue {
	/// Parses a binary data block as a `RegistryValue`.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Whilst a few validations are made, assumes the binary data block has the
	/// correct content, according to the informed [`co::REG`](crate::co::REG).
	#[must_use]
	pub unsafe fn from_raw(
		buf: Vec<u8>,
		reg_type: co::REG,
	) -> SysResult<RegistryValue>
	{
		match reg_type {
			co::REG::BINARY => Ok(RegistryValue::Binary(buf)),
			co::REG::DWORD => {
				if buf.len() != std::mem::size_of::<u32>() { // validate size
					Err(co::ERROR::INVALID_DATA)
				} else {
					Ok(RegistryValue::Dword(
						u32::from_ne_bytes(
							*std::mem::transmute::<_, *const [u8; 4]>(buf.as_ptr()),
						)
					))
				}
			},
			co::REG::QWORD => {
				if buf.len() != std::mem::size_of::<u64>() { // validate size
					Err(co::ERROR::INVALID_DATA)
				} else {
					Ok(RegistryValue::Qword(
						u64::from_ne_bytes(
							*std::mem::transmute::<_, *const [u8; 8]>(buf.as_ptr()),
						)
					))
				}
			},
			co::REG::SZ => {
				let (_, vec16, _) = buf.align_to::<u16>();
				Ok(RegistryValue::Sz(WString::from_wchars_slice(&vec16).to_string()))
			},
			co::REG::EXPAND_SZ => {
				let (_, vec16, _) = buf.align_to::<u16>();
				Ok(RegistryValue::ExpandSz(WString::from_wchars_slice(&vec16).to_string()))
			},
			co::REG::MULTI_SZ => {
				let (_, vec16, _) = buf.align_to::<u16>();
				Ok(RegistryValue::MultiSz(
					parse_multi_z_str(vec16.as_ptr(), Some(vec16.len())),
				))
			},
			co::REG::NONE => Ok(RegistryValue::None),
			_ => Err(co::ERROR::CALL_NOT_IMPLEMENTED), // other types not implemented yet
		}
	}

	/// Returns a pointer to the raw data, along with the raw data length.
	#[must_use]
	pub fn as_ptr_with_len(&self,
		str_buf: &mut WString,
	) -> (*const std::ffi::c_void, u32)
	{
		match self {
			Self::Binary(b) => (b.as_ptr() as _, b.len() as _),
			Self::Dword(n) => (n as *const _ as _, std::mem::size_of::<u32>() as _),
			Self::Qword(n) => (n as *const _ as _, std::mem::size_of::<u64>() as _),
			Self::Sz(s) => {
				*str_buf = WString::from_str(s);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::ExpandSz(s) => {
				*str_buf = WString::from_str(s);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::MultiSz(v) => {
				*str_buf = WString::from_str_vec(v);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::None => (std::ptr::null(), 0),
		}
	}

	fn as_ptr_with_len_str(str_buf: &WString) -> (*const std::ffi::c_void, u32) {
		(
			str_buf.as_ptr() as _,
			(str_buf.buf_len() * std::mem::size_of::<u16>()) as _, // will include terminating null
		)
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	#[must_use]
	pub const fn reg_type(&self) -> co::REG {
		match self {
			Self::Binary(_) => co::REG::BINARY,
			Self::Dword(_) => co::REG::DWORD,
			Self::Qword(_) => co::REG::QWORD,
			Self::Sz(_) => co::REG::SZ,
			Self::ExpandSz(_) => co::REG::EXPAND_SZ,
			Self::MultiSz(_) => co::REG::MULTI_SZ,
			Self::None => co::REG::NONE,
		}
	}
}

/// Notification content for
/// [`HSERVICESTATUS::RegisterServiceCtrlHandlerEx`](crate::prelude::advapi_Hservicestatus::RegisterServiceCtrlHandlerEx)
/// callback, describing [`co::SERVICE_CONTROL`](crate::co::SERVICE_CONTROL).
pub enum SvcCtl<'a> {
	Continue,
	Interrogate,
	NetBindAdd,
	NetBindDisable,
	NetBindEnable,
	NetBindRemove,
	ParamChange,
	Pause,
	PreShutdown,
	Shutdown,
	Stop,

	DeviceEvent(co::DBT, SvcCtlDeviceEvent<'a>),
	HardwareProfileChange(co::DBT),
	PowerEvent(SvcCtlPowerEvent<'a>),
	SessionChange(co::WTS, &'a WTSSESSION_NOTIFICATION),
	TimeChange(&'a SERVICE_TIMECHANGE_INFO),
	TriggerEvent,
	UserModeReboot,

	UserDefined(u8, u32, usize),
}

impl<'a> SvcCtl<'a> {
	/// Constructs the enum according to the raw data.
	///
	/// # Safety
	///
	/// This enum is constructed when building the output of
	/// [`HSERVICESTATUS::RegisterServiceCtrlHandlerEx`](crate::prelude::advapi_Hservicestatus::RegisterServiceCtrlHandlerEx)
	/// callback, make sure all parameters are correct.
	#[must_use]
	pub unsafe fn from_raw(
		control: u32,
		event_type: u32,
		event_data: *mut std::ffi::c_void,
	) -> Self
	{
		match co::SERVICE_CONTROL::from_raw(control) {
			co::SERVICE_CONTROL::CONTINUE => Self::Continue,
			co::SERVICE_CONTROL::INTERROGATE => Self::Interrogate,
			co::SERVICE_CONTROL::NETBINDADD => Self::NetBindAdd,
			co::SERVICE_CONTROL::NETBINDDISABLE => Self::NetBindDisable,
			co::SERVICE_CONTROL::NETBINDENABLE => Self::NetBindEnable,
			co::SERVICE_CONTROL::NETBINDREMOVE => Self::NetBindRemove,
			co::SERVICE_CONTROL::PARAMCHANGE => Self::ParamChange,
			co::SERVICE_CONTROL::PAUSE => Self::Pause,
			co::SERVICE_CONTROL::PRESHUTDOWN => Self::PreShutdown,
			co::SERVICE_CONTROL::SHUTDOWN => Self::Shutdown,
			co::SERVICE_CONTROL::STOP => Self::Stop,

			co::SERVICE_CONTROL::DEVICEEVENT => Self::DeviceEvent(
				co::DBT::from_raw(event_type as _),
				SvcCtlDeviceEvent::from_raw(&*(event_data as *const _)),
			),
			co::SERVICE_CONTROL::HARDWAREPROFILECHANGE => Self::HardwareProfileChange(
				co::DBT::from_raw(event_type as _),
			),
			co::SERVICE_CONTROL::POWEREVENT => Self::PowerEvent(
				SvcCtlPowerEvent::from_raw(co::PBT::from_raw(event_type), event_data),
			),
			co::SERVICE_CONTROL::SESSIONCHANGE => Self::SessionChange(
				co::WTS::from_raw(event_type as _),
				&*(event_data as *const _),
			),
			co::SERVICE_CONTROL::TIMECHANGE => Self::TimeChange(
				&*(event_data as *const _),
			),
			co::SERVICE_CONTROL::TRIGGEREVENT => Self::TriggerEvent,
			co::SERVICE_CONTROL::USERMODEREBOOT => Self::UserModeReboot,

			_ => Self::UserDefined(control as _, event_type, event_data as _),
		}
	}
}

/// Notification content for [`SvcCtl`](crate::SvcCtl).
pub enum SvcCtlDeviceEvent<'a> {
	Interface(&'a DEV_BROADCAST_DEVICEINTERFACE),
	Handle(&'a DEV_BROADCAST_HANDLE),
	Oem(&'a DEV_BROADCAST_OEM),
	Port(&'a DEV_BROADCAST_PORT),
	Volume(&'a DEV_BROADCAST_VOLUME),
}

impl<'a> SvcCtlDeviceEvent<'a> {
	/// Constructs the enum according to the raw data.
	///
	/// # Panics
	///
	/// Panics if `dbch_devicetype` field is invalid.
	///
	/// # Safety
	///
	/// This enum is constructed when building the output of
	/// [`HSERVICESTATUS::RegisterServiceCtrlHandlerEx`](crate::prelude::advapi_Hservicestatus::RegisterServiceCtrlHandlerEx)
	/// callback, make sure all parameters are correct.
	#[must_use]
	pub unsafe fn from_raw(event_data: &DEV_BROADCAST_HDR) -> Self {
		let ptr = event_data as *const DEV_BROADCAST_HDR;
		match event_data.dbch_devicetype {
			co::DBT_DEVTYP::DEVICEINTERFACE => Self::Interface(&*(ptr as *const _)),
			co::DBT_DEVTYP::HANDLE => Self::Handle(&*(ptr as *const _)),
			co::DBT_DEVTYP::OEM => Self::Oem(&*(ptr as *const _)),
			co::DBT_DEVTYP::PORT => Self::Port(&*(ptr as *const _)),
			co::DBT_DEVTYP::VOLUME => Self::Volume(&*(ptr as *const _)),
			_ => panic!("Invalid co::DBT_DEVTYP."),
		}
	}
}

/// Notification content for [`SvcCtl`](crate::SvcCtl).
pub enum SvcCtlPowerEvent<'a> {
	StatusChange,
	ResumeAutomatic,
	ResumeSuspend,
	Suspend,
	PowerSettingChange(&'a POWERBROADCAST_SETTING),
}

impl<'a> SvcCtlPowerEvent<'a> {
	/// Constructs the enum according to the raw data.
	///
	/// # Panics
	///
	/// Panics if `event` is invalid.
	///
	/// # Safety
	///
	/// This enum is constructed when building the output of
	/// [`HSERVICESTATUS::RegisterServiceCtrlHandlerEx`](crate::prelude::advapi_Hservicestatus::RegisterServiceCtrlHandlerEx)
	/// callback, make sure all parameters are correct.
	#[must_use]
	pub unsafe fn from_raw(
		event: co::PBT,
		event_data: *mut std::ffi::c_void,
	) -> Self {
		match event {
			co::PBT::APMPOWERSTATUSCHANGE => Self::StatusChange,
			co::PBT::APMRESUMEAUTOMATIC => Self::ResumeAutomatic,
			co::PBT::APMRESUMESUSPEND => Self::ResumeSuspend,
			co::PBT::APMSUSPEND => Self::Suspend,
			co::PBT::POWERSETTINGCHANGE => Self::PowerSettingChange(&*(event_data as *const _)),
			_ => panic!("Invalid co::PBT."),
		}
	}
}

/// Variant parameter for:
///
/// * [`HACCESSTOKEN::GetTokenInformation`](crate::prelude::advapi_Haccesstoken::GetTokenInformation).
///
/// The enum values match those in
/// [`co::TOKEN_INFORMATION_CLASS`](crate::co::TOKEN_INFORMATION_CLASS) constant
/// type.
pub enum TokenInfo<'a, 'b, 'c, 'd, 'e, 'f> {
	User(Box<TOKEN_USER<'a>>),
	Groups(Box<TOKEN_GROUPS<'a>>),
	Privileges(Box<TOKEN_PRIVILEGES>),
	Owner(Box<TOKEN_OWNER<'a>>),
	PrimaryGroup(Box<TOKEN_PRIMARY_GROUP<'a>>),
	DefaultDacl(Box<TOKEN_DEFAULT_DACL<'a>>),
	Source(Box<TOKEN_SOURCE>),
	Type(Box<co::TOKEN_TYPE>),
	ImpersonationLevel(Box<co::SECURITY_IMPERSONATION>),
	Statistics(Box<TOKEN_STATISTICS>),
	RestrictedSids(Box<TOKEN_GROUPS<'a>>),
	SessionId(Box<u32>),
	GroupsAndPrivileges(Box<TOKEN_GROUPS_AND_PRIVILEGES<'a, 'b, 'c>>),
	SandBoxInert(Box<u32>),
	Origin(Box<TOKEN_ORIGIN>),
	ElevationType(Box<co::TOKEN_ELEVATION_TYPE>),
	LinkedToken(Box<TOKEN_LINKED_TOKEN>),
	Elevation(Box<TOKEN_ELEVATION>),
	HasRestrictions(Box<u32>),
	AccessInformation(Box<TOKEN_ACCESS_INFORMATION<'a, 'b, 'c, 'd, 'e, 'f>>),
	VirtualizationAllowed(Box<u32>),
	VirtualizationEnabled(Box<u32>),
	IntegrityLevel(Box<TOKEN_MANDATORY_LABEL<'a>>),
	UIAccess(Box<u32>),
	MandatoryPolicy(Box<TOKEN_MANDATORY_POLICY>),
	LogonSid(Box<TOKEN_GROUPS<'a>>),
	IsAppContainer(Box<u32>),
	Capabilities(Box<TOKEN_GROUPS<'a>>),
	AppContainerNumber(Box<u32>),
	DeviceClaimAttributes(Box<CLAIM_SECURITY_ATTRIBUTES_INFORMATION<'a, 'b>>),
	DeviceGroups(Box<TOKEN_GROUPS<'a>>),
	RestrictedDeviceGroups(Box<TOKEN_GROUPS<'a>>),
}
