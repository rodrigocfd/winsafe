#![allow(non_snake_case)]

use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::oleaut::{ffi, privs::*};

/// [`PROPERTYKEY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ns-wtypes-propertykey)
/// struct.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PROPERTYKEY {
	pub fmtid: GUID,
	pub pid: u32,
}

impl_default!(PROPERTYKEY);

impl PROPERTYKEY {
	/// Creates a new `PROPERTYKEY` by setting `pid` to `PID_FIRST_USABLE`
	/// (`0x02`).
	#[must_use]
	pub const fn new(fmtid: GUID) -> Self {
		Self { fmtid, pid: PID_FIRST_USABLE }
	}
}

/// [`PROPVARIANT`](https://learn.microsoft.com/en-us/windows/win32/api/propidlbase/ns-propidlbase-propvariant)
/// struct.
///
/// Should be manipulated through the [`PropVariant`](crate::PropVariant) enum.
#[repr(C)]
pub struct PROPVARIANT {
	pub(crate) vt: co::VT,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	pub(crate) data: PROPVARIANT_union,
}

#[repr(C)]
pub(crate) union PROPVARIANT_union {
	pub(crate) cVal: i8,
	pub(crate) bVal: u8,
	pub(crate) iVal: i16,
	pub(crate) uiVal: u16,
	pub(crate) lVal: i32,
	pub(crate) ulVal: u32,
	pub(crate) hVal: i64,
	pub(crate) uhVal: u64,
	pub(crate) fltVal: f32,
	pub(crate) dblVal: f64,
	pub(crate) ptr: *mut std::ffi::c_void, // for all pointer fields
	pub(crate) cac: ManuallyDrop<CAC>,
}

#[repr(C)]
pub(crate) struct CAC {
	cElems: u32,
	pElems: *mut i8,
}

impl Drop for PROPVARIANT {
	fn drop(&mut self) {
		if self.vt() != co::VT::EMPTY {
			unsafe {
				ffi::PropVariantClear(self as *mut _ as _);
			} // ignore errors
		}
	}
}

impl Default for PROPVARIANT {
	fn default() -> Self {
		unsafe { std::mem::zeroed::<Self>() } // PropVariantInit() is just a macro
	}
}

impl PROPVARIANT {
	/// Returns the [`co::VT`](crate::co::VT) variant type currently being held.
	#[must_use]
	pub const fn vt(&self) -> co::VT {
		self.vt
	}
}

/// [`VARIANT`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/ns-oaidl-variant)
/// struct.
///
/// Should be manipulated through the [`Variant`](crate::Variant) enum.
#[repr(C)]
pub struct VARIANT {
	pub(crate) vt: co::VT,
	wReserved1: u16,
	wReserved2: u16,
	wReserved3: u16,
	pub(crate) data: VARIANT_union,
}

#[repr(C)]
pub(crate) union VARIANT_union {
	pub(crate) llVal: i64,
	pub(crate) lVal: i32,
	pub(crate) bVal: u8,
	pub(crate) iVal: i16,
	pub(crate) fltVal: f32,
	pub(crate) dblVal: f64,
	pub(crate) cVal: i8,
	pub(crate) uiVal: u16,
	pub(crate) ulVal: u32,
	pub(crate) ullVal: u64,
	pub(crate) ptr: *mut std::ffi::c_void, // for all pointer fields
	pub(crate) brecord: ManuallyDrop<BRECORD>,
}

#[repr(C)]
pub(crate) struct BRECORD {
	pvRecord: *mut std::ffi::c_void,
	pRecInfo: COMPTR,
}

impl Drop for VARIANT {
	fn drop(&mut self) {
		if self.vt != co::VT::EMPTY {
			unsafe {
				ffi::VariantClear(self as *mut _ as _); // ignore errors
			}
		}
	}
}

impl Default for VARIANT {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		unsafe {
			ffi::VariantInit(&mut obj as *mut _ as _);
		}
		obj
	}
}

impl VARIANT {
	/// Returns the [`co::VT`](crate::co::VT) variant type currently being held.
	#[must_use]
	pub const fn vt(&self) -> co::VT {
		self.vt
	}
}
