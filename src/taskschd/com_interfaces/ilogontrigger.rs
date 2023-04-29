#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCSTR, PSTR};
use crate::ole::decl::{ComPtr, HrResult};
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
	fn_bstr_get! { get_Delay: ILogonTriggerVT;
		/// [`ILogonTrigger::get_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_delay)
		/// method.
	}

	fn_bstr_get! { get_UserId: ILogonTriggerVT;
		/// [`ILogonTrigger::get_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-get_userid)
		/// method.
	}

	fn_bstr_set! { put_Delay: ILogonTriggerVT, delay;
		/// [`ILogonTrigger::put_RandomDelay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_delay)
		/// method.
	}

	fn_bstr_set! { put_UserId: ILogonTriggerVT, user_id;
		/// [`ILogonTrigger::put_UserId`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-ilogontrigger-put_userid)
		/// method.
	}
}
