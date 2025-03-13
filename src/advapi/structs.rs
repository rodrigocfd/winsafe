#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::advapi::privs::*;
use crate::co;
use crate::decl::*;
use crate::guard::*;

/// [`CLAIM_SECURITY_ATTRIBUTES_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-claim_security_attributes_information)
/// struct.
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION<'a, 'b> {
	pub Version: u16,
	Reserved: u16,
	AttributeCount: u32,
	pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b>,
}

impl_default!(CLAIM_SECURITY_ATTRIBUTES_INFORMATION, 'a, 'b);

impl<'a, 'b> CLAIM_SECURITY_ATTRIBUTES_INFORMATION<'a, 'b> {
	/// Returns the `pAttributeV1` field.
	#[must_use]
	pub fn pAttributeV1(&self) -> &[CLAIM_SECURITY_ATTRIBUTE_V1<'a, 'b>] {
		unsafe { std::slice::from_raw_parts(self.pAttributeV1, self.AttributeCount as _) }
	}
}

/// [`DEV_BROADCAST_DEVICEINTERFACE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_deviceinterface_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_DEVICEINTERFACE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcc_classguid: GUID,
	dbcc_name: [u16; 1],
}

impl DEV_BROADCAST_DEVICEINTERFACE {
	/// Returns the `dbcc_name` field.
	#[must_use]
	pub fn dbcc_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcc_name.as_ptr()) }.to_string()
	}
}

/// [`DEV_BROADCAST_HANDLE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_handle)
/// struct.
#[repr(C)]
pub struct DEV_BROADCAST_HANDLE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbch_handle: usize,
	pub dbch_hdevnotify: usize, // HDEVNOTIFY
	pub dbch_eventguid: GUID,
	pub dbch_nameoffset: i16,
	pub dbch_data: [u8; 1],
}

/// [`DEV_BROADCAST_OEM`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_oem)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_OEM {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbco_identifier: u32,
	pub dbco_suppfunc: u32,
}

/// [`DEV_BROADCAST_PORT`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_port_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_PORT {
	pub hdr: DEV_BROADCAST_HDR,
	dbcp_name: [u16; 1],
}

impl DEV_BROADCAST_PORT {
	/// Returns the `dbcp_name` field.
	#[must_use]
	pub fn dbcp_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcp_name.as_ptr()) }.to_string()
	}
}

/// [`DEV_BROADCAST_VOLUME`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_volume)
/// struct.
#[derive(Default)]
pub struct DEV_BROADCAST_VOLUME {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcv_unitmask: u32,
	pub dbcv_flags: co::DBTF,
}

/// [`LUID_AND_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-luid_and_attributes)
/// struct.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LUID_AND_ATTRIBUTES {
	pub Luid: LUID,
	pub Attributes: co::SE_PRIV_ATTR,
}

impl LUID_AND_ATTRIBUTES {
	/// Constructs a new `LUID_AND_ATTRIBUTES`.
	#[must_use]
	pub const fn new(luid: LUID, attrs: co::SE_PRIV_ATTR) -> Self {
		Self { Luid: luid, Attributes: attrs }
	}
}

/// [`SERVICE_STATUS`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct SERVICE_STATUS {
	pub dwServiceType: co::SERVICE_TYPE,
	pub dwCurrentState: co::SERVICE_STATE,
	pub dwControlsAccepted: co::SERVICE_ACCEPT,
	pub dwWin32ExitCode: u32,
	pub dwServiceSpecificExitCode: u32,
	pub dwCheckPoint: u32,
	pub dwWaitPoint: u32,
}

/// [`SERVICE_TIMECHANGE_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_timechange_info)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SERVICE_TIMECHANGE_INFO {
	liNewTime: i64,
	liOldTime: i64,
}

impl SERVICE_TIMECHANGE_INFO {
	/// Returns the `liNewTime` field.
	#[must_use]
	pub const fn liNewTime(&self) -> FILETIME {
		FILETIME {
			dwLowDateTime: LODWORD(self.liNewTime as _),
			dwHighDateTime: HIDWORD(self.liNewTime as _),
		}
	}

	/// Returns the `liOldTime` field.
	#[must_use]
	pub const fn liOldTime(&self) -> FILETIME {
		FILETIME {
			dwLowDateTime: LODWORD(self.liOldTime as _),
			dwHighDateTime: HIDWORD(self.liOldTime as _),
		}
	}

	/// Sets the `liNewTime` field.
	pub const fn set_liNewTime(&mut self, ft: FILETIME) {
		self.liNewTime = MAKEQWORD(ft.dwLowDateTime, ft.dwHighDateTime) as _;
	}

	/// Sets the `liOldTime` field.
	pub const fn set_liOldTime(&mut self, ft: FILETIME) {
		self.liOldTime = MAKEQWORD(ft.dwLowDateTime, ft.dwHighDateTime) as _;
	}
}

/// [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
/// struct.
///
/// Note that you cannot directly instantiate this struct, because the
/// `SubAuthority` field is dynamically allocated.
///
/// Possible ways:
///
/// * [`AllocateAndInitializeSid`](crate::AllocateAndInitializeSid) as [`FreeSidGuard`](crate::guard::FreeSidGuard);
/// * [`ConvertStringSidToSid`](crate::ConvertStringSidToSid) as [`LocalFreeSidGuard`](crate::guard::LocalFreeSidGuard);
/// * [`CopySid`](crate::CopySid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`CreateWellKnownSid`](crate::CreateWellKnownSid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`GetWindowsAccountDomainSid`](crate::GetWindowsAccountDomainSid) as [`SidGuard`](crate::guard::SidGuard);
/// * [`LookupAccountName`](crate::LookupAccountName) as [`SidGuard`](crate::guard::SidGuard).
#[repr(C)]
pub struct SID {
	pub Revision: u8,
	pub(in crate::advapi) SubAuthorityCount: u8,
	pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
	SubAuthority: [co::RID; 1],
}

impl std::fmt::Display for SID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match ConvertSidToStringSid(self) {
			Ok(name) => write!(f, "{}", name),
			Err(err) => write!(f, "{}", err),
		}
	}
}

impl PartialEq<SID> for SID {
	fn eq(&self, other: &SID) -> bool {
		EqualSid(self, other).unwrap() // assumes valid references
	}
}

impl Eq for SID {}

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
			std::slice::from_raw_parts(self.SubAuthority.as_ptr(), self.SubAuthorityCount as _)
		}
	}
}

/// [`SID_AND_ATTRIBUTES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct SID_AND_ATTRIBUTES<'a> {
	Sid: *mut SID,
	pub Attributes: u32,

	_Sid: PhantomData<&'a mut SID>,
}

impl_default!(SID_AND_ATTRIBUTES, 'a);

impl<'a> SID_AND_ATTRIBUTES<'a> {
	pub_fn_ptr_get_set!('a, Sid, set_Sid, SID);
}

/// [`SID_AND_ATTRIBUTES_HASH`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_and_attributes_hash)
/// struct.
#[repr(C)]
pub struct SID_AND_ATTRIBUTES_HASH<'a> {
	SidCount: u32,
	SidAttr: *mut SID_AND_ATTRIBUTES<'a>,
	pub Hash: [usize; SID_HASH_SIZE],
}

impl_default!(SID_AND_ATTRIBUTES_HASH, 'a);

impl<'a> SID_AND_ATTRIBUTES_HASH<'a> {
	pub_fn_array_buf_get_set!('a, SidAttr, set_SidAttr, SidCount, SID_AND_ATTRIBUTES);
}

/// [`SID_IDENTIFIER_AUTHORITY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_identifier_authority)
/// struct.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SID_IDENTIFIER_AUTHORITY {
	pub Value: [u8; 6],
}

impl std::fmt::Display for SID_IDENTIFIER_AUTHORITY {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.Value, f) // delegate to array Debug
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

/// [`TOKEN_ACCESS_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_access_information)
/// struct.
#[repr(C)]
pub struct TOKEN_ACCESS_INFORMATION<'a, 'b, 'c, 'd, 'e, 'f> {
	SidHash: *mut SID_AND_ATTRIBUTES_HASH<'a>,
	RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH<'b>,
	Privileges: *mut TOKEN_PRIVILEGES,
	pub AuthenticationId: LUID,
	pub TokenType: LUID,
	pub ImpersonationLevel: co::SECURITY_IMPERSONATION,
	pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
	Flags: u32,
	pub AppContainerNumber: u32,
	PackageSid: *mut SID,
	CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH<'e>,
	TrustLevelSid: *mut SID,
	SecurityAttributes: *mut std::ffi::c_void,

	_Privileges: PhantomData<&'c mut TOKEN_PRIVILEGES>,
	_PackageSid: PhantomData<&'d mut SID>,
	_TrustLevelSid: PhantomData<&'f mut SID>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> TOKEN_ACCESS_INFORMATION<'a, 'b, 'c, 'd, 'e, 'f> {
	pub_fn_ptr_get_set!('a, SidHash, set_SidHash, SID_AND_ATTRIBUTES_HASH<'a>);
	pub_fn_ptr_get_set!('b, RestrictedSidHash, set_RestrictedSidHash, SID_AND_ATTRIBUTES_HASH<'b>);
	pub_fn_ptr_get_set!('c, Privileges, set_Privileges, TOKEN_PRIVILEGES);
	pub_fn_ptr_get_set!('d, PackageSid, set_PackageSid, SID);
	pub_fn_ptr_get_set!('e, CapabilitiesHash, set_CapabilitiesHash, SID_AND_ATTRIBUTES_HASH<'e>);
	pub_fn_ptr_get_set!('f, TrustLevelSid, set_TrustLevelSid, SID);
}

impl_default!(TOKEN_ACCESS_INFORMATION, 'a, 'b, 'c, 'd, 'e, 'f);

/// [`TOKEN_APPCONTAINER_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_appcontainer_information)
/// struct.
#[repr(C)]
pub struct TOKEN_APPCONTAINER_INFORMATION<'a> {
	TokenAppContainer: *mut SID,

	_TokenAppContainer: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_APPCONTAINER_INFORMATION, 'a);

impl<'a> TOKEN_APPCONTAINER_INFORMATION<'a> {
	pub_fn_ptr_get_set!('a, TokenAppContainer, set_TokenAppContainer, SID);
}

/// [`TOKEN_DEFAULT_DACL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl)
/// struct.
#[repr(C)]
pub struct TOKEN_DEFAULT_DACL<'a> {
	DefaultDacl: *mut ACL,

	_DefaultDacl: PhantomData<&'a mut ACL>,
}

impl_default!(TOKEN_DEFAULT_DACL, 'a);

impl<'a> TOKEN_DEFAULT_DACL<'a> {
	pub_fn_ptr_get_set!('a, DefaultDacl, set_DefaultDacl, ACL);
}

/// [`TOKEN_ELEVATION`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_elevation)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TOKEN_ELEVATION {
	TokenIsElevated: u32,
}

impl TOKEN_ELEVATION {
	pub_fn_bool_get_set!(TokenIsElevated, set_TokenIsElevated);
}

/// [`TOKEN_GROUPS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups)
/// struct.
///
/// The `Group` field is dynamically allocated.
#[repr(C)]
pub struct TOKEN_GROUPS<'a> {
	pub(in crate::advapi) GroupCount: u32,
	Groups: [SID_AND_ATTRIBUTES<'a>; 1],
}

impl<'a> TOKEN_GROUPS<'a> {
	/// Returns a dynamically allocated
	/// [`TokenGroupsGuard`](crate::guard::TokenGroupsGuard).
	#[must_use]
	pub fn new(groups: &'a [SID_AND_ATTRIBUTES<'a>]) -> SysResult<TokenGroupsGuard<'a>> {
		TokenGroupsGuard::new(groups)
	}

	/// Returns a constant slice over the `Groups` entries.
	#[must_use]
	pub const fn Groups(&self) -> &[SID_AND_ATTRIBUTES<'a>] {
		unsafe { std::slice::from_raw_parts(self.Groups.as_ptr(), self.GroupCount as _) }
	}

	/// Returns a mutable slice over the `Groups` entries.
	#[must_use]
	pub fn Groups_mut(&mut self) -> &mut [SID_AND_ATTRIBUTES<'a>] {
		unsafe { std::slice::from_raw_parts_mut(self.Groups.as_mut_ptr(), self.GroupCount as _) }
	}
}

/// [`TOKEN_GROUPS_AND_PRIVILEGES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_groups_and_privileges)
/// struct.
#[repr(C)]
pub struct TOKEN_GROUPS_AND_PRIVILEGES<'a, 'b, 'c> {
	pub SidCount: u32,
	pub SidLength: u32,
	Sids: *mut SID_AND_ATTRIBUTES<'a>,
	pub RestrictedSidCount: u32,
	pub RestrictedSidLength: u32,
	RestrictedSids: *mut SID_AND_ATTRIBUTES<'b>,
	pub PrivilegeCount: u32,
	pub PrivilegeLength: u32,
	Privileges: *mut LUID_AND_ATTRIBUTES,
	pub AuthenticationId: LUID,

	_Privileges: PhantomData<&'c LUID_AND_ATTRIBUTES>,
}

impl_default!(TOKEN_GROUPS_AND_PRIVILEGES, 'a, 'b, 'c);

impl<'a, 'b, 'c> TOKEN_GROUPS_AND_PRIVILEGES<'a, 'b, 'c> {
	pub_fn_ptr_get_set!('a, Sids, set_Sids, SID_AND_ATTRIBUTES<'a>);
	pub_fn_ptr_get_set!('b, RestrictedSids, set_RestrictedSids, SID_AND_ATTRIBUTES<'b>);
	pub_fn_ptr_get_set!('c, Privileges, set_Privileges, LUID_AND_ATTRIBUTES);
}

/// [`TOKEN_LINKED_TOKEN`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_linked_token)
/// struct.
#[repr(C)]
pub struct TOKEN_LINKED_TOKEN {
	pub LinkedToken: HACCESSTOKEN,
}

impl_default!(TOKEN_LINKED_TOKEN);

/// [`TOKEN_MANDATORY_LABEL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_label)
/// struct.
#[repr(C)]
pub struct TOKEN_MANDATORY_LABEL<'a> {
	pub Label: SID_AND_ATTRIBUTES<'a>,
}

impl_default!(TOKEN_MANDATORY_LABEL, 'a);

/// [`TOKEN_MANDATORY_POLICY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_mandatory_policy)
/// struct.
#[repr(C)]
pub struct TOKEN_MANDATORY_POLICY {
	pub Policy: co::TOKEN_MANDATORY_POLICY,
}

impl_default!(TOKEN_MANDATORY_POLICY);

/// [`TOKEN_ORIGIN`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_origin)
/// struct.
#[repr(C)]
pub struct TOKEN_ORIGIN {
	pub OriginatingLogonSession: LUID,
}

impl_default!(TOKEN_ORIGIN);

/// [`TOKEN_OWNER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_owner)
/// struct.
#[repr(C)]
pub struct TOKEN_OWNER<'a> {
	Owner: *mut SID,

	_Owner: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_OWNER, 'a);

impl<'a> TOKEN_OWNER<'a> {
	pub_fn_ptr_get_set!('a, Owner, set_Owner, SID);
}

/// [`TOKEN_PRIMARY_GROUP`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_primary_group)
/// struct.
#[repr(C)]
pub struct TOKEN_PRIMARY_GROUP<'a> {
	PrimaryGroup: *mut SID,

	_Owner: PhantomData<&'a mut SID>,
}

impl_default!(TOKEN_PRIMARY_GROUP, 'a);

impl<'a> TOKEN_PRIMARY_GROUP<'a> {
	pub_fn_ptr_get_set!('a, PrimaryGroup, set_PrimaryGroup, SID);
}

/// [`TOKEN_PRIVILEGES`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_privileges)
/// struct.
///
/// The `Privileges` field is dynamically allocated.
#[repr(C)]
pub struct TOKEN_PRIVILEGES {
	pub(in crate::advapi) PrivilegeCount: u32,
	Privileges: [LUID_AND_ATTRIBUTES; 1],
}

impl TOKEN_PRIVILEGES {
	/// Returns a dynamically allocated
	/// [`TokenPrivilegesGuard`](crate::guard::TokenPrivilegesGuard).
	#[must_use]
	pub fn new(privileges: &[LUID_AND_ATTRIBUTES]) -> SysResult<TokenPrivilegesGuard> {
		TokenPrivilegesGuard::new(privileges)
	}

	/// Returns a constant slice over the `Privileges` entries.
	#[must_use]
	pub const fn Privileges(&self) -> &[LUID_AND_ATTRIBUTES] {
		unsafe { std::slice::from_raw_parts(self.Privileges.as_ptr(), self.PrivilegeCount as _) }
	}

	/// Returns a mutable slice over the `Privileges` entries.
	#[must_use]
	pub fn Privileges_mut(&mut self) -> &mut [LUID_AND_ATTRIBUTES] {
		unsafe {
			std::slice::from_raw_parts_mut(self.Privileges.as_mut_ptr(), self.PrivilegeCount as _)
		}
	}
}

/// [`TOKEN_SOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_source)
/// struct.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct TOKEN_SOURCE {
	pub SourceName: [i8; TOKEN_SOURCE_LENGTH],
	pub SourceIdentifier: LUID,
}

impl_default!(TOKEN_SOURCE);

/// [`TOKEN_STATISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_statistics)
/// struct.
#[repr(C)]
pub struct TOKEN_STATISTICS {
	pub TokenId: LUID,
	pub AuthenticationId: LUID,
	pub ExpirationTime: i64,
	pub TokenType: co::TOKEN_TYPE,
	pub ImpersonationLevel: co::SECURITY_IMPERSONATION,
	pub DynamicCharged: u32,
	pub DynamicAvailable: u32,
	pub GroupCount: u32,
	pub PrivilegeCount: u32,
	pub ModifiedId: LUID,
}

impl_default!(TOKEN_STATISTICS);

/// [`TOKEN_USER`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_user)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TOKEN_USER<'a> {
	pub User: SID_AND_ATTRIBUTES<'a>,
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
	#[must_use]
	pub unsafe fn buf_projection<'a>(&'a self, src: &'a [u8]) -> &'a [u8] {
		let proj_idx = self.ve_valueptr - src.as_ptr() as usize;
		let proj_past_idx = proj_idx + self.ve_valuelen as usize;
		&src[proj_idx..proj_past_idx]
	}
}

/// [`WTSSESSION_NOTIFICATION`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wtssession_notification)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct WTSSESSION_NOTIFICATION {
	pub cbSize: u32,
	pub dwSessionId: u32,
}
