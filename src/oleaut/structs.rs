#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::oleaut::ffi;

/// A
/// [string data type](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr)
/// used with COM automation.
///
/// Automatically calls
/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
/// when the object goes out of scope.
#[repr(transparent)]
pub struct BSTR(*mut u16);

impl Drop for BSTR {
	fn drop(&mut self) {
		if !self.0.is_null() {
			unsafe {
				ffi::SysFreeString(self.0);
			}
		}
	}
}

impl Default for BSTR {
	fn default() -> Self {
		Self(std::ptr::null_mut())
	}
}

impl From<BSTR> for WString {
	fn from(v: BSTR) -> WString {
		unsafe { WString::from_wchars_nullt(v.as_ptr()) }
	}
}

impl std::fmt::Display for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let txt = unsafe { WString::from_wchars_nullt(self.as_ptr()) };
		std::fmt::Display::fmt(&txt, f)
	}
}
impl std::fmt::Debug for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "BSTR \"{}\"", self)
	}
}

impl BSTR {
	/// [`SysAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysallocstring)
	/// function.
	#[must_use]
	pub fn SysAllocString(s: &str) -> HrResult<Self> {
		let str_obj = WString::from_str(s);
		let ptr = unsafe { ffi::SysAllocString(str_obj.as_ptr()) };
		if ptr.is_null() { Err(co::HRESULT::E_OUTOFMEMORY) } else { Ok(Self(ptr)) }
	}

	/// [`SysReAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysreallocstring)
	/// function.
	///
	/// The underlying pointer is automatically updated.
	pub fn SysReAllocString(&mut self, s: &str) -> HrResult<()> {
		let str_obj = WString::from_str(s);
		let ptr = unsafe { ffi::SysReAllocString(self.0, str_obj.as_ptr()) };
		if ptr.is_null() {
			Err(co::HRESULT::E_OUTOFMEMORY)
		} else {
			self.0 = ptr;
			Ok(())
		}
	}

	/// [`SysStringLen`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysstringlen)
	/// function.
	#[must_use]
	pub fn SysStringLen(&self) -> u32 {
		unsafe { ffi::SysStringLen(self.0) }
	}

	/// Creates a new `BSTR` by wrapping a pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer has the correct type and isn't owned by anyone else,
	/// otherwise you may cause memory access violations.
	#[must_use]
	pub const unsafe fn from_ptr(p: *mut u16) -> Self {
		Self(p)
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the null-terminated wide string.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut u16 {
		self.0
	}

	/// Returns a pointer to the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer to the null-terminated wide string.
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut *mut u16 {
		&mut self.0
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// memory block as a null-terminated `u16` slice.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		unsafe { std::slice::from_raw_parts(self.0, self.SysStringLen() as usize + 1) }
	}

	/// Ejects the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer leaving a null pointer in its place, so that
	/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
	/// won't be called.
	///
	/// Be sure to free the pointer, otherwise, as the name of this method
	/// implies, you will cause a memory leak.
	#[must_use]
	pub const fn leak(&mut self) -> *mut u16 {
		std::mem::replace(&mut self.0, std::ptr::null_mut())
	}
}

/// [`DISPPARAMS`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/ns-oaidl-dispparams)
/// struct.
#[repr(C)]
pub struct DISPPARAMS<'a, 'b> {
	rvarg: *mut VARIANT, // in reverse order
	rgdispidNamedArgs: *mut co::DISPID,
	cArgs: u32,
	cNamedArgs: u32,

	_rvar: PhantomData<&'a mut VARIANT>,
	_rgdispidNamedArgs: PhantomData<&'b mut co::DISPID>,
}

impl_default!(DISPPARAMS, 'a, 'b);

impl<'a, 'b> DISPPARAMS<'a, 'b> {
	pub_fn_array_buf_get_set!('a, rvarg, set_rvarg, cArgs, VARIANT);
	pub_fn_array_buf_get_set!('b, rgdispidNamedArgs, set_rgdispidNamedArgs, cNamedArgs, co::DISPID);
}

/// [`EXCEPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/ns-oaidl-excepinfo)
/// struct.
///
/// This struct is returned in case of remote exception by
/// [`IDispatch::Invoke`](crate::prelude::oleaut_IDispatch::Invoke); in order to
/// provide full security, it implements the standard [`Drop`](std::ops::Drop)
/// trait to free the [`BSTR`](crate::BSTR) pointers.
#[repr(C)]
pub struct EXCEPINFO {
	pub wCode: u16,
	wReserved: u16,
	bstrSource: *mut u16,
	bstrDescription: *mut u16,
	bstrHelpFile: *mut u16,
	pub dwHelpContext: u32,
	pvReserved: *mut std::ffi::c_void,
	pfnDeferredFillIn: *mut std::ffi::c_void,
	pub scode: i32,
}

unsafe impl Send for EXCEPINFO {}
unsafe impl Sync for EXCEPINFO {}

impl Drop for EXCEPINFO {
	fn drop(&mut self) {
		if !self.bstrSource.is_null() {
			let _ = unsafe { BSTR::from_ptr(self.bstrSource) };
		}
		if !self.bstrDescription.is_null() {
			let _ = unsafe { BSTR::from_ptr(self.bstrDescription) };
		}
		if !self.bstrHelpFile.is_null() {
			let _ = unsafe { BSTR::from_ptr(self.bstrHelpFile) };
		}
	}
}

impl std::error::Error for EXCEPINFO {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

impl std::fmt::Display for EXCEPINFO {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} - {}",
			self.bstrSource().unwrap_or("(no source)".to_owned()),
			self.bstrDescription()
				.unwrap_or("(no description)".to_owned()),
		)
	}
}
impl std::fmt::Debug for EXCEPINFO {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self, f)
	}
}

impl_default!(EXCEPINFO);

impl EXCEPINFO {
	pub_fn_bstr_get!(bstrSource);
	pub_fn_bstr_get!(bstrDescription);
	pub_fn_bstr_get!(bstrHelpFile);
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
				ffi::PropVariantClear(self as *mut _ as _); // ignore errors
			}
		}
	}
}

impl_default!(PROPVARIANT); // PropVariantInit() is just a macro

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
			ffi::VariantInit(pvoid(&mut obj));
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
