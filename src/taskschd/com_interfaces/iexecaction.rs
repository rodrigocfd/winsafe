#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::prelude::{oleaut_IDispatch, taskschd_IAction};
use crate::vt::IActionVT;

/// [`IExecAction`](crate::IExecAction) virtual table.
#[repr(C)]
pub struct IExecActionVT {
	pub IAction: IActionVT,
	pub get_Path: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Path: fn(ComPtr, PCSTR) -> HRES,
	pub get_Arguments: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Arguments: fn(ComPtr, PCSTR) -> HRES,
	pub get_WorkingDirectory: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_WorkingDirectory: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { IExecAction: "4c3d624d-fd6b-49a3-b9b7-09cb3cd3f047";
	/// [`IExecAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iexecaction)
	/// COM interface over [`IExecActionVT`](crate::vt::IExecActionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IAction, IExecAction};
	///
	/// let action: IExecAction; // initialized somewhere
	/// # let action = IAction::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let exec_action = action
	///     .QueryInterface::<IExecAction>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for IExecAction {}
impl taskschd_IAction for IExecAction {}
impl taskschd_IExecAction for IExecAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IExecAction`](crate::IExecAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IExecAction: taskschd_IAction {
	fn_bstr_get! { get_Arguments: IExecActionVT;
		/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_arguments)
		/// method.
	}

	fn_bstr_get! { get_Path: IExecActionVT;
		/// [`IExecAction::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_path)
		/// method.
	}

	fn_bstr_get! { get_WorkingDirectory: IExecActionVT;
		/// [`IExecAction::get_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_workingdirectory)
		/// method.
	}

	fn_bstr_set! { put_Arguments: IExecActionVT, arguments;
		/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_arguments)
		/// method.
	}

	fn_bstr_set! { put_Path: IExecActionVT, path;
		/// [`IExecAction::put_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_path)
		/// method.
	}

	fn_bstr_set! { put_WorkingDirectory: IExecActionVT, working_directory;
		/// [`IExecAction::put_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_workingdirectory)
		/// method.
	}
}
