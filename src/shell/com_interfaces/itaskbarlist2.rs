#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, COMPTR, HANDLE, HRES};
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::prelude::{Handle, shell_ITaskbarList};
use crate::user::decl::HWND;
use crate::vt::ITaskbarListVT;

/// [`ITaskbarList2`](crate::ITaskbarList2) virtual table.
#[repr(C)]
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,
	pub MarkFullscreenWindow: fn(COMPTR, HANDLE, BOOL) -> HRES,
}

com_interface! { ITaskbarList2: "602d4995-b13a-429b-a66e-1935e44f4317";
	/// [`ITaskbarList2`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
	/// COM interface over [`ITaskbarList2VT`](crate::vt::ITaskbarList2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
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
}

impl shell_ITaskbarList for ITaskbarList2 {}
impl shell_ITaskbarList2 for ITaskbarList2 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ITaskbarList2`](crate::ITaskbarList2).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_ITaskbarList2: shell_ITaskbarList {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	fn MarkFullscreenWindow(&self,
		hwnd: &HWND, full_screen: bool) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<ITaskbarList2VT>(self).MarkFullscreenWindow)(
					self.ptr(),
					hwnd.as_ptr(),
					full_screen as _,
				)
			},
		)
	}
}
