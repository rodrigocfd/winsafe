#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PSTR};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{oleaut_IDispatch, taskschd_ITrigger};
use crate::vt::ITriggerVT;

/// [`IDailyTrigger`](crate::IDailyTrigger) virtual table.
#[repr(C)]
pub struct IDailyTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_DaysInterval: fn(COMPTR, *mut i16) -> HRES,
	pub put_DaysInterval: fn(COMPTR, i16) -> HRES,
	pub get_RandomDelay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_RandomDelay: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IDailyTrigger: "126c5cd8-b288-41d5-8dbf-e491446adc5c";
	/// [`IDailyTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-idailytrigger)
	/// COM interface over [`IDailyTriggerVT`](crate::vt::IDailyTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IDailyTrigger, ITrigger};
	///
	/// let trigger: IDailyTrigger; // initialized somewhere
	/// # let trigger = unsafe { ITrigger::null() };
	///
	/// let daily_trigger = trigger
	///     .QueryInterface::<IDailyTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for IDailyTrigger {}
impl taskschd_ITrigger for IDailyTrigger {}
impl taskschd_IDailyTrigger for IDailyTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IDailyTrigger`](crate::IDailyTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IDailyTrigger: taskschd_ITrigger {
	/// [`IDailyTrigger::get_DaysInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-get_daysinterval)
	/// method.
	#[must_use]
	fn get_DaysInterval(&self) -> HrResult<i16> {
		let mut days = i16::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDailyTriggerVT>(self).get_DaysInterval)(
					self.ptr(),
					&mut days,
				)
			},
		).map(|_| days)
	}

	fn_bstr_get! { get_RandomDelay: IDailyTriggerVT;
		/// [`IDailyTrigger::get_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-get_randomdelay)
		/// method.
	}

	/// [`IDailyTrigger::put_DaysInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-put_daysinterval)
	/// method.
	fn put_DaysInterval(&self, days: i16) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDailyTriggerVT>(self).put_DaysInterval)(self.ptr(), days)
			},
		)
	}

	fn_bstr_set! { put_RandomDelay: IDailyTriggerVT, random_delay;
		/// [`IDailyTrigger::put_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-put_randomdelay)
		/// method.
	}
}
