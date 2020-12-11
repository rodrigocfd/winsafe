#![allow(non_snake_case)]

use crate::co::ERROR;
use crate::com::{IUnknown, IUnknownVtbl, PPVtbl, Vtbl};
use crate::ffi::HANDLE;
use crate::handles::HWND;
use crate::structs::IID;

vtbl_type! {
	/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
	ITaskbarListVtbl,
	0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090,

	iUnknownVtbl, IUnknownVtbl
	HrInit, fn(PPVtbl<Self>) -> u32
	AddTab, fn(PPVtbl<Self>, HANDLE) -> u32
	DeleteTab, fn(PPVtbl<Self>, HANDLE) -> u32
	ActivateTab, fn(PPVtbl<Self>, HANDLE) -> u32
	SetActiveAlt, fn(PPVtbl<Self>, HANDLE) -> u32
}

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
/// let mut obj: shell::ITaskbarList = CoCreateInstance(
///   &shell::clsid::TaskbarList,
///   None,
///   co::CLSCTX::INPROC_SERVER,
/// );
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
	pub fn ActivateTab(&self, hwnd: HWND) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			match ERROR::from(
				((*(*ppv)).ActivateTab)(ppv, hwnd.as_ptr()),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	pub fn AddTab(&self, hwnd: HWND) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			match ERROR::from(
				((*(*ppv)).AddTab)(ppv, hwnd.as_ptr()),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	pub fn DeleteTab(&self, hwnd: HWND) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			match ERROR::from(
				((*(*ppv)).DeleteTab)(ppv, hwnd.as_ptr()),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	pub fn HrInit(&self) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			match ERROR::from(
				((*(*ppv)).HrInit)(ppv),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}

	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) -> Result<(), ERROR> {
		unsafe {
			let ppv = self.IUnknown.ppv::<ITaskbarListVtbl>();
			match ERROR::from(
				((*(*ppv)).SetActiveAlt)(ppv, hwnd.as_ptr()),
			) {
				ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}