#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::funcs::hr_to_winresult;
use crate::com::shell::vt::ITaskbarListVT;
use crate::handles::HWND;

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// COM interface. Backed by [`ITaskbarListVT`](crate::shell::ITaskbarListVT)
/// virtual table.
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
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj: shell::ITaskbarList = CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct ITaskbarList {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<ITaskbarListVT>> for ITaskbarList {
	fn from(ppv: PPComVT<ITaskbarListVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl ITaskbarList {
	unsafe fn ppv(&self) -> PPComVT<ITaskbarListVT> {
		self.IUnknown.ppv::<ITaskbarListVT>()
	}

	/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	pub fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).ActivateTab)(self.ppv(), hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	pub fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).AddTab)(self.ppv(), hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	pub fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).DeleteTab)(self.ppv(), hwnd.ptr) )
		}
	}

	/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	pub fn HrInit(&self) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).HrInit)(self.ppv()) )
		}
	}

	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).SetActiveAlt)(self.ppv(), hwnd.ptr) )
		}
	}
}
