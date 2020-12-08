#![allow(non_snake_case)]

use std::ffi::c_void;

use crate as w;
use crate::co;
use crate::ffi::user32;

handle_type! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	HWND
}

impl HWND {
	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	pub fn CreateWindowEx(
		dwExStyle: co::WS_EX,
		lpClassName: w::AtomOrStr,
		lpWindowName: Option<&str>,
		dwStyle: co::WS,
		X: i32, Y: i32,
		nWidth: i32, nHeight: i32,
		hWndParent: Option<HWND>,
		hMenu: w::IdOrMenu,
		hInstance: w::HINSTANCE,
		lpParam: Option<*const c_void>
) -> Result<HWND, co::ERROR> {
		let mut classNameBuf16 = w::Utf16::default();

		match ptr_to_opt!(
			user32::CreateWindowExW(
				dwExStyle.into(),
				lpClassName.MAKEINTRESOURCE(&mut classNameBuf16),
				w::Utf16::from_opt_str(lpWindowName).as_ptr(),
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

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	pub fn DestroyWindow(&self) {
		unsafe { user32::DestroyWindow(self.0); }
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		lpClassName: &str, lpWindowName: &str) -> Result<HWND, co::ERROR>
	{
		match ptr_to_opt!(
			user32::FindWindowW(
				w::Utf16::from_str(lpClassName).as_ptr(),
				w::Utf16::from_str(lpWindowName).as_ptr(),
			)
		 ) {
			Some(p) => Ok(Self(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
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
			user32::MessageBoxW(
				self.0,
				w::Utf16::from_str(lpText).as_ptr(),
				w::Utf16::from_str(lpCaption).as_ptr(),
				uType.into(),
			)
		} {
			0 => Err(co::ERROR::GetLastError()),
			ret => Ok(co::DLGID::from(ret)),
		}
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(&self, nCmdShow: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.0, nCmdShow.into()) != 0 }
	}
}