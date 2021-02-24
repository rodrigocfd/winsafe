#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co::ERROR;
use crate::com::{IUnknown, IUnknownVtbl, PPVtbl, Vtbl};
use crate::ffi::HANDLE;
use crate::handles::HWND;
use crate::structs::IID;

/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,

	HrInit: fn(PPVtbl<Self>) -> u32,
	AddTab: fn(PPVtbl<Self>, HANDLE) -> u32,
	DeleteTab: fn(PPVtbl<Self>, HANDLE) -> u32,
	ActivateTab: fn(PPVtbl<Self>, HANDLE) -> u32,
	SetActiveAlt: fn(PPVtbl<Self>, HANDLE) -> u32,
}

impl_iid!(ITaskbarListVtbl, 0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090);

//------------------------------------------------------------------------------

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// COM interface.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
///
/// # Examples
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance};
///
/// let mut obj: shell::ITaskbarList = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct ITaskbarList {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPVtbl<ITaskbarListVtbl>> for ITaskbarList {
	fn from(ppv: PPVtbl<ITaskbarListVtbl>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPVtbl<IUnknownVtbl>)
		}
	}
}

impl ITaskbarList {
	/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	pub fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			into_result!( ((**ppv).ActivateTab)(ppv, hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	pub fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			into_result!( ((**ppv).AddTab)(ppv, hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	pub fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			into_result!( ((**ppv).DeleteTab)(ppv, hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	pub fn HrInit(&self) -> WinResult<()> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			into_result!( ((**ppv).HrInit)(ppv) )
		}
	}

	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			into_result!( ((**ppv).SetActiveAlt)(ppv, hwnd.ptr) )
		}
	}
}
