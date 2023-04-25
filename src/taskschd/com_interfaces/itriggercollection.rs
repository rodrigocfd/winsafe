#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::VARIANT;
use crate::prelude::{oleaut_IDispatch, oleaut_Variant};
use crate::taskschd::decl::ITrigger;
use crate::vt::IDispatchVT;

/// [`ITriggerCollection`](crate::ITriggerCollection) virtual table.
#[repr(C)]
pub struct ITriggerCollectionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Count: fn(ComPtr, *mut i32) -> HRES,
	pub get_Item: fn(ComPtr, i32, *mut ComPtr) -> HRES,
	pub get__NewEnum: fn(ComPtr, *mut ComPtr) -> HRES,
	pub Create: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub Remove: fn(ComPtr, VARIANT) -> HRES,
	pub Clear: fn(ComPtr) -> HRES,
}

com_interface! { ITriggerCollection: "85df5081-1b24-4f32-878a-d9d14df4cb77";
	/// [`ITriggerCollection`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-itriggercollection)
	/// COM interface over [`ITriggerCollectionVT`](crate::vt::ITriggerCollectionVT).
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_ITriggerCollection: oleaut_IDispatch {
	/// [`ITriggerCollection::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-clear)
	/// method.
	fn Clear(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerCollectionVT>();
			ok_to_hrresult((vt.Clear)(self.ptr()))
		}
	}

	/// [`ITriggerCollection::Create`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-create)
	/// method.
	#[must_use]
	fn Create(&self,
		trigger_type: co::TASK_TRIGGER_TYPE2) -> HrResult<ITrigger>
	{
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<ITriggerCollectionVT>();
			ok_to_hrresult(
				(vt.Create)(self.ptr(), trigger_type.0, &mut ppv_queried),
			).map(|_| ITrigger::from(ppv_queried))
		}
	}

	/// [`ITriggerCollection::get_Count`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-get_count)
	/// method.
	#[must_use]
	fn get_Count(&self) -> HrResult<i32> {
		let mut count = i32::default();
		unsafe {
			let vt = self.vt_ref::<ITriggerCollectionVT>();
			ok_to_hrresult((vt.get_Count)(self.ptr(), &mut count))
		}.map(|_| count)
	}

	/// [`ITriggerCollection::get_Item`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-get_item)
	/// method.
	#[must_use]
	fn get_Item(&self, index: i32) -> HrResult<ITrigger> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<ITriggerCollectionVT>();
			ok_to_hrresult((vt.get_Item)(self.ptr(), index, &mut ppv_queried))
				.map(|_| ITrigger::from(ppv_queried))
		}
	}

	/// [`ITriggerCollection::Remove`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-itriggercollection-remove)
	/// method.
	fn Remove(&self, index: i32) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITriggerCollectionVT>();
			ok_to_hrresult((vt.Remove)(self.ptr(), VARIANT::new_i32(index)))
		}
	}
}
