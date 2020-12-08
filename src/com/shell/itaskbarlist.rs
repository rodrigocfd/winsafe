#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ComVtbl, HWND, IID, IUnknown, IUnknownVtbl};
use crate::co::ERROR;

type PPVtbl = *const *const ITaskbarListVtbl;

/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,
	HrInit: fn(PPVtbl) -> u32,
	AddTab: fn(PPVtbl, *const c_void) -> u32,
	DeleteTab: fn(PPVtbl, *const c_void) -> u32,
	ActivateTab: fn(PPVtbl, *const c_void) -> u32,
	SetActiveAlt: fn(PPVtbl, *const c_void) -> u32,
}

impl ComVtbl for ITaskbarListVtbl {
	fn IID() -> IID {
		IID::new(0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090)
	}
}

//------------------------------------------------------------------------------

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// COM interface.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Usually instantiated with [`CoCreateInstance`](crate::CoCreateInstance):
/// ```rust,ignore
/// let mut obj: w::shell::ITaskbarList = w::CoCreateInstance(
///   &w::shell::clsid::TaskbarList,
///   None,
///   w::co::CLSCTX::INPROC_SERVER,
/// );
/// ```
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
pub struct ITaskbarList {
	/// Methods of base interface
	/// [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPVtbl> for ITaskbarList {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as *const *const IUnknownVtbl)
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