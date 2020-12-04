#![allow(non_snake_case)]

use crate::*;
use crate::com::*;
use crate::ffi::*;

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// -> `IUnknown`.
pub struct ITaskbarList {
	base: IUnknown,
}

#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,
	hrInit: *mut Void,
	addTab: *mut Void,
	deleteTab: *mut Void,
	activateTab: *mut Void,
	setActiveAlt: fn(*mut *mut ITaskbarListVtbl, *mut Void),
}

impl From<*mut *mut ITaskbarListVtbl> for ITaskbarList {
	fn from(ppv: *mut *mut ITaskbarListVtbl) -> Self {
		Self {
			base: IUnknown::from(ppv as *mut *mut IUnknownVtbl)
		}
	}
}

impl ITaskbarList {
	pub fn AddRef(&self) -> u32 { self.base.AddRef() }
	pub fn Release(&self) -> u32 { self.base.Release() }

	/// [ITaskbarList::SetActiveAlt](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) {
		let ppv = self.base.ppv::<ITaskbarListVtbl>();
		let pfun = unsafe { (*(*ppv)).setActiveAlt };
		pfun(ppv, hwnd.0)
	}
}