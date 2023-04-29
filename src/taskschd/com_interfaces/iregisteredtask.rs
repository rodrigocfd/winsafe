#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::VARIANT;
use crate::prelude::oleaut_IDispatch;
use crate::taskschd::decl::ITaskDefinition;
use crate::vt::IDispatchVT;

/// [`IRegisteredTask`](crate::IRegisteredTask) virtual table.
#[repr(C)]
pub struct IRegisteredTaskVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Name: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_Path: fn(ComPtr, *mut PSTR) -> HRES,
	pub get_State: fn(ComPtr, *mut u32) -> HRES,
	pub get_Enabled: fn(ComPtr, *mut i16) -> HRES,
	pub put_Enabled: fn(ComPtr, i16) -> HRES,
	pub Run: fn(ComPtr, VARIANT, *mut ComPtr) -> HRES,
	pub RunEx: fn(ComPtr, VARIANT, i32, i32, PCSTR, *mut ComPtr) -> HRES,
	pub GetInstances: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub get_LastRunTime: fn(ComPtr, *mut f64) -> HRES,
	pub get_LastTaskResult: fn(ComPtr, *mut i32) -> HRES,
	pub get_NumberOfMissedRuns: fn(ComPtr, *mut i32) -> HRES,
	pub get_NextRunTime: fn(ComPtr, *mut f64) -> HRES,
	pub get_Definition: fn(ComPtr, *mut ComPtr) -> HRES,
	pub get_Xml: fn(ComPtr, *mut PSTR) -> HRES,
	pub GetSecurityDescriptor: fn(ComPtr, i32, *mut PSTR) -> HRES,
	pub SetSecurityDescriptor: fn(ComPtr, PCSTR, i32) -> HRES,
	pub Stop: fn(ComPtr, i32) -> HRES,
	pub GetRunTimes: fn(ComPtr, PCVOID, PCVOID, *mut u32, PVOID) -> HRES,
}

com_interface! { IRegisteredTask: "9c86f320-dee3-4dd1-b972-a303f26b061e";
	/// [`IRegisteredTask`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iregisteredtask)
	/// COM interface over [`IRegisteredTaskVT`](crate::vt::IRegisteredTaskVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IRegisteredTask: oleaut_IDispatch {
	fn_com_get! { get_Definition: IRegisteredTaskVT, ITaskDefinition;
		/// [`IRegisteredTask::get_Definition`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_definition)
		/// method.
	}

	/// [`IRegisteredTask::get_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_enabled)
	/// method.
	#[must_use]
	fn get_Enabled(&self) -> HrResult<bool> {
		let mut enabled = i16::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_Enabled)(self.ptr(), &mut enabled))
		}.map(|_| enabled != 0)
	}

	/// [`IRegisteredTask::get_LastRunTime`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_lastruntime)
	/// method.
	#[must_use]
	fn get_LastRunTime(&self) -> HrResult<f64> {
		let mut rt = f64::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_LastRunTime)(self.ptr(), &mut rt))
		}.map(|_| rt)
	}

	/// [`IRegisteredTask::get_LastTaskResult`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_lasttaskresult)
	/// method.
	#[must_use]
	fn get_LastTaskResult(&self) -> HrResult<i32> {
		let mut r = i32::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_LastTaskResult)(self.ptr(), &mut r))
		}.map(|_| r)
	}

	fn_bstr_get! { get_Name: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Name`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_name)
		/// method.
	}

	/// [`IRegisteredTask::get_NextRunTime`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_nextruntime)
	/// method.
	#[must_use]
	fn get_NextRunTime(&self) -> HrResult<f64> {
		let mut rt = f64::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_NextRunTime)(self.ptr(), &mut rt))
		}.map(|_| rt)
	}

	/// [`IRegisteredTask::get_NumberOfMissedRuns`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_numberofmissedruns)
	/// method.
	#[must_use]
	fn get_NumberOfMissedRuns(&self) -> HrResult<i32> {
		let mut mr = i32::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_NumberOfMissedRuns)(self.ptr(), &mut mr))
		}.map(|_| mr)
	}

	fn_bstr_get! { get_Path: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_path)
		/// method.
	}

	/// [`IRegisteredTask::get_State`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_state)
	/// method.
	#[must_use]
	fn get_State(&self) -> HrResult<co::TASK_STATE> {
		let mut state = co::TASK_STATE::default();
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.get_State)(self.ptr(), &mut state.0))
		}.map(|_| state)
	}

	fn_bstr_get! { get_Xml: IRegisteredTaskVT;
		/// [`IRegisteredTask::get_Xml`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-get_xml)
		/// method.
	}

	/// [`IRegisteredTask::put_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-put_enabled)
	/// method.
	fn put_Enabled(&self, enabled: bool) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.put_Enabled)(self.ptr(), enabled as _))
		}
	}

	/// [`IRegisteredTask::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iregisteredtask-stop)
	/// method.
	fn Stop(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IRegisteredTaskVT>();
			ok_to_hrresult((vt.Stop)(self.ptr(), 0))
		}
	}
}
