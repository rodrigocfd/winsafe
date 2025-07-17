#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { ITaskbarList: "56fdf342-fd6d-11d0-958a-006097c9a090";
	/// [`ITaskbarList`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let obj = w::CoCreateInstance::<w::ITaskbarList>(
	///     &co::CLSID::TaskbarList,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList`](crate::ITaskbarList).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList: ole_IUnknown {
	/// [`ITaskbarList::ActivateTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-activatetab)
	/// method.
	fn ActivateTab(&self, hwnd: &HWND) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskbarListVT>(self).ActivateTab)(self.ptr(), hwnd.ptr()) })
			.to_hrresult()
	}

	/// [`ITaskbarList::AddTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-addtab)
	/// method.
	fn AddTab(&self, hwnd: &HWND) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskbarListVT>(self).AddTab)(self.ptr(), hwnd.ptr()) }).to_hrresult()
	}

	/// [`ITaskbarList::DeleteTab`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-deletetab)
	/// method.
	fn DeleteTab(&self, hwnd: &HWND) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskbarListVT>(self).DeleteTab)(self.ptr(), hwnd.ptr()) })
			.to_hrresult()
	}

	fn_com_noparm! { HrInit: ITaskbarListVT;
		/// [`ITaskbarList::HrInit`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-hrinit)
		/// method.
	}

	/// [`ITaskbarList::SetActiveAlt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	fn SetActiveAlt(&self, hwnd: &HWND) -> HrResult<()> {
		HrRet(unsafe { (vt::<ITaskbarListVT>(self).SetActiveAlt)(self.ptr(), hwnd.ptr()) })
			.to_hrresult()
	}
}
