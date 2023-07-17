#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PSTR};
use crate::ole::decl::HrResult;
use crate::prelude::{oleaut_IDispatch, taskschd_ITrigger};
use crate::vt::ITriggerVT;

/// [`IBootTrigger`](crate::IBootTrigger) virtual table.
#[repr(C)]
pub struct IBootTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Delay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Delay: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IBootTrigger: "2a9c35da-d357-41f4-bbc1-207ac1b1f3cb";
	/// [`IBootTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iboottrigger)
	/// COM interface over [`IBootTriggerVT`](crate::vt::IBootTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IBootTrigger, ITrigger};
	///
	/// let trigger: ITrigger; // initialized somewhere
	/// # let trigger = unsafe { ITrigger::null() };
	///
	/// let boot_trigger = trigger
	///     .QueryInterface::<IBootTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for IBootTrigger {}
impl taskschd_ITrigger for IBootTrigger {}
impl taskschd_IBootTrigger for IBootTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IBootTrigger`](crate::IBootTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IBootTrigger: taskschd_ITrigger {
	fn_com_bstr_get! { get_Delay: IBootTriggerVT;
		/// [`IBootTrigger::get_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iboottrigger-get_delay)
		/// method.
	}

	fn_com_bstr_set! { put_Delay: IBootTriggerVT, delay;
		/// [`IBootTrigger::put_Delay`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iboottrigger-put_delay)
		/// method.
	}
}