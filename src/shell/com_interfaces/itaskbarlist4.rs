#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::{HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ShellITaskbarList, ShellITaskbarList2, ShellITaskbarList3};
use crate::user::decl::HWND;
use crate::vt::ITaskbarList3VT;

/// [`ITaskbarList4`](crate::ITaskbarList4) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(ComPtr, HANDLE, u32) -> HRES,
}

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
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct ITaskbarList4(ComPtr);

impl_iunknown!(ITaskbarList4, 0xc43dc798, 0x95d1, 0x4bea, 0x9030, 0xbb99e2983a1a);
impl ShellITaskbarList for ITaskbarList4 {}
impl ShellITaskbarList2 for ITaskbarList4 {}
impl ShellITaskbarList3 for ITaskbarList4 {}
impl ShellITaskbarList4 for ITaskbarList4 {}

/// [`ITaskbarList4`](crate::ITaskbarList4) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellITaskbarList4: ShellITaskbarList3 {
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
