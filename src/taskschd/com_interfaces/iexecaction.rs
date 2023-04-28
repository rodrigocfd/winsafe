#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::BSTR;
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
	/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_arguments)
	/// method.
	#[must_use]
	fn get_Arguments(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult((vt.get_Arguments)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`IExecAction::get_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_path)
	/// method.
	#[must_use]
	fn get_Path(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult((vt.get_Path)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`IExecAction::get_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-get_workingdirectory)
	/// method.
	#[must_use]
	fn get_WorkingDirectory(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult((vt.get_WorkingDirectory)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`IExecAction::get_Arguments`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_arguments)
	/// method.
	fn put_Arguments(&self, arguments: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult(
				(vt.put_Arguments)(
					self.ptr(), BSTR::SysAllocString(arguments)?.as_ptr(),
				),
			)
		}
	}

	/// [`IExecAction::put_Path`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_path)
	/// method.
	fn put_Path(&self, path: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult(
				(vt.put_Path)(
					self.ptr(), BSTR::SysAllocString(path)?.as_ptr(),
				),
			)
		}
	}

	/// [`IExecAction::put_WorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iexecaction-put_workingdirectory)
	/// method.
	fn put_WorkingDirectory(&self, working_directory: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IExecActionVT>();
			ok_to_hrresult(
				(vt.put_WorkingDirectory)(
					self.ptr(), BSTR::SysAllocString(working_directory)?.as_ptr(),
				),
			)
		}
	}
}
