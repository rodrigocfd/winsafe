#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::WString;

/// [`COMDLG_FILTERSPEC`](https://docs.microsoft.com/en-us/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec)
/// struct.
#[repr(C)]
pub struct COMDLG_FILTERSPEC<'a, 'b> {
	pszName: *const u16,
	pszName_: PhantomData<&'a mut u16>,
	pszSpec: *const u16,
	pszSpec_: PhantomData<&'b mut u16>,
}

impl_default_zero!(COMDLG_FILTERSPEC, 'a, 'b);

impl<'a, 'b> COMDLG_FILTERSPEC<'a, 'b> {
	/// Returns the `pszName` field.
	pub fn pszName(&self) -> String {
		WString::from_wchars_nullt(self.pszName).to_string()
	}

	/// Sets the `pszName` field.
	pub fn set_pszName(&mut self, buf: &'a WString) {
		self.pszName = unsafe { buf.as_ptr() };
	}

	/// Returns the `pszSpec` field.
	pub fn pszSpec(&self) -> String {
		WString::from_wchars_nullt(self.pszSpec).to_string()
	}

	/// Sets the `pszSpec` field.
	pub fn set_pszSpec(&mut self, buf: &'b WString) {
		self.pszSpec = unsafe { buf.as_ptr() };
	}
}
