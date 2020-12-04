#![allow(non_snake_case)]

use crate::HWND;
use crate::com::{IUnknown, IUnknownVtbl};
use crate::ffi::Void;

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// ->
/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown).
pub struct ITaskbarList {
	/// Base
	/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown).
	pub base: IUnknown,
}

#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,
	hrInit: fn(*mut *mut ITaskbarListVtbl) -> u32,
	addTab: fn(*mut *mut ITaskbarListVtbl, *mut Void) -> u32,
	deleteTab: fn(*mut *mut ITaskbarListVtbl, *mut Void) -> u32,
	activateTab: fn(*mut *mut ITaskbarListVtbl, *mut Void) -> u32,
	setActiveAlt: fn(*mut *mut ITaskbarListVtbl, *mut Void) -> u32,
}

impl From<*mut *mut ITaskbarListVtbl> for ITaskbarList {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: *mut *mut ITaskbarListVtbl) -> Self {
		Self {
			base: IUnknown::from(ppv as *mut *mut IUnknownVtbl)
		}
	}
}

impl ITaskbarList {
	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) {
		let ppv = self.base.ppv::<ITaskbarListVtbl>();
		let pfun = unsafe { (*(*ppv)).setActiveAlt };
		pfun(ppv, hwnd.0);
	}
}