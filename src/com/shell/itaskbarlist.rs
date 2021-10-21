#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HANDLE, HRESULT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(ComPtr) -> HRESULT,
	pub AddTab: fn(ComPtr, HANDLE) -> HRESULT,
	pub DeleteTab: fn(ComPtr, HANDLE) -> HRESULT,
	pub ActivateTab: fn(ComPtr, HANDLE) -> HRESULT,
	pub SetActiveAlt: fn(ComPtr, HANDLE) -> HRESULT,
}

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// COM interface over [`ITaskbarListVT`](crate::shell::vt::ITaskbarListVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::ITaskbarList>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList(ComPtr);

impl_iunknown!(ITaskbarList, 0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090);
impl ITaskbarListT for ITaskbarList {}

/// Exposes the [`ITaskbarList`](crate::shell::ITaskbarList) methods.
pub trait ITaskbarListT: IUnknownT {
	/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	fn ActivateTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarListVT);
			hr_to_winresult((vt.ActivateTab)(self.ptr(), hwnd.ptr))
		}
	}

	/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	fn AddTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarListVT);
			hr_to_winresult((vt.AddTab)(self.ptr(), hwnd.ptr))
		}
	}

	/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	fn DeleteTab(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarListVT);
			hr_to_winresult((vt.DeleteTab)(self.ptr(), hwnd.ptr))
		}
	}

	/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	fn HrInit(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarListVT);
			hr_to_winresult((vt.HrInit)(self.ptr()))
		}
	}

	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	fn SetActiveAlt(&self, hwnd: HWND) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarListVT);
			hr_to_winresult((vt.SetActiveAlt)(self.ptr(), hwnd.ptr))
		}
	}
}
