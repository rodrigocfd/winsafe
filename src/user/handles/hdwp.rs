#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

handle! { HDWP;
	/// Handle to a
	/// [deferred window position](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp).
}

impl HDWP {
	/// [`BeginDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-begindeferwindowpos)
	/// function.
	///
	/// In the original C implementation, `BeginDeferWindowPos` returns a handle
	/// which must be passed to
	/// [`EndDeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because
	/// `BeginDeferWindowPos` returns an
	/// [`EndDeferWindowPosGuard`](crate::guard::EndDeferWindowPosGuard), which
	/// automatically calls `EndDeferWindowPos` when the guard goes out of
	/// scope.
	#[must_use]
	pub fn BeginDeferWindowPos(num_windows: u32) -> SysResult<EndDeferWindowPosGuard> {
		unsafe {
			PtrRet(ffi::BeginDeferWindowPos(num_windows as _))
				.to_sysresult_handle()
				.map(|h| EndDeferWindowPosGuard::new(h))
		}
	}

	/// [`DeferWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// function.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	pub fn DeferWindowPos(
		&mut self,
		hwnd: &HWND,
		hwnd_insert_after: HwndPlace,
		top_left: POINT,
		sz: SIZE,
		flags: co::SWP,
	) -> SysResult<()> {
		match unsafe {
			ffi::DeferWindowPos(
				self.ptr(),
				hwnd.ptr(),
				hwnd_insert_after.as_ptr(),
				top_left.x,
				top_left.y,
				sz.cx,
				sz.cy,
				flags.raw(),
			)
			.as_mut()
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
