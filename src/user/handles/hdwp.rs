#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::prelude::Handle;
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};

impl_handle! { HDWP;
	/// Handle to a
	/// [deferred window position](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp).
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
pub trait user_Hdwp: Handle {
	/// [`BeginDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-begindeferwindowpos)
	/// static method.
	#[must_use]
	fn BeginDeferWindowPos(num_windows: u32) -> SysResult<HdwpGuard> {
		unsafe { user::ffi::BeginDeferWindowPos(num_windows as _).as_mut() }
			.map(|ptr| HdwpGuard { handle: HDWP(ptr) })
			.ok_or_else(|| GetLastError())
	}

	/// [`DeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// method.
	fn DeferWindowPos(&self,
		hwnd: &HWND,
		hwnd_insert_after: HwndPlace,
		top_left: POINT,
		sz: SIZE,
		flags: co::SWP) -> SysResult<HDWP>
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
}

//------------------------------------------------------------------------------

handle_guard! { HdwpGuard: HDWP;
	user::ffi::EndDeferWindowPos;
	/// RAII implementation for [`HDWP`](crate::HDWP) which automatically calls
	/// [`EndDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// when the object goes out of scope.
}
