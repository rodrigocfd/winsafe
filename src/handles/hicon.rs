#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;

/// Handle to an
/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HICON(pub(crate) *mut std::ffi::c_void);

impl_handle!(HICON);

impl HICON {
	/// [`CopyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-copyicon)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HICON::DestroyIcon`](crate::HICON::DestroyIcon) call.
	pub fn CopyIcon(self) -> WinResult<HICON> {
		unsafe { user32::CopyIcon(self.0).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	pub fn DestroyIcon(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DestroyIcon(self.0) })
	}
}
