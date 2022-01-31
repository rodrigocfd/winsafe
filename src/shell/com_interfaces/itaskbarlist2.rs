#![allow(non_snake_case)]

use crate::ffi_types::{BOOL, HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ShellITaskbarList;
use crate::user::decl::HWND;
use crate::vt::ITaskbarListVT;

/// [`ITaskbarList2`](crate::ITaskbarList2) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,
	pub MarkFullscreenWindow: fn(ComPtr, HANDLE, BOOL) -> HRES,
}

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// COM interface over [`ITaskbarList2VT`](crate::vt::ITaskbarList2VT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, ITaskbarList2};
///
/// let obj = CoCreateInstance::<ITaskbarList2>(
///     &co::CLSID::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct ITaskbarList2(ComPtr);

impl_iunknown!(ITaskbarList2, 0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317);
impl ShellITaskbarList for ITaskbarList2 {}
impl ShellITaskbarList2 for ITaskbarList2 {}

/// [`ITaskbarList2`](crate::ITaskbarList2) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellITaskbarList2: ShellITaskbarList {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	fn MarkFullscreenWindow(&self,
		hwnd: HWND, full_screen: bool) -> HrResult<()>
	{
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITaskbarList2VT);
			ok_to_hrresult(
				(vt.MarkFullscreenWindow)(self.ptr(), hwnd.0, full_screen as _),
			)
		}
	}
}
