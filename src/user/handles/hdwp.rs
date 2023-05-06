#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::Handle;
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};
use crate::user::guard::EndDeferWindowPosGuard;

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
	fn BeginDeferWindowPos(
		num_windows: u32) -> SysResult<EndDeferWindowPosGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				user::ffi::BeginDeferWindowPos(num_windows as _),
			).map(|h| EndDeferWindowPosGuard::new(h))
		}
	}

	/// [`DeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn DeferWindowPos(&mut self,
		hwnd: &HWND,
		hwnd_insert_after: HwndPlace,
		top_left: POINT,
		sz: SIZE,
		flags: co::SWP,
	) -> SysResult<()>
	{
		match unsafe {
			user::ffi::DeferWindowPos(
				self.as_ptr(),
				hwnd.as_ptr(),
				hwnd_insert_after.as_ptr(),
				top_left.x, top_left.y, sz.cx, sz.cy,
				flags.raw(),
			).as_mut()
		} {
			Some(ptr) => {
				*self = unsafe { Self::from_ptr(ptr) };
				Ok(())
			},
			None => {
				*self = Self::INVALID; // prevent EndDeferWindowPos()
				Err(GetLastError())
			},
		}
	}
}
