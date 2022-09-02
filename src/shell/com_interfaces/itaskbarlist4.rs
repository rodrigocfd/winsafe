#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{
	shell_ITaskbarList, shell_ITaskbarList2, shell_ITaskbarList3,
};
use crate::user::decl::HWND;
use crate::vt::ITaskbarList3VT;

/// [`ITaskbarList4`](crate::ITaskbarList4) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(ComPtr, HANDLE, u32) -> HRES,
}

com_interface! { ITaskbarList4: "shell";
	"c43dc798-95d1-4bea-9030-bb99e2983a1a";
	/// [`ITaskbarList4`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist4)
	/// COM interface over [`ITaskbarList4VT`](crate::vt::ITaskbarList4VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, ITaskbarList4};
	///
	/// let obj = CoCreateInstance::<ITaskbarList4>(
	///     &co::CLSID::TaskbarList,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_ITaskbarList for ITaskbarList4 {}
impl shell_ITaskbarList2 for ITaskbarList4 {}
impl shell_ITaskbarList3 for ITaskbarList4 {}
impl shell_ITaskbarList4 for ITaskbarList4 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList4`](crate::ITaskbarList4).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_ITaskbarList4: shell_ITaskbarList3 {
	/// [`ITaskbarList4::SetTabProperties`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist4-settabproperties)
	/// method.
	fn SetTabProperties(&self,
		hwnd_tab: HWND, stp_flags: co::STPFLAG) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList4VT);
			ok_to_hrresult(
				(vt.SetTabProperties)(self.ptr(), hwnd_tab.0, stp_flags.0),
			)
		}
	}
}
