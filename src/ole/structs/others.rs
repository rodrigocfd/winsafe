#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;

/// [`FORMATETC`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ns-objidl-formatetc)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
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
