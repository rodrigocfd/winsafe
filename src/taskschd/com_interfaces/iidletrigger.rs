#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;

com_interface! { IIdleTrigger: "d537d2b0-9fb3-4d34-9739-1ff5ce7b1ef3";
	/// [`IIdleTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iidletrigger)
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
	/// let idle_trigger = trigger
	///     .QueryInterface::<w::IIdleTrigger>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IIdleTrigger {}
impl taskschd_ITrigger for IIdleTrigger {}
impl taskschd_IIdleTrigger for IIdleTrigger {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IIdleTrigger`](crate::IIdleTrigger).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IIdleTrigger: taskschd_ITrigger {}
