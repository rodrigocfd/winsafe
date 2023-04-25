#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::BSTR;
use crate::prelude::oleaut_IDispatch;
use crate::vt::IDispatchVT;

/// [`ITrigger`](crate::ITrigger) virtual table.
#[repr(C)]
pub struct ITriggerVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Type: fn(ComPtr, *mut u32) -> HRES,
	pub get_Id: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Id: fn(ComPtr, PCSTR) -> HRES,
	pub get_Repetition: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_Repetition: fn(ComPtr, ComPtr) -> HRES,
	pub get_ExecutionTimeLimit: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_ExecutionTimeLimit: fn(ComPtr, PCSTR) -> HRES,
	pub get_StartBoundary: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_StartBoundary: fn(ComPtr, PCSTR) -> HRES,
	pub get_EndBoundary: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_EndBoundary: fn(ComPtr, PCSTR) -> HRES,
	pub get_Enabled: fn(ComPtr, *mut i16) -> HRES,
	pub put_Enabled: fn(ComPtr, i16) -> HRES,
}

com_interface! { ITrigger: "09941815-ea89-4b5b-89e0-2a773801fac3";
	/// [`ITrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itrigger)
	/// COM interface over [`ITriggerVT`](crate::vt::ITriggerVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITrigger: oleaut_IDispatch {
	/// [`ITrigger::get_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_enabled)
	/// method.
	#[must_use]
	fn get_Enabled(&self) -> HrResult<bool> {
		let mut enabled = i16::default();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_Enabled)(self.ptr(), &mut enabled))
		}.map(|_| enabled != 0)
	}

	/// [`ITrigger::get_EndBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_endboundary)
	/// method.
	#[must_use]
	fn get_EndBoundary(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_EndBoundary)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITrigger::get_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_executiontimelimit)
	/// method.
	#[must_use]
	fn get_ExecutionTimeLimit(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_ExecutionTimeLimit)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITrigger::get_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_id)
	/// method.
	#[must_use]
	fn get_Id(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_Id)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITrigger::get_StartBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_startboundary)
	/// method.
	#[must_use]
	fn get_StartBoundary(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_StartBoundary)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ITrigger::get_Type`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-get_type)
	/// method.
	#[must_use]
	fn get_Type(&self) -> HrResult<co::TASK_TRIGGER_TYPE2> {
		let mut ty = co::TASK_TRIGGER_TYPE2::default();
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.get_Type)(self.ptr(), &mut ty.0))
		}.map(|_| ty)
	}

	/// [`ITrigger::put_Enabled`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_enabled)
	/// method.
	fn put_Enabled(&self, enabled: bool) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult((vt.put_Enabled)(self.ptr(), enabled as _))
		}
	}

	/// [`ITrigger::put_EndBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_endboundary)
	/// method.
	fn put_EndBoundary(&self, id: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult(
				(vt.put_EndBoundary)(
					self.ptr(),
					BSTR::SysAllocString(id)?.as_ptr(),
				),
			)
		}
	}

	/// [`ITrigger::put_ExecutionTimeLimit`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_executiontimelimit)
	/// method.
	fn put_ExecutionTimeLimit(&self, id: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult(
				(vt.put_ExecutionTimeLimit)(
					self.ptr(),
					BSTR::SysAllocString(id)?.as_ptr(),
				),
			)
		}
	}

	/// [`ITrigger::put_Id`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_id)
	/// method.
	fn put_Id(&self, id: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult(
				(vt.put_Id)(self.ptr(), BSTR::SysAllocString(id)?.as_ptr()),
			)
		}
	}

	/// [`ITrigger::put_StartBoundary`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itrigger-put_startboundary)
	/// method.
	fn put_StartBoundary(&self, id: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerVT>();
			ok_to_hrresult(
				(vt.put_StartBoundary)(
					self.ptr(),
					BSTR::SysAllocString(id)?.as_ptr(),
				),
			)
		}
	}
}
