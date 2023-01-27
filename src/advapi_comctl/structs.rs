#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::advapi::decl::HKEY;
use crate::kernel::decl::WString;

/// [`TBSAVEPARAMS`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbsaveparamsw)
/// struct.
#[repr(C)]
pub struct TBSAVEPARAMS<'a> {
	pub hkr: HKEY,
	pszSubKey: *mut u16,
	pszValueName: *mut u16,

	_pszSubKey: PhantomData<&'a mut u16>,
	_pszValueName: PhantomData<&'a mut u16>,
}

impl_default!(TBSAVEPARAMS, 'a);

impl<'a> TBSAVEPARAMS<'a> {
	pub_fn_string_ptr_get_set!('a, pszSubKey, set_pszSubKey);
	pub_fn_string_ptr_get_set!('a, pszValueName, set_pszValueName);
}
