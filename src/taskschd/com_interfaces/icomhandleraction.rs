#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PSTR};
use crate::ole::decl::HrResult;
use crate::prelude::{oleaut_IDispatch, taskschd_IAction};
use crate::vt::IActionVT;

/// [`IComHandlerAction`](crate::IComHandlerAction) virtual table.
#[repr(C)]
pub struct IComHandlerActionVT {
	pub IAction: IActionVT,
	pub get_ClassId: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_ClassId: fn(COMPTR, PCSTR) -> HRES,
	pub get_Data: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Data: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IComHandlerAction: "6d2fd252-75c5-4f66-90ba-2a7d8cc3039f";
	/// [`IComHandlerAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-icomhandleraction)
	/// COM interface over
	/// [`IComHandlerActionVT`](crate::vt::IComHandlerActionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IAction, IComHandlerAction};
	///
	/// let action: IAction; // initialized somewhere
	/// # let action = unsafe { IAction::null() };
	///
	/// let ch_action = action
	///     .QueryInterface::<IComHandlerAction>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for IComHandlerAction {}
impl taskschd_IAction for IComHandlerAction {}
impl taskschd_IComHandlerAction for IComHandlerAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IComHandlerAction`](crate::IComHandlerAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IComHandlerAction: taskschd_IAction {
	fn_com_bstr_get! { get_ClassId: IComHandlerActionVT;
		/// [`IComHandlerAction::get_ClassId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-icomhandleraction-get_classid)
		/// method.
	}

	fn_com_bstr_get! { get_Data: IComHandlerActionVT;
		/// [`IComHandlerAction::get_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-icomhandleraction-get_data)
		/// method.
	}

	fn_com_bstr_set! { put_ClassId: IComHandlerActionVT, class_id;
		/// [`IComHandlerAction::put_ClassId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-icomhandleraction-put_classid)
		/// method.
	}

	fn_com_bstr_set! { put_Data: IComHandlerActionVT, data;
		/// [`IComHandlerAction::put_Data`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-icomhandleraction-put_data)
		/// method.
	}
}
