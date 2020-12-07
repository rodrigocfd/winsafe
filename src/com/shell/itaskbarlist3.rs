#![allow(non_snake_case)]

use crate::{ComVtbl, HWND, IID};
use crate::co;
use crate::ffi::Void;
use crate::shell::{ITaskbarList2, ITaskbarList2Vtbl};

type PPVtbl = *const *const ITaskbarList3Vtbl;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3Vtbl {
	iTaskbarList2Vtbl: ITaskbarList2Vtbl,
	SetProgressValue: fn(PPVtbl, *const Void, u64, u64) -> u32,
	SetProgressState: fn(PPVtbl, *const Void, u32) -> u32,
	RegisterTab: fn(PPVtbl, *const Void, *const Void) -> u32,
	UnregisterTab: fn(PPVtbl, *const Void) -> u32,
	SetTabOrder: fn(PPVtbl, *const Void, *const Void) -> u32,
	SetTabActive: fn(PPVtbl, *const Void, *const Void, u32) -> u32,
	ThumbBarAddButtons: fn(PPVtbl, *const Void, u32, *const Void) -> u32,
	ThumbBarUpdateButtons: fn(PPVtbl, *const Void, u32, *const Void) -> u32,
	ThumbBarSetImageList: fn(PPVtbl, *const Void, *const Void) -> u32,
	SetOverlayIcon: fn(PPVtbl, *const Void, *const Void, *const u16) -> u32,
	SetThumbnailTooltip: fn(PPVtbl, *const Void, *const u16) -> u32,
	SetThumbnailClip: fn(PPVtbl, *const Void, *const Void) -> u32,
}

impl ComVtbl for ITaskbarList3Vtbl {
	fn IID() -> IID {
		IID::new(0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf)
	}
}

//------------------------------------------------------------------------------

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// let mut obj = shell::ITaskbarList3::from(
///   CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
///   ),
/// );
/// ```
pub struct ITaskbarList3 {
	/// Base
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub ITaskbarList2: ITaskbarList2,
}

impl From<PPVtbl> for ITaskbarList3 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
		Self {
			ITaskbarList2: ITaskbarList2::from(ppv as *const *const ITaskbarList2Vtbl),
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
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match co::ERROR::from(
				((*(*ppv)).RegisterTab)(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr()),
			) {
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
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match co::ERROR::from(
				((*(*ppv)).SetProgressValue)(
					ppv, hwnd.as_ptr(), ullCompleted, ullTotal,
				),
			) {
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
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match co::ERROR::from(
				((*(*ppv)).SetTabActive)(ppv, hwndTab.as_ptr(), hwndMDI.as_ptr(), 0),
			) {
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
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			match co::ERROR::from(
				((*(*ppv)).SetTabOrder)(
					ppv, hwndTab.as_ptr(), hwndInsertBefore.as_ptr(),
				),
			) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}