#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{user32, Void};
use crate::{AtomOrStr, HINSTANCE, HMENU};
use crate::Utf16;

handle_type!(HWND,
	"Handle to a
	[window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).");

impl Default for HWND {
	fn default() -> Self {
		Self(std::ptr::null())
	}
}

impl HWND {
	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// function.
	pub fn CreateWindowEx(
		dwExStyle: co::WS_EX, lpClassName: AtomOrStr, lpWindowName: Option<&str>,
		dwStyle: co::WS, X: i32, Y: i32, nWidth: i32, nHeight: i32,
		hWndParent: Option<HWND>, hMenu: HMENU, hInstance: HINSTANCE,
		lpParam: Option<*const Void>) -> Result<HWND, co::ERROR>
	{
		let mut classNameBuf16 = Utf16::default();

		match ptr_to_opt!(
			user32::CreateWindowExW(
				dwExStyle.into(),
				lpClassName.MAKEINTRESOURCE(&mut classNameBuf16),
				Utf16::from_opt_str(lpWindowName).as_ptr(),
				dwStyle.into(),
				X, Y, nWidth, nHeight,
				hWndParent.unwrap_or_default().as_ptr(),
				hMenu.as_ptr(),
				hInstance.as_ptr(),
				lpParam.unwrap_or(std::ptr::null())
			)
		) {
			Some(p) => Ok(Self(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// function.
	pub fn GetForegroundWindow() -> Option<HWND> {
		ptr_to_opt!(user32::GetForegroundWindow())
			.map(|p| Self(p))
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	pub fn GetParent(&self) -> Result<HWND, co::ERROR> {
		match ptr_to_opt!(user32::GetParent(self.0)) {
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
		match ptr_to_opt!(user32::GetWindow(self.0, uCmd.into())) {
			Some(p) => Ok(Self(p)),
			None => match co::ERROR::GetLastError() {
				co::ERROR::SUCCESS => Ok(Self::default()),
				err => Err(err),
			},
		}
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	pub fn MessageBox(&self, lpText: &str, lpCaption: &str,
		uType: co::MB) -> Result<co::DLGID, co::ERROR>
	{
		match unsafe {
			user32::MessageBoxW(self.0, Utf16::from_str(lpText).as_ptr(),
				Utf16::from_str(lpCaption).as_ptr(), uType.into())
		} {
			0 => Err(co::ERROR::GetLastError()),
			ret => Ok(co::DLGID::from(ret)),
		}
	}
}