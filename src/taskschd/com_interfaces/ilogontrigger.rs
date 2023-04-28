#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::BSTR;
use crate::prelude::{oleaut_IDispatch, taskschd_ITrigger};
use crate::vt::ITriggerVT;

/// [`ILogonTrigger`](crate::ILogonTrigger) interface.
#[repr(C)]
pub struct ILogonTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Delay: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Delay: fn(ComPtr, PCSTR) -> HRES,
	pub get_UserId: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_UserId: fn(ComPtr, PCSTR) -> HRES,
}

com_interface! { ILogonTrigger: "72dade38-fae4-4b3e-baf4-5d009af02b1c";
	/// [`ILogonTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iLogontrigger)
	/// COM interface over [`ILogonTriggerVT`](crate::vt::ILogonTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{ILogonTrigger, ITrigger};
	///
	/// let trigger: ILogonTrigger; // initialized somewhere
	/// # let trigger = ITrigger::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let Logon_trigger = trigger
	///     .QueryInterface::<ILogonTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for ILogonTrigger {}
impl taskschd_ITrigger for ILogonTrigger {}
impl taskschd_ILogonTrigger for ILogonTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ILogonTrigger`](crate::ILogonTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ILogonTrigger: taskschd_ITrigger {
	/// [`ILogonTrigger::get_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_delay)
	/// method.
	#[must_use]
	fn get_Delay(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ILogonTriggerVT>();
			ok_to_hrresult((vt.get_Delay)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ILogonTrigger::get_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_userid)
	/// method.
	#[must_use]
	fn get_UserId(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		unsafe {
			let vt = self.vt_ref::<ILogonTriggerVT>();
			ok_to_hrresult((vt.get_UserId)(self.ptr(), &mut pstr))
		}.map(|_| {
			let bstr = unsafe { BSTR::from_ptr(pstr) };
			bstr.to_string()
		})
	}

	/// [`ILogonTrigger::put_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_delay)
	/// method.
	fn put_RandomDelay(&self, delay: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ILogonTriggerVT>();
			ok_to_hrresult(
				(vt.put_Delay)(
					self.ptr(), BSTR::SysAllocString(delay)?.as_ptr(),
				),
			)
		}
	}

	/// [`ILogonTrigger::put_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_userid)
	/// method.
	fn put_UserId(&self, user_id: &str) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ILogonTriggerVT>();
			ok_to_hrresult(
				(vt.put_UserId)(
					self.ptr(), BSTR::SysAllocString(user_id)?.as_ptr(),
				),
			)
		}
	}
}
