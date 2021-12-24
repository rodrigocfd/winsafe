#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::comdlg::decl::CCHOOKPROC;
use crate::user::decl::{COLORREF, HWND};

/// [`CHOOSECOLOR`](https://docs.microsoft.com/en-us/windows/win32/api/commdlg/ns-commdlg-choosecolorw-r1)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comdlg")))]
#[repr(C)]
pub struct CHOOSECOLOR<'a> {
	lStructSize: u32,
	pub hwndOwner: HWND,
	pub hInstance: HWND,
	pub rgbResult: COLORREF,
	lpCustColors: *mut [COLORREF; 16],
	pub Flags: co::CC,
	pub lCustData: isize,
	pub lpfnHook: Option<CCHOOKPROC>,
	lpTemplateName: *mut u16, // u16 resource ID

	lpCustColors_: PhantomData<&'a mut COLORREF>,
}

impl_default_with_size!(CHOOSECOLOR, lStructSize, 'a);

impl<'a> CHOOSECOLOR<'a> {
	/// Returns the `lpCustColors` field.
	pub fn lpCustColors(&self) -> Option<&'a mut [COLORREF; 16]> {
		unsafe { self.lpCustColors.as_mut() }
	}

	/// Sets the `lpCustColors` field.
	pub fn set_lpCustColors(&mut self, val: Option<&'a mut [COLORREF; 16]>) {
		self.lpCustColors = val.map_or(std::ptr::null_mut(), |buf| buf);
	}

	pub_fn_resource_id_get_set!(lpTemplateName, set_lpTemplateName);
}
