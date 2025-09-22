#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { IPrincipal: "d98d51e5-c9b4-496a-a9c1-18980261cf0f";
	/// [`IPrincipal`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iprincipal)
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
	/// let task: w::ITaskDefinition; // initialized somewhere
	/// # let task = unsafe { w::ITaskDefinition::null() };
	///
	/// let principal = task
	///     .QueryInterface::<w::IPrincipal>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl oleaut_IDispatch for IPrincipal {}
impl taskschd_IPrincipal for IPrincipal {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IPrincipal`](crate::IPrincipal).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IPrincipal: oleaut_IDispatch {
	/// [`IPrincipal::get_RunLevel`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iprincipal-get_runlevel)
	/// method.
	#[must_use]
	fn get_RunLevel(&self) -> HrResult<co::TASK_RUNLEVEL> {
		let mut rl = co::TASK_RUNLEVEL::default();
		HrRet(unsafe { (vt::<IPrincipalVT>(self).get_RunLevel)(self.ptr(), rl.as_mut()) })
			.to_hrresult()
			.map(|_| rl)
	}

	/// [`IPrincipal::put_RunLevel`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iprincipal-put_runlevel)
	/// method.
	fn put_RunLevel(&self, run_level: co::TASK_RUNLEVEL) -> HrResult<()> {
		HrRet(unsafe { (vt::<IPrincipalVT>(self).put_RunLevel)(self.ptr(), run_level.raw()) })
			.to_hrresult()
	}
}
