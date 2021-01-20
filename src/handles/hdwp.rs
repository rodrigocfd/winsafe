#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::privs::ptr_as_opt;

handle_type! {
	/// Handle to a
	/// [deferred window position](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp)
	/// structure. Exposes methods.
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
		X: i32, Y: i32, cx: u32, cy: u32, uFlags: co::SWP) -> WinResult<HDWP>
	{
		match ptr_as_opt(
			unsafe {
				user32::DeferWindowPos(
					self.ptr, hWnd.ptr, hWndInsertAfter.as_ptr(),
					X, Y, cx as i32, cy as i32, uFlags.into(),
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
		match unsafe { user32::EndDeferWindowPos(self.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}
