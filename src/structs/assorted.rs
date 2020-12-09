//! Assorted Win32 structs.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{HBRUSH, HCURSOR, HDC, HICON, HINSTANCE, HMENU, HWND};
use crate::co;
use crate::structs::consts;

/// [`ATOM`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#atom)
/// returned by
/// [`RegisterClassEx`](crate::RegisterClassEx).
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATOM(u16);

impl From<u16> for ATOM {
	fn from(n: u16) -> ATOM {
		ATOM(n)
	}
}

impl ATOM {
	/// Useful to pass the atom as class name.
	pub fn as_ptr(&self) -> *const u16 {
		self.0 as *const u16
	}
}

/// [`CREATESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CREATESTRUCT {
	pub lpCreateParams: *const c_void,
	pub hInstance: HINSTANCE,
	pub hMenu: HMENU,
	pub hwndParent: HWND,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: co::WS,
	pub lpszName: *const u16,
	pub lpszClass: *const u16,
	pub dwExStyle: co::WS_EX,
}

impl Default for CREATESTRUCT {
	fn default() -> Self {
		CREATESTRUCT {
			lpCreateParams: std::ptr::null(),
			lpszName: std::ptr::null(),
			lpszClass: std::ptr::null(),
			..Default::default()
		}
	}
}

/// [`LOGFONT`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct LOGFONT {
	pub lfHeight: i32,
	pub lfWidth: i32,
	pub lfEscapement: i32,
	pub lfOrientation: i32,
	pub lfWeight: co::FW,
	pub lfItalic: u8,
	pub lfUnderline: u8,
	pub lfStrikeOut: u8,
	pub lfCharSet: co::CHARSET,
	pub lfOutPrecision: co::OUT_PRECIS,
	pub lfClipPrecision: co::CLIP,
	pub lfQuality: co::QUALITY,
	pub lfPitchAndFamily: co::PITCH,
	pub lfFaceName: [u16; consts::LF_FACESIZE],
}

/// [`MSG`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct MSG {
	hwnd: HWND,
	message: co::WM,
	wParam: usize,
	lParam: isize,
	time: u32,
	pt: POINT,
	lPrivate: u32,
}

/// [`NMHDR`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: HWND,
	/// ID of the control sending the message.
	pub idFrom: usize,
	/// Notification code sent in
	/// [WM_NOTIFY](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: co::NM,
}

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PAINTSTRUCT {
	pub hdc: HDC,
	pub fErase: u32,
	pub rcPaint: RECT,
	pub fRestore: u32,
	pub fIncUpdate: u32,
	pub rgbReserved: [u8; 32],
}

/// [`POINT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct POINT {
	pub x: i32,
	pub y: i32,
}

/// [`RECT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct RECT {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

/// [`SIZE`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-size)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct SIZE {
	pub cx: i32,
	pub cy: i32,
}

/// [`WNDCLASSEX`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WNDCLASSEX {
	cbSize: u32,
	pub style: co::CS,
	pub lpfnWndProc: Option<
		unsafe extern "system" fn(
			hWnd: HWND, uMsg: co::WM, wParam: usize, lParam: isize,
		) -> isize,
	>,
	pub cbClsExtra: i32,
	pub cbWndExtra: i32,
	pub hInstance: HINSTANCE,
	pub hIcon: HICON,
	pub hCursor: HCURSOR,
	pub hbrBackground: HBRUSH,
	pub lpszMenuName: *const u16,
	pub lpszClassName: *const u16,
	pub hIconSm: HICON,
}

impl Default for WNDCLASSEX {
	fn default() -> Self {
		WNDCLASSEX {
			cbSize: std::mem::size_of::<WNDCLASSEX>() as u32,
			lpszMenuName: std::ptr::null(),
			lpszClassName: std::ptr::null(),
			..Default::default()
		}
	}
}