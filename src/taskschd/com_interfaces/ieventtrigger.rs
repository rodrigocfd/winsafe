#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IEventTrigger: "d45b0167-9653-4eef-b94f-0732ca7af251";
	/// [`IEventTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-ieventtrigger)
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
	/// let event_trigger = trigger
	///     .QueryInterface::<w::IEventTrigger>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IEventTrigger {}
impl taskschd_ITrigger for IEventTrigger {}
impl taskschd_IEventTrigger for IEventTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IEventTrigger`](crate::IEventTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IEventTrigger: taskschd_ITrigger {
	fn_com_bstr_get! { get_Delay: IEventTriggerVT;
		/// [`IEventTrigger::get_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-get_delay)
		/// method.
	}

	fn_com_bstr_get! { get_Subscription: IEventTriggerVT;
		/// [`IEventTrigger::get_Subscription`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-get_subscription)
		/// method.
	}

	fn_com_bstr_set! { put_Delay: IEventTriggerVT, delay;
		/// [`IEventTrigger::put_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-put_delay)
		/// method.
	}

	fn_com_bstr_set! { put_Subscription: IEventTriggerVT, subscription;
		/// [`IEventTrigger::put_Subscription`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-put_subscription)
		/// method.
	}
}
