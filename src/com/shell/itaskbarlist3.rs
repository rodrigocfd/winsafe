#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co::ERROR;
use crate::com::{PPVtbl, Vtbl};
use crate::com::shell::{co, ITaskbarList2, ITaskbarList2Vtbl};
use crate::ffi::{HANDLE, PCSTR, PCVOID};
use crate::handles::HWND;
use crate::structs::IID;

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3Vtbl {
	iTaskbarList2Vtbl: ITaskbarList2Vtbl,

	SetProgressValue: fn(PPVtbl<Self>, HANDLE, u64, u64) -> u32,
	SetProgressState: fn(PPVtbl<Self>, HANDLE, u32) -> u32,
	RegisterTab: fn(PPVtbl<Self>, HANDLE, HANDLE) -> u32,
	UnregisterTab: fn(PPVtbl<Self>, HANDLE) -> u32,
	SetTabOrder: fn(PPVtbl<Self>, HANDLE, HANDLE) -> u32,
	SetTabActive: fn(PPVtbl<Self>, HANDLE, HANDLE, u32) -> u32,
	ThumbBarAddButtons: fn(PPVtbl<Self>, HANDLE, u32, PCVOID) -> u32,
	ThumbBarUpdateButtons: fn(PPVtbl<Self>, HANDLE, u32, PCVOID) -> u32,
	ThumbBarSetImageList: fn(PPVtbl<Self>, HANDLE, HANDLE) -> u32,
	SetOverlayIcon: fn(PPVtbl<Self>, HANDLE, HANDLE, PCSTR) -> u32,
	SetThumbnailTooltip: fn(PPVtbl<Self>, HANDLE, PCSTR) -> u32,
	SetThumbnailClip: fn(PPVtbl<Self>, HANDLE, PCVOID) -> u32,
}

impl_iid!(ITaskbarList3Vtbl, 0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);

//------------------------------------------------------------------------------

/// [`ITaskbarList3`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist3)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList2`](crate::shell::ITaskbarList2);
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
///
/// # Examples
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let mut obj: shell::ITaskbarList3 = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct ITaskbarList3 {
	/// Methods of base interface
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2).
	pub ITaskbarList2: ITaskbarList2,
}

impl From<PPVtbl<ITaskbarList3Vtbl>> for ITaskbarList3 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl<ITaskbarList3Vtbl>) -> Self {
		Self {
			ITaskbarList2: ITaskbarList2::from(ppv as PPVtbl<ITaskbarList2Vtbl>),
		}
	}
}

impl ITaskbarList3 {
	/// [`ITaskbarList3::RegisterTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-registertab)
	/// method.
	pub fn RegisterTab(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			into_result!( ((**ppv).RegisterTab)(ppv, hwndTab.ptr, hwndMDI.ptr) )
		}
	}

	/// [`ITaskbarList3::SetProgressState`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressstate)
	/// method.
	pub fn SetProgressState(&self,
		hwnd: HWND, tbpfFlags: co::TBPF) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			into_result!(
				((**ppv).SetProgressState)(ppv, hwnd.ptr, tbpfFlags.0)
			)
		}
	}

	/// [`ITaskbarList3::SetProgressValue`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-setprogressvalue)
	/// method.
	pub fn SetProgressValue(&self,
		hwnd: HWND, ullCompleted: u64, ullTotal: u64) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			into_result!(
				((**ppv).SetProgressValue)(ppv, hwnd.ptr, ullCompleted, ullTotal)
			)
		}
	}

	/// [`ITaskbarList3::SetTabActive`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settabactive)
	/// method.
	pub fn SetTabActive(&self,
		hwndTab: HWND, hwndMDI: HWND) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			into_result!(
				((**ppv).SetTabActive)(ppv, hwndTab.ptr, hwndMDI.ptr, 0)
			)
		}
	}

	/// [`ITaskbarList3::SetTabOrder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist3-settaborder)
	/// method.
	pub fn SetTabOrder(&self,
		hwndTab: HWND, hwndInsertBefore: HWND) -> WinResult<()>
	{
		unsafe {
			let ppv = self.ITaskbarList2.ITaskbarList.IUnknown.ppv::<ITaskbarList3Vtbl>();
			into_result!(
				((**ppv).SetTabOrder)(ppv, hwndTab.ptr, hwndInsertBefore.ptr)
			)
		}
	}
}
