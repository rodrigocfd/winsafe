#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::kernel::decl::WString;

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
		co::CF(self.cfFormat as _)
	}

	/// Sets the `cfFormat` field.
	pub fn set_cfFormat(&mut self, val: co::CF) {
		self.cfFormat = val.0 as _;
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
