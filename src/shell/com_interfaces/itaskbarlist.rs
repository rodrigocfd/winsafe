#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HANDLE, HRES};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{Handle, ole_IUnknown};
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`ITaskbarList`](crate::ITaskbarList) virtual table.
#[repr(C)]
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(COMPTR) -> HRES,
	pub AddTab: fn(COMPTR, HANDLE) -> HRES,
	pub DeleteTab: fn(COMPTR, HANDLE) -> HRES,
	pub ActivateTab: fn(COMPTR, HANDLE) -> HRES,
	pub SetActiveAlt: fn(COMPTR, HANDLE) -> HRES,
}

com_interface! { ITaskbarList: "56fdf342-fd6d-11d0-958a-006097c9a090";
	/// [`ITaskbarList`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
	/// COM interface over [`ITaskbarListVT`](crate::vt::ITaskbarListVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, ITaskbarList};
	///
	/// let obj = CoCreateInstance::<ITaskbarList>(
	///     &co::CLSID::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList`](crate::ITaskbarList).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList: ole_IUnknown {
	/// [`ITaskbarList::ActivateTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	fn ActivateTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).ActivateTab)(self.ptr(), hwnd.as_ptr())
			},
		)
	}

	/// [`ITaskbarList::AddTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	fn AddTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).AddTab)(self.ptr(), hwnd.as_ptr())
			},
		)
	}

	/// [`ITaskbarList::DeleteTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	fn DeleteTab(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).DeleteTab)(self.ptr(), hwnd.as_ptr())
			},
		)
	}

	/// [`ITaskbarList::HrInit`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	fn HrInit(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<ITaskbarListVT>(self).HrInit)(self.ptr()) })
	}

	/// [`ITaskbarList::SetActiveAlt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	fn SetActiveAlt(&self, hwnd: &HWND) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarListVT>(self).SetActiveAlt)(self.ptr(), hwnd.as_ptr())
			},
		)
	}
}
