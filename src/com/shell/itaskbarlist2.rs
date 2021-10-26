#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::ComPtr;
use crate::com::shell::itaskbarlist::{ITaskbarListT, ITaskbarListVT};
use crate::ffi::{BOOL, HANDLE, HRESULT};
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
#[repr(C)]
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,
	pub MarkFullscreenWindow: fn(ComPtr, HANDLE, BOOL) -> HRESULT,
}

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// COM interface over [`ITaskbarList2VT`](crate::shell::vt::ITaskbarList2VT).
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
/// let obj = CoCreateInstance::<shell::ITaskbarList2>(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct ITaskbarList2(ComPtr);

impl_iunknown!(ITaskbarList2, 0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317);
impl ITaskbarListT for ITaskbarList2 {}
impl ITaskbarList2T for ITaskbarList2 {}

/// Exposes the [`ITaskbarList2`](crate::shell::ITaskbarList2) methods.
pub trait ITaskbarList2T: ITaskbarListT {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	fn MarkFullscreenWindow(&self,
		hwnd: HWND, full_screen: bool) -> WinResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList2VT);
			hr_to_winresult(
				(vt.MarkFullscreenWindow)(self.ptr(), hwnd.ptr, full_screen as _),
			)
		}
	}
}
