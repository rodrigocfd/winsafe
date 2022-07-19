#![allow(non_snake_case)]

use std::mem::ManuallyDrop;

use crate::{co, oleaut};
use crate::kernel::decl::{SYSTEMTIME, WinResult};
use crate::ole::decl::ComPtr;
use crate::oleaut::decl::{
	BSTR, SystemTimeToVariantTime, VariantTimeToSystemTime,
};
use crate::prelude::{ole_IUnknown, oleaut_IDispatch};

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
		if self.vt != co::VT::EMPTY {
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

impl VARIANT {
	/// Returns the [`co::VT`](crate::co::VT) variant type currently being
	/// stored in the object.
	#[must_use]
	pub const fn variant_type(&self) -> co::VT {
		self.vt
	}

	/// Returns a reference to the raw data held by the `VARIANT`.
	#[must_use]
	pub const unsafe fn raw(&self) -> &[u8; 16] {
		&self.data
	}

	/// Tells whether the `VARIANT` holds no value.
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.vt == co::VT::EMPTY
	}

	/// Tells whether the `VARIANT` holds an SQL style null.
	#[must_use]
	pub fn is_null(&self) -> bool {
		self.vt == co::VT::NULL
	}

	/// Crates a new `VARIANT` holding a `bool` value.
	#[must_use]
	pub fn new_bool(val: bool) -> VARIANT {
		let val16: i16 = if val { -1 } else { 0 };
		Self::new_serialized(&val16.to_ne_bytes(), co::VT::BOOL)
	}

	/// If the `VARIANT` holds a `bool` value, returns it, otherwise `None`.
	#[must_use]
	pub fn bool(&self) -> Option<bool> {
		if self.vt == co::VT::BOOL {
			let val16 = i16::from_ne_bytes(self.data[..2].try_into().unwrap());
			Some(val16 != 0)
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding a [`BSTR`](crate::BSTR) value.
	#[must_use]
	pub fn new_bstr(val: &str) -> VARIANT {
		let mut bstr = BSTR::SysAllocString(val);
		let ptr = unsafe { bstr.leak() } as usize;
		Self::new_serialized(&ptr.to_ne_bytes(), co::VT::BSTR)
	}

	/// If the `VARIANT` holds a [`BSTR`](crate::BSTR) value, returns it,
	/// otherwise `None`.
	#[must_use]
	pub fn bstr(&self) -> Option<String> {
		if self.vt == co::VT::BSTR {
			let ptr = usize::from_ne_bytes(self.data[..8].try_into().unwrap());
			let bstr = ManuallyDrop::new(BSTR(ptr as _));
			Some(bstr.to_string())
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `f32` value.
	#[must_use]
	pub fn new_f32(val: f32) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::R4)
	}

	/// If the `VARIANT` holds an `f32` value, returns it, otherwise `None`.
	#[must_use]
	pub fn f32(&self) -> Option<f32> {
		if self.vt == co::VT::R4 {
			Some(f32::from_ne_bytes(self.data[..4].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `f64` value.
	#[must_use]
	pub fn new_f64(val: f64) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::R8)
	}

	/// If the `VARIANT` holds an `f64` value, returns it, otherwise `None`.
	#[must_use]
	pub fn f64(&self) -> Option<f64> {
		if self.vt == co::VT::R8 {
			Some(f64::from_ne_bytes(self.data[..8].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `i8` value.
	#[must_use]
	pub fn new_i8(val: i8) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::I1)
	}

	/// If the `VARIANT` holds an `i8` value, returns it, otherwise `None`.
	#[must_use]
	pub fn i8(&self) -> Option<i8> {
		if self.vt == co::VT::I1 {
			Some(i8::from_ne_bytes(self.data[..1].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `i16` value.
	#[must_use]
	pub fn new_i16(val: i16) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::I2)
	}

	/// If the `VARIANT` holds an `i16` value, returns it, otherwise `None`.
	#[must_use]
	pub fn i16(&self) -> Option<i16> {
		if self.vt == co::VT::I2 {
			Some(i16::from_ne_bytes(self.data[..2].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `i32` value.
	#[must_use]
	pub fn new_i32(val: i32) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::I4)
	}

	/// If the `VARIANT` holds an `i32` value, returns it, otherwise `None`.
	#[must_use]
	pub fn i32(&self) -> Option<i32> {
		if self.vt == co::VT::I4 {
			Some(i32::from_ne_bytes(self.data[..4].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `i64` value.
	#[must_use]
	pub fn new_i64(val: i64) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::I8)
	}

	/// If the `VARIANT` holds an `i64` value, returns it, otherwise `None`.
	#[must_use]
	pub fn i64(&self) -> Option<i64> {
		if self.vt == co::VT::I8 {
			Some(i64::from_ne_bytes(self.data[..8].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an [`IDispatch`](crate::IDispatch)
	/// COM value.
	///
	/// Note that the `IDispatch` object will be cloned into the `VARIANT`,
	/// still being able to be used thereafter.
	#[must_use]
	pub fn new_idispatch<T>(val: &T) -> VARIANT
		where T: oleaut_IDispatch,
	{
		let mut cloned = val.clone();
		let ptr: usize = unsafe { cloned.leak() }.into();
		Self::new_serialized(&ptr.to_ne_bytes(), co::VT::DISPATCH)
	}

	/// If the `VARIANT` holds an [`IDispatch`](crate::IDispatch) COM value,
	/// returns it, otherwise `None`.
	///
	/// Note that the returned object will be a clone of the `IDispatch` being
	/// held.
	#[must_use]
	pub fn idispatch<T>(&self) -> Option<T>
		where T: oleaut_IDispatch,
	{
		if self.vt == co::VT::DISPATCH {
			let ptr = usize::from_ne_bytes(self.data[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(T::from(ComPtr(ptr as *mut _)));
			let cloned = T::clone(&obj);
			Some(cloned)
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an [`IUnknown`](crate::IUnknown)
	/// COM value.
	///
	/// Note that the `IUnknown` object will be cloned into the `VARIANT`,
	/// still being able to be used thereafter.
	#[must_use]
	pub fn new_iunknown<T>(val: &T) -> VARIANT
		where T: ole_IUnknown,
	{
		let mut cloned = val.clone();
		let ptr: usize = unsafe { cloned.leak() }.into();
		Self::new_serialized(&ptr.to_ne_bytes(), co::VT::UNKNOWN)
	}

	/// If the `VARIANT` holds an [`IUnknown`](crate::IUnknown) COM value,
	/// returns it, otherwise `None`.
	///
	/// Note that the returned object will be a clone of the `IUnknown` being
	/// held.
	#[must_use]
	pub fn iunknown<T>(&self) -> Option<T>
		where T: ole_IUnknown,
	{
		if self.vt == co::VT::UNKNOWN {
			let ptr = usize::from_ne_bytes(self.data[..8].try_into().unwrap());
			let obj = ManuallyDrop::new(T::from(ComPtr(ptr as *mut _)));
			let cloned = T::clone(&obj);
			Some(cloned)
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding a date/time value.
	#[must_use]
	pub fn new_time(val: &SYSTEMTIME) -> WinResult<VARIANT> {
		let double = SystemTimeToVariantTime(val)?;
		Ok(Self::new_serialized(&double.to_ne_bytes(), co::VT::DATE))
	}

	/// If the `VARIANT` holds a date/time value, returns it, otherwise `None`.
	#[must_use]
	pub fn time(&self) -> Option<SYSTEMTIME> {
		if self.vt == co::VT::DATE {
			let double = f64::from_ne_bytes(self.data[..8].try_into().unwrap());
			let mut st = SYSTEMTIME::default();
			VariantTimeToSystemTime(double, &mut st).unwrap();
			Some(st)
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `u8` value.
	#[must_use]
	pub fn new_u8(val: u8) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::UI1)
	}

	/// If the `VARIANT` holds an `u8` value, returns it, otherwise `None`.
	#[must_use]
	pub fn u8(&self) -> Option<u8> {
		if self.vt == co::VT::UI1 {
			Some(u8::from_ne_bytes(self.data[..1].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `u16` value.
	#[must_use]
	pub fn new_u16(val: u16) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::UI2)
	}

	/// If the `VARIANT` holds an `u16` value, returns it, otherwise `None`.
	#[must_use]
	pub fn u16(&self) -> Option<u16> {
		if self.vt == co::VT::UI2 {
			Some(u16::from_ne_bytes(self.data[..2].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `u32` value.
	#[must_use]
	pub fn new_u32(val: u32) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::UI4)
	}

	/// If the `VARIANT` holds an `u32` value, returns it, otherwise `None`.
	#[must_use]
	pub fn u32(&self) -> Option<u32> {
		if self.vt == co::VT::UI4 {
			Some(u32::from_ne_bytes(self.data[..4].try_into().unwrap()))
		} else {
			None
		}
	}

	/// Creates a new `VARIANT` holding an `u64` value.
	#[must_use]
	pub fn new_u64(val: u64) -> VARIANT {
		Self::new_serialized(&val.to_ne_bytes(), co::VT::UI8)
	}

	/// If the `VARIANT` holds an `u64` value, returns it, otherwise `None`.
	#[must_use]
	pub fn u64(&self) -> Option<u64> {
		if self.vt == co::VT::UI8 {
			Some(u64::from_ne_bytes(self.data[..8].try_into().unwrap()))
		} else {
			None
		}
	}

	fn new_serialized(data: &[u8], vt: co::VT) -> Self {
		let mut obj = Self::default();
		obj.vt = vt;
		data.iter()
			.zip(&mut obj.data)
			.for_each(|(src, dest)| *dest = *src);
		obj
	}
}
