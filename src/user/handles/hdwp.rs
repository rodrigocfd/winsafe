#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};

impl_handle! { HDWP: "user32";
	/// Handle to a
	/// [deferred window position](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp).
}

impl user_Hdwp for HDWP {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HDWP`](crate::HDWP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Hdwp: Handle {
	/// [`BeginDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-begindeferwindowpos)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HDWP::EndDeferWindowPos`](crate::prelude::user_Hdwp::EndDeferWindowPos)
	/// call.
	#[must_use]
	fn BeginDeferWindowPos(num_windows: u32) -> WinResult<HDWP> {
		unsafe { user::ffi::BeginDeferWindowPos(num_windows as _).as_mut() }
			.map(|ptr| HDWP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// method.
	fn DeferWindowPos(self,
		hwnd: HWND, hwnd_insert_after: HwndPlace,
		top_left: POINT, sz: SIZE, flags: co::SWP) -> WinResult<HDWP>
	{
		unsafe {
			user::ffi::DeferWindowPos(
				self.as_ptr(),
				hwnd.0,
				hwnd_insert_after.as_ptr(),
				top_left.x, top_left.y, sz.cx, sz.cy,
				flags.0,
			).as_mut()
		}.map(|ptr| HDWP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`EndDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// method.
	fn EndDeferWindowPos(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::EndDeferWindowPos(self.as_ptr()) })
	}
}
