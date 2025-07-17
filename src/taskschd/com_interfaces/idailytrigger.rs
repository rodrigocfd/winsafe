#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IDailyTrigger: "126c5cd8-b288-41d5-8dbf-e491446adc5c";
	/// [`IDailyTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-idailytrigger)
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
	/// let trigger: w::ITrigger; // initialized somewhere
	/// # let trigger = unsafe { w::ITrigger::null() };
	///
	/// let daily_trigger = trigger
	///     .QueryInterface::<w::IDailyTrigger>()?;
	/// # w::HrResult::Ok(())
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IDailyTrigger: taskschd_ITrigger {
	/// [`IDailyTrigger::get_DaysInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-get_daysinterval)
	/// method.
	#[must_use]
	fn get_DaysInterval(&self) -> HrResult<i16> {
		let mut days = i16::default();
		HrRet(unsafe { (vt::<IDailyTriggerVT>(self).get_DaysInterval)(self.ptr(), &mut days) })
			.to_hrresult()
			.map(|_| days)
	}

	fn_com_bstr_get! { get_RandomDelay: IDailyTriggerVT;
		/// [`IDailyTrigger::get_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-get_randomdelay)
		/// method.
	}

	/// [`IDailyTrigger::put_DaysInterval`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-put_daysinterval)
	/// method.
	fn put_DaysInterval(&self, days: i16) -> HrResult<()> {
		HrRet(unsafe { (vt::<IDailyTriggerVT>(self).put_DaysInterval)(self.ptr(), days) })
			.to_hrresult()
	}

	fn_com_bstr_set! { put_RandomDelay: IDailyTriggerVT, random_delay;
		/// [`IDailyTrigger::put_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-idailytrigger-put_randomdelay)
		/// method.
	}
}
