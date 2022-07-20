#![allow(non_snake_case)]

use std::mem::ManuallyDrop;

use crate::{co, oleaut};
use crate::ole::decl::ComPtr;
use crate::prelude::{ole_IUnknown, oleaut_IDispatch, oleaut_Variant};

/// [`VARIANT`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/ns-oaidl-variant)
/// struct.
///
/// Automatically calls
/// [`VariantClear`](https://docs.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-variantclear)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[repr(C)]
pub struct VARIANT {
	vt: co::VT,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	data: [u8; 16],
}

impl Drop for VARIANT {
	fn drop(&mut self) {
		if self.vt() != co::VT::EMPTY {
			unsafe { oleaut::ffi::VariantClear(self as *mut _ as _); } // ignore errors
		}
	}
}

impl Default for VARIANT {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		unsafe { oleaut::ffi::VariantInit(&mut obj as *mut _ as _); }
		obj
	}
}

impl oleaut_Variant for VARIANT {
	unsafe fn raw(&self) -> &[u8; 16] {
		&self.data
	}

	unsafe fn from_raw(vt: co::VT, data: &[u8]) -> Self {
		let mut obj = Self::default();
		obj.vt = vt;
		data.iter()
			.zip(&mut obj.data)
			.for_each(|(src, dest)| *dest = *src);
		obj
	}

	fn vt(&self) -> co::VT {
		self.vt
	}
}

impl VARIANT {
	/// Creates a new object holding an [`IDispatch`](crate::IDispatch) COM
	/// value.
	///
	/// Note that the `IDispatch` object will be cloned into the object, still
	/// being able to be used thereafter.
	#[must_use]
	pub fn new_idispatch<T>(val: &T) -> VARIANT
		where T: oleaut_IDispatch,
	{
		let mut cloned = val.clone();
		let ptr: usize = unsafe { cloned.leak() }.into();
		unsafe { Self::from_raw(co::VT::DISPATCH, &ptr.to_ne_bytes()) }
	}

	/// If the object holds an [`IDispatch`](crate::IDispatch) COM value,
	/// returns it, otherwise `None`.
	///
	/// Note that the returned object will be a clone of the `IDispatch` being
	/// held.
	#[must_use]
	pub fn idispatch<T>(&self) -> Option<T>
		where T: oleaut_IDispatch,
	{
		if self.vt() == co::VT::DISPATCH {
			let ptr = usize::from_ne_bytes(unsafe { self.raw() }[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(T::from(ComPtr(ptr as *mut _)));
			let cloned = T::clone(&obj);
			Some(cloned)
		} else {
			None
		}
	}

	/// Creates a new object holding an [`IUnknown`](crate::IUnknown) COM value.
	///
	/// Note that the `IUnknown` object will be cloned into the object, still
	/// being able to be used thereafter.
	#[must_use]
	pub fn new_iunknown<T>(val: &T) -> VARIANT
		where T: ole_IUnknown,
	{
		let mut cloned = val.clone();
		let ptr: usize = unsafe { cloned.leak() }.into();
		unsafe { Self::from_raw(co::VT::UNKNOWN, &ptr.to_ne_bytes()) }
	}

	/// If the object holds an [`IUnknown`](crate::IUnknown) COM value, returns
	/// it, otherwise `None`.
	///
	/// Note that the returned object will be a clone of the `IUnknown` being
	/// held.
	#[must_use]
	pub fn iunknown<T>(&self) -> Option<T>
		where T: ole_IUnknown,
	{
		if self.vt() == co::VT::UNKNOWN {
			let ptr = usize::from_ne_bytes(unsafe { self.raw() }[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(T::from(ComPtr(ptr as *mut _)));
			let cloned = T::clone(&obj);
			Some(cloned)
		} else {
			None
		}
	}
}
