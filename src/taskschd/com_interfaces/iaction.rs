#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::oleaut_IDispatch;
use crate::vt::IDispatchVT;

/// [`IAction`](crate::IAction) virtual table.
#[repr(C)]
pub struct IActionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Id: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Id: fn(ComPtr, PCSTR) -> HRES,
	pub get_Type: fn(ComPtr, *mut u32) -> HRES,
}

com_interface! { IAction: "bae54997-48b1-4cbe-9965-d6be263ebea4";
	/// [`IAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iaction)
	/// COM interface over [`IActionVT`](crate::vt::IActionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IAction {}
impl taskschd_IAction for IAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IAction`](crate::IAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IAction: oleaut_IDispatch {
	fn_bstr_get! { get_Id: IActionVT;
		/// [`IAction::get_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-get_id)
		/// method.
	}

	/// [`IAction::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::TASK_ACTION_TYPE> {
		let mut at = co::TASK_ACTION_TYPE::default();
		unsafe {
			let vt = self.vt_ref::<IActionVT>();
			ok_to_hrresult((vt.get_Type)(self.ptr(), &mut at.0))
		}.map(|_| at)
	}

	fn_bstr_set! { put_Id: IActionVT, id;
		/// [`IAction::put_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iaction-put_id)
		/// method.
	}
}
