#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`ITaskbarList`](crate::ITaskbarList) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct ITaskbarListVT {
	pub IUnknownVT: IUnknownVT,
	pub HrInit: fn(ComPtr) -> HRES,
	pub AddTab: fn(ComPtr, HANDLE) -> HRES,
	pub DeleteTab: fn(ComPtr, HANDLE) -> HRES,
	pub ActivateTab: fn(ComPtr, HANDLE) -> HRES,
	pub SetActiveAlt: fn(ComPtr, HANDLE) -> HRES,
}

com_interface! { ITaskbarList: "shell";
	"56fdf342-fd6d-11d0-958a-006097c9a090";
	/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
	/// COM interface over [`ITaskbarListVT`](crate::vt::ITaskbarListVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
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
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_ITaskbarList: ole_IUnknown {
	/// [`ITaskbarList::ActivateTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	fn ActivateTab(&self, hwnd: HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarListVT>();
			ok_to_hrresult((vt.ActivateTab)(self.ptr(), hwnd.0))
		}
	}

	/// [`ITaskbarList::AddTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	fn AddTab(&self, hwnd: HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarListVT>();
			ok_to_hrresult((vt.AddTab)(self.ptr(), hwnd.0))
		}
	}

	/// [`ITaskbarList::DeleteTab`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	fn DeleteTab(&self, hwnd: HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarListVT>();
			ok_to_hrresult((vt.DeleteTab)(self.ptr(), hwnd.0))
		}
	}

	/// [`ITaskbarList::HrInit`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
	/// method.
	fn HrInit(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarListVT>();
			ok_to_hrresult((vt.HrInit)(self.ptr()))
		}
	}

	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	fn SetActiveAlt(&self, hwnd: HWND) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<ITaskbarListVT>();
			ok_to_hrresult((vt.SetActiveAlt)(self.ptr(), hwnd.0))
		}
	}
}
