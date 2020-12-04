#![allow(non_snake_case)]

use crate::com::*;
use crate::ffi::*;

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// -> `IUnknown`.
#[repr(C)]
pub struct ITaskbarList(IUnknown);

#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,
	hrInit: *mut Void,
	addTab: *mut Void,
	deleteTab: *mut Void,
	activateTab: *mut Void,
	setActiveAlt: *mut Void,
}