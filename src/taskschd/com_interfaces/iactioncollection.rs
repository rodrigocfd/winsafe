#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::{BSTR, VARIANT};
use crate::prelude::{oleaut_IDispatch, oleaut_Variant};
use crate::vt::IDispatchVT;

/// [`IActionCollection`](crate::IActionCollection) virtual table.
#[repr(C)]
pub struct IActionCollectionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Count: fn(ComPtr, *mut i32) -> HRES,
	pub get_Item: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub get__NewEnum: fn(ComPtr, *mut ComPtr) -> HRES,
	pub get_XmlText: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_XmlText: fn(ComPtr, PCSTR) -> HRES,
	pub Create: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub Remove: fn(ComPtr, VARIANT) -> HRES,
	pub Clear: fn(ComPtr) -> HRES,
	pub get_Context: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Context: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { IActionCollection: "02820e19-7b98-4ed2-b2e8-fdccceff619b";
	/// [`IActionCollection`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iactioncollection)
	/// COM interface over [`IActionCollectionVT`](crate::vt::IActionCollectionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IActionCollection {}
impl taskschd_IActionCollection for IActionCollection {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IActionCollection`](crate::IActionCollection).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IActionCollection: oleaut_IDispatch {
	/// [`IActionCollection::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-clear)
	/// method.
	fn Clear(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult((vt.Clear)(self.ptr()))
		}
	}

	/// [`IActionCollection::get_Context`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-get_context)
	/// method.
	#[must_use]
	fn get_Context(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult((vt.get_Context)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`IActionCollection::get_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-get_xmltext)
	/// method.
	#[must_use]
	fn get_XmlText(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult((vt.get_XmlText)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`IActionCollection::put_Context`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-put_context)
	/// method.
	fn put_Context(&self, context: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult(
				(vt.put_Context)(
					self.ptr(),
					BSTR::SysAllocString(context)?.as_ptr(),
				),
			)
		}
	}

	/// [`IActionCollection::put_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-put_xmltext)
	/// method.
	fn put_XmlText(&self, text: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult(
				(vt.put_XmlText)(self.ptr(), BSTR::SysAllocString(text)?.as_ptr()),
			)
		}
	}

	/// [`IActionCollection::Remove`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-remove)
	/// method.
	fn Remove(&self, index: i32) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IActionCollectionVT>();
			ok_to_hrresult((vt.Remove)(self.ptr(), VARIANT::new_i32(index)))
		}
	}
}
