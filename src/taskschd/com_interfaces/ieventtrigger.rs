#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
use crate::prelude::{oleaut_IDispatch, taskschd_ITrigger};
use crate::vt::ITriggerVT;

/// [`IEventTrigger`](crate::IEventTrigger) virtual table.
#[repr(C)]
pub struct IEventTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Subscription: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Subscription: fn(ComPtr, PCSTR) -> HRES,
	pub get_Delay: fn(ComPtr, *mut PSTR) -> HRES,
	pub put_Delay: fn(ComPtr, PCSTR) -> HRES,
	pub get_ValueQueries: fn(ComPtr, *mut ComPtr) -> HRES,
	pub put_ValueQueries: fn(ComPtr, ComPtr) -> HRES,
}

com_interface! { IEventTrigger: "d45b0167-9653-4eef-b94f-0732ca7af251";
	/// [`IEventTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-ieventtrigger)
	/// COM interface over [`IEventTriggerVT`](crate::vt::IEventTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IEventTrigger, ITrigger};
	///
	/// let trigger: IEventTrigger; // initialized somewhere
	/// # let trigger = ITrigger::from(unsafe { winsafe::ComPtr::null() });
	///
	/// let event_trigger = trigger
	///     .QueryInterface::<IEventTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IEventTrigger: taskschd_ITrigger {
	fn_bstr_get! { get_Delay: IEventTriggerVT;
		/// [`IEventTrigger::get_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-get_delay)
		/// method.
	}

	fn_bstr_get! { get_Subscription: IEventTriggerVT;
		/// [`IEventTrigger::get_Subscription`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-get_subscription)
		/// method.
	}

	fn_bstr_set! { put_Delay: IEventTriggerVT, delay;
		/// [`IEventTrigger::put_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-put_delay)
		/// method.
	}

	fn_bstr_set! { put_Subscription: IEventTriggerVT, subscription;
		/// [`IEventTrigger::put_Subscription`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ieventtrigger-put_subscription)
		/// method.
	}
}
