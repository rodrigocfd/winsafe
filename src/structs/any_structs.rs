//! Assorted Win32 structs.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::aliases::WNDPROC;
use crate::co;
use crate::handles as h;
use crate::internal_defs::LF_FACESIZE;

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

/// [`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
/// struct;
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BITMAPINFOHEADER {
	biSize: u32,
	pub biWidth: i32,
	pub biHeight: i32,
	pub biPlanes: u16,
	pub biBitCount: u16,
	pub biCompression: co::BI,
	pub biSizeImage: u32,
	pub biXPelsPerMeter: i32,
	pub biYPelsPerMeter: i32,
	pub biClrUsed: u32,
	pub biClrImportant: u32,
}

impl Default for BITMAPINFOHEADER {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.biSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

/// [`CREATESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CREATESTRUCT {
	pub lpCreateParams: *const c_void,
	pub hInstance: h::HINSTANCE,
	pub hMenu: h::HMENU,
	pub hwndParent: h::HWND,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: co::WS,
	pub lpszName: *const u16,
	pub lpszClass: *const u16,
	pub dwExStyle: co::WS_EX,
}

impl_default_zero!(CREATESTRUCT);

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
	pub lfFaceName: [u16; LF_FACESIZE],
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
	pub hbrBack: h::HBRUSH,
	pub dwContextHelpID: u32,
	pub dwMenuData: usize,
}

impl Default for MENUINFO {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
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
	pub hSubMenu: h::HMENU,
	pub hbmpChecked: h::HBITMAP,
	pub hbmpUnchecked: h::HBITMAP,
	pub dwItemData: usize,
	pub dwTypeData: *mut u16,
	pub cch: u32,
	pub hbmpItem: h::HBITMAP,
}

impl Default for MENUITEMINFO {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

/// [`MSG`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct MSG {
	pub hwnd: h::HWND,
	pub message: co::WM,
	pub wParam: usize,
	pub lParam: isize,
	pub time: u32,
	pub pt: POINT,
	lPrivate: u32,
}

/// [`NMHDR`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: h::HWND,
	/// ID of the control sending the message.
	pub idFrom: usize,
	/// Notification code sent in
	/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: co::NM,
}

/// [`OSVERSIONINFOEX`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSVERSIONINFOEX {
	dwOSVersionInfoSize: u32,
	pub dwMajorVersion: u32,
	pub dwMinorVersion: u32,
	pub dwBuildNumber: u32,
	pub dwPlatformId: co::VER_PLATFORM,
	pub szCSDVersion: [u16; 128],
	pub wServicePackMajor: u16,
	pub wServicePackMinor: u16,
	pub wSuiteMask: co::VER_SUITE,
	pub wProductType: co::VER_NT,
	wReserved: u8,
}

impl Default for OSVERSIONINFOEX {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.dwOSVersionInfoSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct PAINTSTRUCT {
	pub hdc: h::HDC,
	pub fErase: u32,
	pub rcPaint: RECT,
	fRestore: u32,
	fIncUpdate: u32,
	rgbReserved: [u8; 32],
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

/// [`SECURITY_ATTRIBUTES`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
	nLength: u32,
	pub lpSecurityDescriptor: *mut c_void,
	pub bInheritHandle: u32,
}

impl Default for SECURITY_ATTRIBUTES {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.nLength = std::mem::size_of::<Self>() as u32;
		obj
	}
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
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
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
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.length = std::mem::size_of::<Self>() as u32;
		obj
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
	pub hInstance: h::HINSTANCE,
	pub hIcon: h::HICON,
	pub hCursor: h::HCURSOR,
	pub hbrBackground: h::HBRUSH,
	pub lpszMenuName: *const u16,
	pub lpszClassName: *const u16,
	pub hIconSm: h::HICON,
}

impl Default for WNDCLASSEX {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}