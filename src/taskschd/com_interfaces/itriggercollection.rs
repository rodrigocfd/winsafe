#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::taskschd::vts::*;

com_interface! { ITriggerCollection: "85df5081-1b24-4f32-878a-d9d14df4cb77";
	/// [`ITriggerCollection`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itriggercollection)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for ITriggerCollection {}
impl taskschd_ITriggerCollection for ITriggerCollection {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`ITriggerCollection`](crate::ITriggerCollection).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITriggerCollection: oleaut_IDispatch {
	fn_com_noparm! { Clear: ITriggerCollectionVT;
		/// [`ITriggerCollection::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-clear)
		/// method.
	}

	/// [`ITriggerCollection::Create`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-create)
	/// method.
	#[must_use]
	fn Create(&self, trigger_type: co::TASK_TRIGGER_TYPE2) -> HrResult<ITrigger> {
		let mut queried = unsafe { ITrigger::null() };
		HrRet(unsafe {
			(vt::<ITriggerCollectionVT>(self).Create)(
				self.ptr(),
				trigger_type.raw(),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`ITriggerCollection::get_Count`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-get_count)
	/// method.
	#[must_use]
	fn get_Count(&self) -> HrResult<i32> {
		let mut count = 0i32;
		HrRet(unsafe { (vt::<ITriggerCollectionVT>(self).get_Count)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`ITriggerCollection::get_Item`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-get_item)
	/// method.
	#[must_use]
	fn get_Item(&self, index: i32) -> HrResult<ITrigger> {
		let mut queried = unsafe { ITrigger::null() };
		HrRet(unsafe {
			(vt::<ITriggerCollectionVT>(self).get_Item)(self.ptr(), index, queried.as_mut())
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`ITriggerCollection::Remove`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-remove)
	/// method.
	fn Remove(&self, index: i32) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<ITriggerCollectionVT>(self).Remove)(self.ptr(), Variant::I4(index).to_raw()?)
		})
		.to_hrresult()
	}
}
