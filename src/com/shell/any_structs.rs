#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::WString;

/// [`COMDLG_FILTERSPEC`](https://docs.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec)
/// struct.
#[repr(C)]
pub struct COMDLG_FILTERSPEC<'a, 'b> {
	pszName: *mut u16,
	pszName_: PhantomData<&'a mut u16>,
	pszSpec: *mut u16,
	pszSpec_: PhantomData<&'b mut u16>,
}

impl_default!(COMDLG_FILTERSPEC, 'a, 'b);

impl<'a, 'b> COMDLG_FILTERSPEC<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszName, set_pszName);
	pub_fn_string_ptr_get_set!('b, pszSpec, set_pszSpec);
}
