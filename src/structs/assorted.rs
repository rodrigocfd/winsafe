//! Assorted Win32 structs.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{HBITMAP, HBRUSH, HCURSOR, HDC, HICON, HINSTANCE, HMENU, HWND};
use crate::co;
use crate::structs::const_vals;

/// Type alias to callback function.
///
/// Used in:
/// * [`WNDCLASSEX`](crate::WNDCLASSEX) `lpfnWndProc`.
pub type WNDPROC =
	unsafe extern "system" fn(
		hWnd: HWND, uMsg: co::WM, wParam: usize, lParam: isize,
	) -> isize;

//------------------------------------------------------------------------------

/// [`ACCEL`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-accel)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct ACCEL {
	pub fVirt: co::ACCELF,
	pub key: co::VK,
	pub cmd: u16,
}

/// [`ATOM`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#atom)
/// returned by
/// [`RegisterClassEx`](crate::RegisterClassEx).
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATOM(u16);

impl From<u16> for ATOM {
	fn from(n: u16) -> Self {
		Self(n)
	}
}

impl ATOM {
	/// Useful to pass the atom as class name.
	pub fn as_ptr(self) -> *const u16 {
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
		Self {
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
	pub lfFaceName: [u16; const_vals::LF_FACESIZE],
}

/// [`MENUINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuinfo)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MENUINFO {
	cbSize: u32,
	pub fMask: co::MIM,
	pub dwStyle: co::MNS,
	pub cyMax: u32,
	pub hbrBack: HBRUSH,
	pub dwContextHelpID: u32,
	pub dwMenuData: usize,
}

impl Default for MENUINFO {
	fn default() -> Self {
		Self {
			cbSize: std::mem::size_of::<Self>() as u32,
			..Default::default()
		}
	}
}

/// [`MENUITEMINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuiteminfow)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MENUITEMINFO {
	cbSize: u32,
	pub fMask: co::MIIM,
	pub fType: co::MFT,
	pub fState: co::MFS,
	pub wID: u32,
	pub hSubMenu: HMENU,
	pub hbmpChecked: HBITMAP,
	pub hbmpUnchecked: HBITMAP,
	pub dwItemData: usize,
	pub dwTypeData: *mut u16,
	pub cch: u32,
	pub hbmpItem: HBITMAP,
}

impl Default for MENUITEMINFO {
	fn default() -> Self {
		Self {
			cbSize: std::mem::size_of::<Self>() as u32,
			..Default::default()
		}
	}
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
	/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: co::NM,
}

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
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

/// [`WINDOWINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowinfo)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WINDOWINFO {
	cbSize: u32,
	pub rcWindow: RECT,
	pub rcClient: RECT,
	pub dwStyle: co::WS,
	pub dwExStyle: co::WS_EX,
	pub dwWindowStatus: u32,
	pub cxWindowBorders: u32,
	pub cyWindowBorders: u32,
	pub atomWindowType: ATOM,
	pub wCreatorVersion: u16,
}

impl Default for WINDOWINFO {
	fn default() -> Self {
		Self {
			cbSize: std::mem::size_of::<Self>() as u32,
			..Default::default()
		}
	}
}

/// [`WINDOWPLACEMENT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WINDOWPLACEMENT {
	length: u32,
	pub flags: co::WPF,
	pub showCmd: co::SW,
	pub ptMinPosition: POINT,
	pub ptMaxPosition: POINT,
	pub rcNormalPosition: RECT,
	pub rcDevice: RECT,
}

impl Default for WINDOWPLACEMENT {
	fn default() -> Self {
		Self {
			length: std::mem::size_of::<Self>() as u32,
			..Default::default()
		}
	}
}

/// [`WNDCLASSEX`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WNDCLASSEX {
	cbSize: u32,
	pub style: co::CS,
	pub lpfnWndProc: Option<WNDPROC>,
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
		Self {
			cbSize: std::mem::size_of::<Self>() as u32,
			lpszMenuName: std::ptr::null(),
			lpszClassName: std::ptr::null(),
			..Default::default()
		}
	}
}