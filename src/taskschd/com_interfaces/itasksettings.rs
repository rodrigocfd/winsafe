#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { ITaskSettings: "8fd4711d-2d02-4c8c-87e3-eff699de127e";
	/// [`ITaskSettings`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itasksettings)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let task: w::ITaskDefinition; // initialized somewhere
	/// # let task = unsafe { w::ITaskDefinition::null() };
	///
	/// let settings = task
	///     .QueryInterface::<w::ITaskSettings>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for ITaskSettings {}
impl taskschd_ITaskSettings for ITaskSettings {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITaskSettings`](crate::ITaskSettings).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITaskSettings: oleaut_IDispatch {
	/// [`ITaskSettings::get_AllowDemandStart`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_allowdemandstart)
	/// method.
	#[must_use]
	fn get_AllowDemandStart(&self) -> HrResult<bool> {
		let mut allow = i16::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_AllowDemandStart)(self.ptr(), &mut allow) })
			.to_hrresult()
			.map(|_| allow != 0)
	}

	/// [`ITaskSettings::get_AllowHardTerminate`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_allowhardterminate)
	/// method.
	#[must_use]
	fn get_AllowHardTerminate(&self) -> HrResult<bool> {
		let mut allow = i16::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_AllowHardTerminate)(self.ptr(), &mut allow)
		})
		.to_hrresult()
		.map(|_| allow != 0)
	}

	/// [`ITaskSettings::get_Compatibility`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_compatibility)
	/// method.
	#[must_use]
	fn get_Compatibility(&self) -> HrResult<co::TASK_COMPATIBILITY> {
		let mut compat = co::TASK_COMPATIBILITY::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_Compatibility)(self.ptr(), compat.as_mut())
		})
		.to_hrresult()
		.map(|_| compat)
	}

	fn_com_bstr_get! { get_DeleteExpiredTaskAfter: ITaskSettingsVT;
		/// [`ITaskSettings::get_DeleteExpiredTaskAfter`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_deleteexpiredtaskafter)
		/// method.
	}

	/// [`ITaskSettings::get_DisallowStartIfOnBatteries`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_disallowstartifonbatteries)
	/// method.
	#[must_use]
	fn get_DisallowStartIfOnBatteries(&self) -> HrResult<bool> {
		let mut disallow = i16::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_DisallowStartIfOnBatteries)(self.ptr(), &mut disallow)
		})
		.to_hrresult()
		.map(|_| disallow != 0)
	}

	fn_com_bstr_get! { get_ExecutionTimeLimit: ITaskSettingsVT;
		/// [`ITaskSettings::get_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_executiontimelimit)
		/// method.
	}

	/// [`ITaskSettings::get_Hidden`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_hidden)
	/// method.
	#[must_use]
	fn get_Hidden(&self) -> HrResult<bool> {
		let mut hidden = i16::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_Hidden)(self.ptr(), &mut hidden) })
			.to_hrresult()
			.map(|_| hidden != 0)
	}

	/// [`ITaskSettings::get_MultipleInstances`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_multipleinstances)
	/// method.
	#[must_use]
	fn get_MultipleInstances(&self) -> HrResult<co::TASK_INSTANCES_POLICY> {
		let mut policy = co::TASK_INSTANCES_POLICY::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_MultipleInstances)(self.ptr(), policy.as_mut())
		})
		.to_hrresult()
		.map(|_| policy)
	}

	/// [`ITaskSettings::get_Priority`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_priority)
	/// method.
	#[must_use]
	fn get_Priority(&self) -> HrResult<i32> {
		let mut priority = i32::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_Priority)(self.ptr(), &mut priority) })
			.to_hrresult()
			.map(|_| priority)
	}

	fn_com_bstr_get! { get_RestartInterval: ITaskSettingsVT;
		/// [`ITaskSettings::get_RestartInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_restartinterval)
		/// method.
	}

	/// [`ITaskSettings::get_RestartCount`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_restartcount)
	/// method.
	#[must_use]
	fn get_RestartCount(&self) -> HrResult<i32> {
		let mut count = i32::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_RestartCount)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`ITaskSettings::get_RunOnlyIfIdle`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_runonlyifidle)
	/// method.
	#[must_use]
	fn get_RunOnlyIfIdle(&self) -> HrResult<bool> {
		let mut run = i16::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_RunOnlyIfIdle)(self.ptr(), &mut run) })
			.to_hrresult()
			.map(|_| run != 0)
	}

	/// [`ITaskSettings::get_RunOnlyIfNetworkAvailable`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_runonlyifnetworkavailable)
	/// method.
	#[must_use]
	fn get_RunOnlyIfNetworkAvailable(&self) -> HrResult<bool> {
		let mut run = i16::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_RunOnlyIfNetworkAvailable)(self.ptr(), &mut run)
		})
		.to_hrresult()
		.map(|_| run != 0)
	}

	/// [`ITaskSettings::get_StartWhenAvailable`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_startwhenavailable)
	/// method.
	#[must_use]
	fn get_StartWhenAvailable(&self) -> HrResult<bool> {
		let mut start = i16::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_StartWhenAvailable)(self.ptr(), &mut start)
		})
		.to_hrresult()
		.map(|_| start != 0)
	}

	/// [`ITaskSettings::get_StopIfGoingOnBatteries`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_stopifgoingonbatteries)
	/// method.
	#[must_use]
	fn get_StopIfGoingOnBatteries(&self) -> HrResult<bool> {
		let mut stop = i16::default();
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).get_StopIfGoingOnBatteries)(self.ptr(), &mut stop)
		})
		.to_hrresult()
		.map(|_| stop != 0)
	}

	/// [`ITaskSettings::get_WakeToRun`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-get_waketorun)
	/// method.
	#[must_use]
	fn get_WakeToRun(&self) -> HrResult<bool> {
		let mut wake = i16::default();
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).get_WakeToRun)(self.ptr(), &mut wake) })
			.to_hrresult()
			.map(|_| wake != 0)
	}

	/// [`ITaskSettings::put_AllowDemandStart`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_allowdemandstart)
	/// method.
	fn put_AllowDemandStart(&self, allow: bool) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_AllowDemandStart)(self.ptr(), allow as _) })
			.to_hrresult()
	}

	/// [`ITaskSettings::put_AllowHardTerminate`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_allowhardterminate)
	/// method.
	fn put_AllowHardTerminate(&self, allow: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_AllowHardTerminate)(self.ptr(), allow as _)
		})
		.to_hrresult()
	}

	/// [`ITaskSettings::put_Compatibility`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_compatibility)
	/// method.
	fn put_Compatibility(&self, level: co::TASK_COMPATIBILITY) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_Compatibility)(self.ptr(), level.raw()) })
			.to_hrresult()
	}

	fn_com_bstr_set! { put_DeleteExpiredTaskAfter: ITaskSettingsVT, delay;
		/// [`ITaskSettings::put_DeleteExpiredTaskAfter`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_deleteexpiredtaskafter)
		/// method.
	}

	/// [`ITaskSettings::put_DisallowStartIfOnBatteries`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_disallowstartifonbatteries)
	/// method.
	fn put_DisallowStartIfOnBatteries(&self, disallow: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_DisallowStartIfOnBatteries)(self.ptr(), disallow as _)
		})
		.to_hrresult()
	}

	fn_com_bstr_set! { put_ExecutionTimeLimit: ITaskSettingsVT, limit;
		/// [`ITaskSettings::put_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_executiontimelimit)
		/// method.
	}

	/// [`ITaskSettings::put_Hidden`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_hidden)
	/// method.
	fn put_Hidden(&self, hidden: bool) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_Hidden)(self.ptr(), hidden as _) })
			.to_hrresult()
	}

	/// [`ITaskSettings::put_MultipleInstances`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_multipleinstances)
	/// method.
	fn put_MultipleInstances(&self, policy: co::TASK_INSTANCES_POLICY) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_MultipleInstances)(self.ptr(), policy.raw())
		})
		.to_hrresult()
	}

	/// [`ITaskSettings::put_Priority`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_priority)
	/// method.
	fn put_Priority(&self, priority: i32) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_Priority)(self.ptr(), priority) })
			.to_hrresult()
	}

	fn_com_bstr_set! { put_RestartInterval: ITaskSettingsVT, interval;
		/// [`ITaskSettings::put_RestartInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_restartinterval)
		/// method.
	}

	/// [`ITaskSettings::put_RestartCount`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_restartcount)
	/// method.
	fn put_RestartCount(&self, count: i32) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_RestartCount)(self.ptr(), count) })
			.to_hrresult()
	}

	/// [`ITaskSettings::put_RunOnlyIfIdle`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_runonlyifidle)
	/// method.
	fn put_RunOnlyIfIdle(&self, run: bool) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_RunOnlyIfIdle)(self.ptr(), run as _) })
			.to_hrresult()
	}

	/// [`ITaskSettings::put_RunOnlyIfNetworkAvailable`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_runonlyifnetworkavailable)
	/// method.
	fn put_RunOnlyIfNetworkAvailable(&self, run: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_RunOnlyIfNetworkAvailable)(self.ptr(), run as _)
		})
		.to_hrresult()
	}

	/// [`ITaskSettings::put_StartWhenAvailable`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_startwhenavailable)
	/// method.
	fn put_StartWhenAvailable(&self, start: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_StartWhenAvailable)(self.ptr(), start as _)
		})
		.to_hrresult()
	}

	/// [`ITaskSettings::put_StopIfGoingOnBatteries`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_stopifgoingonbatteries)
	/// method.
	fn put_StopIfGoingOnBatteries(&self, stop: bool) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITaskSettingsVT>(self).put_StopIfGoingOnBatteries)(self.ptr(), stop as _)
		})
		.to_hrresult()
	}

	/// [`ITaskSettings::put_WakeToRun`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itasksettings-put_waketorun)
	/// method.
	fn put_WakeToRun(&self, wake: bool) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskSettingsVT>(self).put_WakeToRun)(self.ptr(), wake as _) })
			.to_hrresult()
	}
}
