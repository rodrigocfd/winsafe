#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::privs::bool_to_winresult;
use crate::structs::{POINT, SIZE};

/// Handle to a
/// [deferred window position](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdwp).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HDWP(pub(crate) *mut std::ffi::c_void);

impl_handle!(HDWP);

impl HDWP {
	/// [`BeginDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-begindeferwindowpos)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HDWP::EndDeferWindowPos`](crate::HDWP::EndDeferWindowPos) call.
	pub fn BeginDeferWindowPos(num_windows: u32) -> WinResult<HDWP> {
		unsafe { user32::BeginDeferWindowPos(num_windows as _).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deferwindowpos)
	/// method.
	pub fn DeferWindowPos(self,
		hwnd: HWND, hwnd_insert_after: HwndPlace,
		top_left: POINT, sz: SIZE, flags: co::SWP) -> WinResult<HDWP>
	{
		unsafe {
			user32::DeferWindowPos(
				self.0,
				hwnd.0,
				hwnd_insert_after.as_ptr(),
				top_left.x, top_left.y, sz.cx, sz.cy,
				flags.0,
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`EndDeferWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddeferwindowpos)
	/// method.
	pub fn EndDeferWindowPos(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::EndDeferWindowPos(self.0) })
	}
}
