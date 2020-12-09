#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{AtomStr, IdMenu};
use crate::{HDC, HINSTANCE};
use crate::{PAINTSTRUCT, RECT};
use crate::co;
use crate::ffi::user32;
use crate::Utf16;

handle_type! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	HWND
}

impl HWND {
	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// method.
	pub fn BeginPaint(&self, lpPaint: &mut PAINTSTRUCT) -> Result<HDC, ()> {
		match ptr_to_opt!(
			user32::BeginPaint(
				self.0,
				lpPaint as *mut PAINTSTRUCT as *mut c_void,
			)
		) {
			Some(p) => Ok(unsafe { HDC::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	pub fn CreateWindowEx(
		dwExStyle: co::WS_EX,
		lpClassName: AtomStr,
		lpWindowName: Option<&str>,
		dwStyle: co::WS,
		X: i32, Y: i32,
		nWidth: i32, nHeight: i32,
		hWndParent: Option<HWND>,
		hMenu: IdMenu,
		hInstance: HINSTANCE,
		lpParam: Option<*const c_void>
) -> Result<HWND, co::ERROR> {
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

	/// [`DefWindowProc`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	pub fn DefWindowProc(
		&self, Msg: co::WM, wParam: usize, lParam: isize) -> isize
	{
		unsafe { user32::DefWindowProcW(self.0, Msg.into(), wParam, lParam) }
	}

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	pub fn DestroyWindow(&self) {
		unsafe { user32::DestroyWindow(self.0); }
	}

	/// [`EnableWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	pub fn EnableWindow(&self, bEnable: bool) -> bool {
		unsafe { user32::EnableWindow(self.0, bEnable as u32) != 0 }
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	pub fn EndPaint(&self, lpPaint: &PAINTSTRUCT) {
		unsafe {
			user32::EndPaint(
				self.0,
				lpPaint as *const PAINTSTRUCT as *const c_void,
			);
		}
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		lpClassName: &str, lpWindowName: &str) -> Result<HWND, co::ERROR>
	{
		match ptr_to_opt!(
			user32::FindWindowW(
				Utf16::from_str(lpClassName).as_ptr(),
				Utf16::from_str(lpWindowName).as_ptr(),
			)
		 ) {
			Some(p) => Ok(Self(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}

	/// [`GetAncestor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	pub fn GetAncestor(&self, gaFlags: co::GA) -> Option<HWND> {
		ptr_to_opt!(user32::GetAncestor(self.0, gaFlags.into()))
			.map(|p| Self(p))
	}

	/// [`GetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	pub fn GetFocus() -> Option<HWND> {
		ptr_to_opt!(user32::GetFocus())
			.map(|p| Self(p))
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

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	pub fn GetWindowLongPtr(&self, nIndex: co::GWLP) -> *const c_void {
		unsafe { user32::GetWindowLongPtrW(self.0, nIndex.into()) }
	}

	/// [`InvalidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// method.
	pub fn InvalidateRect(
		&self, lpRect: Option<&RECT>, bErase: bool) -> Result<(), ()>
	{
		match unsafe {
			user32::InvalidateRect(
				self.0,
				lpRect.map_or(
					std::ptr::null(),
					|lpRect| lpRect as *const RECT as *const c_void,
				),
				bErase as u32,
			)
		} {
			0 => Err(()),
			_ => Ok(()),
		}
	}

	/// [`IsWindowEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// method.
	pub fn IsWindowEnabled(&self) -> bool {
		unsafe { user32::IsWindowEnabled(self.0) != 0 }
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	pub fn MessageBox(&self, lpText: &str, lpCaption: &str,
		uType: co::MB) -> Result<co::DLGID, co::ERROR>
	{
		match unsafe {
			user32::MessageBoxW(
				self.0,
				Utf16::from_str(lpText).as_ptr(),
				Utf16::from_str(lpCaption).as_ptr(),
				uType.into(),
			)
		} {
			0 => Err(co::ERROR::GetLastError()),
			ret => Ok(co::DLGID::from(ret)),
		}
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	pub fn SetWindowLongPtr(&self, nIndex: co::GWLP, dwNewLong: isize) -> isize {
		unsafe { user32::SetWindowLongPtrW(self.0, nIndex.into(), dwNewLong) }
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	pub fn SetWindowText(&self, lpString: &str) -> Result<(), co::ERROR> {
		let text16 = Utf16::from_str(lpString);

		match unsafe { user32::SetWindowTextW(self.0, text16.as_ptr()) } {
			0 => Err(co::ERROR::GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(&self, nCmdShow: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.0, nCmdShow.into()) != 0 }
	}

	/// [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	pub fn UpdateWindow(&self) -> Result<(), ()> {
		match unsafe { user32::UpdateWindow(self.0) } {
			0 => Err(()),
			_ => Ok(()),
		}
	}
}