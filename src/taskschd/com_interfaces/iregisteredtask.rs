#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IRegisteredTask: "9c86f320-dee3-4dd1-b972-a303f26b061e";
	/// [`IRegisteredTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iregisteredtask)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IRegisteredTask {}
impl taskschd_IRegisteredTask for IRegisteredTask {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IRegisteredTask`](crate::IRegisteredTask).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IRegisteredTask: oleaut_IDispatch {
	fn_com_interface_get! { get_Definition: IRegisteredTaskVT => ITaskDefinition;
		/// [`IRegisteredTask::get_Definition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_definition)
		/// method.
	}

	/// [`IRegisteredTask::get_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_enabled)
	/// method.
	#[must_use]
	fn get_Enabled(&self) -> HrResult<bool> {
		let mut enabled = i16::default();
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_Enabled)(self.ptr(), &mut enabled)
		})
		.map(|_| enabled != 0)
	}

	/// [`IRegisteredTask::get_LastRunTime`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_lastruntime)
	/// method.
	#[must_use]
	fn get_LastRunTime(&self) -> HrResult<f64> {
		let mut rt = f64::default();
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_LastRunTime)(self.ptr(), &mut rt)
		})
		.map(|_| rt)
	}

	/// [`IRegisteredTask::get_LastTaskResult`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_lasttaskresult)
	/// method.
	#[must_use]
	fn get_LastTaskResult(&self) -> HrResult<i32> {
		let mut r = 0i32;
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_LastTaskResult)(self.ptr(), &mut r)
		})
		.map(|_| r)
	}

	fn_com_bstr_get! { get_Name: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Name`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_name)
		/// method.
	}

	/// [`IRegisteredTask::get_NextRunTime`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_nextruntime)
	/// method.
	#[must_use]
	fn get_NextRunTime(&self) -> HrResult<f64> {
		let mut rt = f64::default();
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_NextRunTime)(self.ptr(), &mut rt)
		})
		.map(|_| rt)
	}

	/// [`IRegisteredTask::get_NumberOfMissedRuns`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_numberofmissedruns)
	/// method.
	#[must_use]
	fn get_NumberOfMissedRuns(&self) -> HrResult<i32> {
		let mut mr = 0i32;
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_NumberOfMissedRuns)(self.ptr(), &mut mr)
		})
		.map(|_| mr)
	}

	fn_com_bstr_get! { get_Path: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_path)
		/// method.
	}

	/// [`IRegisteredTask::get_State`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_state)
	/// method.
	#[must_use]
	fn get_State(&self) -> HrResult<co::TASK_STATE> {
		let mut state = co::TASK_STATE::default();
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).get_State)(self.ptr(), state.as_mut())
		})
		.map(|_| state)
	}

	fn_com_bstr_get! { get_Xml: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Xml`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_xml)
		/// method.
	}

	/// [`IRegisteredTask::put_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-put_enabled)
	/// method.
	fn put_Enabled(&self, enabled: bool) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IRegisteredTaskVT>(self).put_Enabled)(self.ptr(), enabled as _)
		})
	}

	/// [`IRegisteredTask::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-stop)
	/// method.
	fn Stop(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IRegisteredTaskVT>(self).Stop)(self.ptr(), 0) })
	}
}
