#![allow(non_camel_case_types, non_snake_case)]

use crate::prelude::*;
use crate::vt::*;

/// [`IIdleTrigger`](crate::IIdleTrigger) virtual table.
#[repr(C)]
pub struct IIdleTriggerVT {
	pub ITriggerVT: ITriggerVT,
}

com_interface! { IIdleTrigger: "d537d2b0-9fb3-4d34-9739-1ff5ce7b1ef3";
	/// [`IIdleTrigger`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iidletrigger)
	/// COM interface over [`IIdleTriggerVT`](crate::vt::IIdleTriggerVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IIdleTrigger, ITrigger};
	///
	/// let trigger: ITrigger; // initialized somewhere
	/// # let trigger = unsafe { ITrigger::null() };
	///
	/// let idle_trigger = trigger
	///     .QueryInterface::<IIdleTrigger>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IIdleTrigger: taskschd_ITrigger {}
