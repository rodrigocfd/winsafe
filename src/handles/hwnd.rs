#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{user32, Void};
use crate::Utf16;

/// Handle to a
/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWND(*const Void);

impl Default for HWND {
	fn default() -> Self {
		Self(std::ptr::null_mut())
	}
}

impl HWND {
	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// function.
	pub fn GetForegroundWindow() -> Option<HWND> {
		ptr_to_opt!(unsafe { user32::GetForegroundWindow() })
			.map(|p| Self(p))
	}

	/// Returns the raw underlying pointer.
	pub fn as_ptr(&self) -> *const Void {
		self.0
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	pub fn GetParent(&self) -> Result<HWND, co::ERROR> {
		match ptr_to_opt!(unsafe { user32::GetParent(self.0) }) {
			Some(p) => Ok(Self(p)),
			None => match co::ERROR::GetLastError() {
				co::ERROR::SUCCESS => Ok(Self::default()),
				err => Err(err),
			},
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	pub fn GetWindow(&self, uCmd: co::GW) -> Result<HWND, co::ERROR> {
		match ptr_to_opt!(unsafe { user32::GetWindow(self.0, uCmd.into()) }) {
			Some(p) => Ok(Self(p)),
			None => match co::ERROR::GetLastError() {
				co::ERROR::SUCCESS => Ok(Self::default()),
				err => Err(err),
			},
		}
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	pub fn MessageBox(&self, lpText: &str, lpCaption: &str, uType: co::MB) -> co::DLGID {
		co::DLGID::from(
			unsafe {
				user32::MessageBoxW(self.0, Utf16::from_str(lpText).as_ptr(),
					Utf16::from_str(lpCaption).as_ptr(), uType.into())
			}
		)
	}
}