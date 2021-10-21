#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::ComPtr;
use crate::com::shell;
use crate::com::shell::itaskbarlist::ITaskbarListT;
use crate::com::shell::itaskbarlist2::ITaskbarList2T;
use crate::com::shell::itaskbarlist3::{ITaskbarList3T, ITaskbarList3VT};
use crate::ffi::{HANDLE, HRESULT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

/// [`ITaskbarList4`](crate::shell::ITaskbarList4) virtual table.
pub struct ITaskbarList4VT {
	pub ITaskbarList3VT: ITaskbarList3VT,
	pub SetTabProperties: fn(ComPtr, HANDLE, u32) -> HRESULT,
}

/// [`ITaskbarList4`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist4)
/// COM interface over [`ITaskbarList4VT`](crate::shell::vt::ITaskbarList4VT).
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
/// let obj = CoCreateInstance::<shell::ITaskbarList4>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList4(ComPtr);

impl_iunknown!(ITaskbarList4, 0xc43dc798, 0x95d1, 0x4bea, 0x9030, 0xbb99e2983a1a);
impl ITaskbarListT for ITaskbarList4 {}
impl ITaskbarList2T for ITaskbarList4 {}
impl ITaskbarList3T for ITaskbarList4 {}
impl ITaskbarList4T for ITaskbarList4 {}

/// Exposes the [`ITaskbarList4`](crate::shell::ITaskbarList4) methods.
pub trait ITaskbarList4T: ITaskbarList3T {
	/// [`ITaskbarList4::SetTabProperties`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist4-settabproperties)
	/// method.
	fn SetTabProperties(&self,
		hwnd_tab: HWND, stp_flags: shell::co::STPFLAG) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList4VT);
			hr_to_winresult(
				(vt.SetTabProperties)(self.ptr(), hwnd_tab.ptr, stp_flags.0),
			)
		}
	}
}
