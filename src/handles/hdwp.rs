#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::privs::{bool_to_winresult, ptr_as_opt};

handle_type! {
	/// Handle to a
	/// [deferred window position](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp).
	HDWP
}

impl HDWP {
	/// [`BeginDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-begindeferwindowpos)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`EndDeferWindowPos`](crate::HDWP::EndDeferWindowPos) call.
	pub fn BeginDeferWindowPos(nNumWindows: u32) -> WinResult<HDWP> {
		match ptr_as_opt(
			unsafe { user32::BeginDeferWindowPos(nNumWindows as i32) },
		) {
			None => Err(GetLastError()),
			Some(ptr) => Ok(Self { ptr }),
		}
	}

	/// [`DeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// method.
	pub fn DeferWindowPos(self,
		hWnd: HWND, hWndInsertAfter: HwndPlace,
		x: i32, y: i32, cx: i32, cy: i32, uFlags: co::SWP) -> WinResult<HDWP>
	{
		match ptr_as_opt(
			unsafe {
				user32::DeferWindowPos(
					self.ptr,
					hWnd.ptr,
					hWndInsertAfter.as_ptr(),
					x, y, cx, cy,
					uFlags.0,
				)
			},
		 ) {
			None => Err(GetLastError()),
			Some(ptr) => Ok(Self { ptr }),
		}
	}

	/// [`EndDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// method.
	pub fn EndDeferWindowPos(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::EndDeferWindowPos(self.ptr) })
	}
}
