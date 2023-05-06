#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::kernel::decl::WString;
use crate::ole::decl::ComPtr;
use crate::prelude::ole_IUnknown;

/// [`COAUTHIDENTITY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-coauthidentity)
/// struct.
#[repr(C)]
pub struct COAUTHIDENTITY<'a, 'b, 'c> {
	User: *mut u16,
	UserLength: u32,
	Domain: *mut u16,
	DomainLength: u32,
	Password: *mut u16,
	PasswordLength: u32,
	pub Flags: co::SEC_WINNT_AUTH_IDENTITY,

	_User: PhantomData<&'a mut u16>,
	_Domain: PhantomData<&'b mut u16>,
	_Password: PhantomData<&'c mut u16>,
}

impl_default!(COAUTHIDENTITY, 'a, 'b, 'c);

impl<'a, 'b, 'c> COAUTHIDENTITY<'a, 'b, 'c> {
	pub_fn_string_ptrlen_get_set!('a, User, set_User, UserLength);
	pub_fn_string_ptrlen_get_set!('b, Domain, set_Domain, DomainLength);
	pub_fn_string_ptrlen_get_set!('c, Password, set_Password, PasswordLength);
}

/// [`COAUTHINFO`](https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-coauthinfo)
/// struct.
#[repr(C)]
pub struct COAUTHINFO<'a, 'b, 'c, 'd, 'e> {
	pub dwAuthnSvc: co::RPC_C_AUTHN,
	pub dwAuthzSvc: co::RPC_C_AUTHZ,
	pwszServerPrincName: *mut u16,
	pub dwAuthnLevel: co::RPC_C_AUTHN,
	pub dwImpersonationLevel: co::RPC_C_IMP_LEVEL,
	pAuthIdentityData: *mut COAUTHIDENTITY<'c, 'd, 'e>,
	pub dwCapabilities: co::RPC_C_QOS_CAPABILITIES,

	_pwszServerPrincName: PhantomData<&'a mut u16>,
	_pAuthIdentityData: PhantomData<&'b mut COAUTHIDENTITY<'c, 'd, 'e>>,
}

impl_default!(COAUTHINFO, 'a, 'b, 'c, 'd, 'e);

impl<'a, 'b, 'c, 'd, 'e> COAUTHINFO<'a, 'b, 'c, 'd, 'e> {
	pub_fn_string_ptr_get_set!('a, pwszServerPrincName, set_pwszServerPrincName);
	pub_fn_ptr_get_set!('b, pAuthIdentityData, set_pAuthIdentityData, COAUTHIDENTITY<'c, 'd, 'e>);
}

/// [`COSERVERINFO`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-coserverinfo)
/// struct.
#[repr(C)]
pub struct COSERVERINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	dwReserved1: u32,
	pwszName: *mut u16,
	pAuthInfo: *mut COAUTHINFO<'c, 'd, 'e, 'f, 'g>,
	dwReserved2: u32,

	_pwszName: PhantomData<&'a mut u16>,
	_pAuthInfo: PhantomData<&'b COAUTHINFO<'c, 'd, 'e, 'f, 'g>>,
}

impl_default!(COSERVERINFO, 'a, 'b, 'c, 'd, 'e, 'f, 'g);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> COSERVERINFO<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub_fn_string_ptr_get_set!('a, pwszName, set_pwszName);
	pub_fn_ptr_get_set!('b, pAuthInfo, set_pAuthInfo, COAUTHINFO<'c, 'd, 'e, 'f, 'g>);
}

/// [`FORMATETC`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-formatetc)
/// struct.
#[repr(C)]
pub struct FORMATETC<'a> {
	cfFormat: u16,
	ptd: *mut DVTARGETDEVICE,
	pub dwAspect: u32,
	pub lindex: i32,
	pub tymed: co::TYMED,

	_ptd: PhantomData<&'a mut DVTARGETDEVICE>,
}

impl_default!(FORMATETC, 'a);

impl<'a> FORMATETC<'a> {
	/// Returns the `cfFormat` field.
	#[must_use]
	pub fn cfFormat(&self) -> co::CF {
		unsafe { co::CF::from_raw(self.cfFormat as _) }
	}

	/// Sets the `cfFormat` field.
	pub fn set_cfFormat(&mut self, val: co::CF) {
		self.cfFormat = val.raw() as _;
	}

	pub_fn_ptr_get_set!('a, ptd, set_ptd, DVTARGETDEVICE);
}

/// [`DVTARGETDEVICE`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-dvtargetdevice)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DVTARGETDEVICE {
	pub tdSize: u32,
	pub tdDriverNameOffset: u16,
	pub tdDeviceNameOffset: u16,
	pub tdPortNameOffset: u16,
	pub tdExtDevmodeOffset: u16,
	pub tdData: [u8; 1],
}

/// [`MULTI_QI`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-multi_qi)
/// struct.
///
/// Used to query multiple interfaces with
/// [`CoCreateInstanceEx`](crate::CoCreateInstanceEx).
#[repr(C)]
pub struct MULTI_QI<'a> {
	pIID: *mut co::IID,
	pItf: ComPtr,
	pub hr: co::HRESULT,

	_pIID: PhantomData<&'a mut co::IID>,
}

impl_default!(MULTI_QI, 'a);
impl_drop_comptr!(pItf, MULTI_QI, 'a);

impl<'a> MULTI_QI<'a> {
	pub_fn_ptr_get_set!('a, pIID, set_pIID, co::IID);
	pub_fn_comptr_get_set!(pItf, set_pItf, ole_IUnknown);
}
