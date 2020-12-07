#![allow(non_snake_case)]

use crate::co;
use crate::ffi::Void;
use crate::HWND;
use crate::shell::{ITaskbarList2, ITaskbarList2Vtbl};

type PPVtbl = *const *const ITaskbarList3Vtbl;

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Usually instantiated as:
/// ```rust,ignore
/// let mut itl = shell::ITaskbarList3::from(
///   CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
///     &shell::iid::ITaskbarList3,
///   ),
/// );
/// ```
pub struct ITaskbarList3 {
	/// Base
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub iTaskbarList2: ITaskbarList2,
}

#[repr(C)]
pub struct ITaskbarList3Vtbl {
	iTaskbarList2Vtbl: ITaskbarList2Vtbl,
	pub SetProgressValue: fn(PPVtbl, *const Void, u64, u64) -> u32,
	pub SetProgressState: fn(PPVtbl, *const Void, u32) -> u32,
	pub RegisterTab: fn(PPVtbl, *const Void, *const Void) -> u32,
	pub UnregisterTab: fn(PPVtbl, *const Void) -> u32,
	pub SetTabOrder: fn(PPVtbl, *const Void, *const Void) -> u32,
	pub SetTabActive: fn(PPVtbl, *const Void, *const Void, u32) -> u32,
	pub ThumbBarAddButtons: fn(PPVtbl, *const Void, u32, *const Void) -> u32,
	pub ThumbBarUpdateButtons: fn(PPVtbl, *const Void, u32, *const Void) -> u32,
	pub ThumbBarSetImageList: fn(PPVtbl, *const Void, *const Void) -> u32,
	pub SetOverlayIcon: fn(PPVtbl, *const Void, *const Void, *const u16) -> u32,
	pub SetThumbnailTooltip: fn(PPVtbl, *const Void, *const u16) -> u32,
	pub SetThumbnailClip: fn(PPVtbl, *const Void, *const Void) -> u32,
}

impl From<PPVtbl> for ITaskbarList3 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
		Self {
			iTaskbarList2: ITaskbarList2::from(ppv as *const *const ITaskbarList2Vtbl),
		}
	}
}

impl ITaskbarList3 {
	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	pub fn RegisterTab(
		&self, hwndTab: HWND, hwndMDI: HWND) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList2.iTaskbarList.iUnknown.ppv::<ITaskbarList3Vtbl>();
			let pfun = (*(*ppv)).RegisterTab;

			match co::ERROR::from(pfun(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr())) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	pub fn SetProgressValue(
		&self, hwnd: HWND,
		ullCompleted: u64, ullTotal: u64) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList2.iTaskbarList.iUnknown.ppv::<ITaskbarList3Vtbl>();
			let pfun = (*(*ppv)).SetProgressValue;

			match co::ERROR::from(pfun(ppv, hwnd.as_ptr(), ullCompleted, ullTotal)) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	pub fn SetTabActive(
		&self, hwndTab: HWND, hwndMDI: HWND) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList2.iTaskbarList.iUnknown.ppv::<ITaskbarList3Vtbl>();
			let pfun = (*(*ppv)).SetTabActive;

			match co::ERROR::from(pfun(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr(), 0)) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	pub fn SetTabOrder(
		&self, hwndTab: HWND, hwndInsertBefore: HWND) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList2.iTaskbarList.iUnknown.ppv::<ITaskbarList3Vtbl>();
			let pfun = (*(*ppv)).SetTabOrder;

			match co::ERROR::from(pfun(ppv, hwndTab.as_ptr(), hwndInsertBefore.as_ptr())) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}