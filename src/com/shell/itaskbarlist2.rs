#![allow(non_snake_case)]

use crate::co;
use crate::com::{IUnknown, IUnknownVtbl};
use crate::com::shell::ITaskbarList;
use crate::ffi::Void;
use crate::HWND;

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// ->
/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// ->
/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown).
pub struct ITaskbarList2 {
	/// Base
	/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist).
	pub iTaskbarList: ITaskbarList,
}

#[repr(C)]
pub struct ITaskbarList2Vtbl {
	markFullscreenWindow: fn(*const *const ITaskbarList2Vtbl, *const Void, u32) -> u32,
}

impl From<*const *const ITaskbarList2Vtbl> for ITaskbarList2 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: *const *const ITaskbarList2Vtbl) -> Self {
		Self {
			iTaskbarList: ITaskbarList {
				iUnknown: IUnknown::from(ppv as *const *const IUnknownVtbl),
			},
		}
	}
}

impl ITaskbarList2 {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	pub fn MarkFullscreenWindow(
		&self, hwnd: HWND, fFullscreen: bool) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList.iUnknown.ppv::<ITaskbarList2Vtbl>();
			let pfun = (*(*ppv)).markFullscreenWindow;

			match co::ERROR::from(pfun(ppv, hwnd.as_ptr(), fFullscreen as u32)) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}