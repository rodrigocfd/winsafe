#![allow(non_camel_case_types, non_snake_case)]

use crate::co::TASK_ACTION_TYPE;
use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PSTR};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::oleaut::decl::VARIANT;
use crate::prelude::{ole_IUnknown, oleaut_IDispatch, oleaut_Variant};
use crate::taskschd::decl::IAction;
use crate::vt::IDispatchVT;

/// [`IActionCollection`](crate::IActionCollection) virtual table.
#[repr(C)]
pub struct IActionCollectionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Count: fn(COMPTR, *mut i32) -> HRES,
	pub get_Item: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub get__NewEnum: fn(COMPTR, *mut COMPTR) -> HRES,
	pub get_XmlText: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_XmlText: fn(COMPTR, PCSTR) -> HRES,
	pub Create: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub Remove: fn(COMPTR, VARIANT) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
	pub get_Context: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Context: fn(COMPTR, PCSTR) -> HRES,
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
		ok_to_hrresult(
			unsafe { (vt::<IActionCollectionVT>(self).Clear)(self.ptr()) },
		)
	}

	fn_bstr_get! { get_Context: IActionCollectionVT;
		/// [`IActionCollection::get_Context`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-get_context)
		/// method.
	}

	fn_bstr_get! { get_XmlText: IActionCollectionVT;
		/// [`IActionCollection::get_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-get_xmltext)
		/// method.
	}

	fn_bstr_set! { put_Context: IActionCollectionVT, context;
		/// [`IActionCollection::put_Context`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-put_context)
		/// method.
	}

	fn_bstr_set! { put_XmlText: IActionCollectionVT, text;
		/// [`IActionCollection::put_XmlText`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-put_xmltext)
		/// method.
	}

	/// [`IActionCollection::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-clear)
	/// method.
	fn Create(&self, action_type: TASK_ACTION_TYPE) -> HrResult<IAction> {
		let mut queried = unsafe { IAction::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IActionCollectionVT>(self).Create)(
					self.ptr(),
					action_type.raw(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IActionCollection::Remove`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iactioncollection-remove)
	/// method.
	fn Remove(&self, index: i32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IActionCollectionVT>(self).Remove)(
					self.ptr(),
					VARIANT::new_i32(index),
				)
			},
		)
	}
}
