#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { ITrigger: "09941815-ea89-4b5b-89e0-2a773801fac3";
	/// [`ITrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itrigger)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITrigger {}
impl taskschd_ITrigger for ITrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITriggerCollection`](crate::ITriggerCollection).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITrigger: oleaut_IDispatch {
	/// [`ITrigger::get_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_enabled)
	/// method.
	#[must_use]
	fn get_Enabled(&self) -> HrResult<bool> {
		let mut enabled = i16::default();
		HrRet(unsafe { (vt::<ITriggerVT>(self).get_Enabled)(self.ptr(), &mut enabled) })
			.to_hrresult()
			.map(|_| enabled != 0)
	}

	fn_com_bstr_get! { get_EndBoundary: ITriggerVT;
		/// [`ITrigger::get_EndBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_endboundary)
		/// method.
	}

	fn_com_bstr_get! { get_ExecutionTimeLimit: ITriggerVT;
		/// [`ITrigger::get_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_executiontimelimit)
		/// method.
	}

	fn_com_bstr_get! { get_Id: ITriggerVT;
		/// [`ITrigger::get_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_id)
		/// method.
	}

	fn_com_bstr_get! { get_StartBoundary: ITriggerVT;
		/// [`ITrigger::get_StartBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_startboundary)
		/// method.
	}

	/// [`ITrigger::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::TASK_TRIGGER_TYPE2> {
		let mut ty = co::TASK_TRIGGER_TYPE2::default();
		HrRet(unsafe { (vt::<ITriggerVT>(self).get_Type)(self.ptr(), ty.as_mut()) })
			.to_hrresult()
			.map(|_| ty)
	}

	/// [`ITrigger::put_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_enabled)
	/// method.
	fn put_Enabled(&self, enabled: bool) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITriggerVT>(self).put_Enabled)(self.ptr(), enabled as _) })
			.to_hrresult()
	}

	fn_com_bstr_set! { put_EndBoundary: ITriggerVT, end;
		/// [`ITrigger::put_EndBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_endboundary)
		/// method.
	}

	fn_com_bstr_set! { put_ExecutionTimeLimit: ITriggerVT, time_limit;
		/// [`ITrigger::put_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_executiontimelimit)
		/// method.
	}

	fn_com_bstr_set! { put_Id: ITriggerVT, id;
		/// [`ITrigger::put_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_id)
		/// method.
	}

	fn_com_bstr_set! { put_StartBoundary: ITriggerVT, start;
		/// [`ITrigger::put_StartBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_startboundary)
		/// method.
	}
}
