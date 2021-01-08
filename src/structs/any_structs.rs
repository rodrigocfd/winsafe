//! Assorted Win32 structs.

#![allow(non_snake_case)]

use std::ffi::c_void;
use std::marker::PhantomData;

use crate::aliases::WNDPROC;
use crate::co;
use crate::enums::IdStr;
use crate::funcs_priv::LF_FACESIZE;
use crate::funcs::{IsWindowsVistaOrGreater, HIDWORD, HIWORD, LOBYTE, LODWORD, LOWORD};
use crate::handles as h;
use crate::WString;

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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ATOM(pub(crate) u16);

impl ATOM {
	/// Useful to pass the atom as class name.
	pub fn as_ptr(self) -> *const u16 {
		self.0 as *const u16
	}
}

/// [`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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

/// [`COLORREF`](https://docs.microsoft.com/en-us/windows/win32/gdi/colorref)
/// struct.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct COLORREF(pub(crate) u32);

impl From<co::CLR> for COLORREF {
	fn from(v: co::CLR) -> Self {
		Self(v.0)
	}
}

impl COLORREF {
	/// [`GetRValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getrvalue)
	/// method. Retrieves the red intensity.
	pub fn GetRValue(self) -> u8 {
		LOBYTE(LOWORD(self.0))
	}

	/// [`GetGValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getgvalue)
	/// method. Retrieves the green intensity.
	pub fn GetGValue(self) -> u8 {
		LOBYTE(LOWORD(self.0) >> 8)
	}

	/// [`GetBValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbvalue)
	/// method. Retrieves the blue intensity.
	pub fn GetBValue(self) -> u8 {
		LOBYTE(LOWORD(self.0 >> 16))
	}
}

/// [`CREATESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct CREATESTRUCT<'a, 'b> {
	pub lpCreateParams: isize,
	pub hInstance: h::HINSTANCE,
	pub hMenu: h::HMENU,
	pub hwndParent: h::HWND,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: co::WS,
	lpszName: *const u16,
	lpszClass: *const u16,
	pub dwExStyle: co::WS_EX,
	m_lpszName: PhantomData<&'a u16>,
	m_lpszClass: PhantomData<&'b u16>,
}

impl_default_zero!(CREATESTRUCT, 'a, 'b);

impl<'a, 'b> CREATESTRUCT<'a, 'b> {
	/// Returns the `lpszName` field.
	pub fn lpszName(&self) -> String {
		WString::from_wchars_nullt(self.lpszName).to_string()
	}

	/// Sets the `lpszName` field.
	pub fn set_lpszName(&mut self, buf: &'a WString) {
		self.lpszName = unsafe { buf.as_ptr() };
	}

	/// Returns the `lpszClass` field.
	pub fn lpszClass(&self) -> String {
		WString::from_wchars_nullt(self.lpszClass).to_string()
	}

	/// Sets the `lpszClass` field.
	pub fn set_lpszClass(&mut self, buf: &'b WString) {
		self.lpszClass = unsafe { buf.as_ptr() };
	}
}

/// [`LOGFONT`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
pub struct MSG {
	pub hwnd: h::HWND,
	pub message: co::WM,
	pub wParam: usize,
	pub lParam: isize,
	pub time: u32,
	pub pt: POINT,
	lPrivate: u32,
}

impl_default_zero!(MSG);

/// [`NMHDR`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: h::HWND,
	/// ID of the control sending the message.
	pub idFrom: usize,
	/// Notification code sent in
	/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: co::NM,
}

impl_default_zero!(NMHDR);

/// [`NONCLIENTMETRICS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NONCLIENTMETRICS {
	cbSize: u32,
	pub iBorderWidth: i32,
	pub iScrollWidth: i32,
	pub iScrollHeight: i32,
	pub iCaptionWidth: i32,
	pub iCaptionHeight: i32,
	pub lfCaptionFont: LOGFONT,
	pub iSmCaptionWidth: i32,
	pub iSmCaptionHeight: i32,
	pub lfSmCaptionFont: LOGFONT,
	pub iMenuWidth: i32,
	pub iMenuHeight: i32,
	pub lfMenuFont: LOGFONT,
	pub lfStatusFont: LOGFONT,
	pub lfMessageFont: LOGFONT,
	pub iPaddedBorderWidth: i32,
}

impl Default for NONCLIENTMETRICS {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		if !IsWindowsVistaOrGreater().unwrap() {
			obj.cbSize -= std::mem::size_of::<i32>() as u32
		}
		obj
	}
}

/// [`OSVERSIONINFOEX`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
pub struct PAINTSTRUCT {
	pub hdc: h::HDC,
	pub fErase: u32,
	pub rcPaint: RECT,
	fRestore: u32,
	fIncUpdate: u32,
	rgbReserved: [u8; 32],
}

impl_default_zero!(PAINTSTRUCT);

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
#[derive(Clone, Eq, PartialEq)]
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

/// [`STYLESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-stylestruct)
/// struct for [`WS`](crate::co::WS).
#[repr(C)]
#[derive(Default, Eq, PartialEq)]
pub struct STYLESTRUCT_WS {
	pub styleOld: co::WS,
	pub styleNew: co::WS,
}

/// [`STYLESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-stylestruct)
/// struct for [`WS_EX`](crate::co::WS_EX).
#[repr(C)]
#[derive(Default, Eq, PartialEq)]
pub struct STYLESTRUCT_WS_EX {
	pub styleOld: co::WS_EX,
	pub styleNew: co::WS_EX,
}

/// [`WINDOWINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowinfo)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone)]
pub struct WNDCLASSEX<'a, 'b> {
	cbSize: u32,
	pub style: co::CS,
	pub lpfnWndProc: Option<WNDPROC>,
	pub cbClsExtra: i32,
	pub cbWndExtra: i32,
	pub hInstance: h::HINSTANCE,
	pub hIcon: h::HICON,
	pub hCursor: h::HCURSOR,
	pub hbrBackground: h::HBRUSH,
	lpszMenuName: *const u16,
	lpszClassName: *const u16,
	pub hIconSm: h::HICON,
	_markerMenuName: PhantomData<&'a u16>,
	_markerClassName: PhantomData<&'b u16>,
}

impl<'a, 'b> Default for WNDCLASSEX<'a, 'b> {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

impl<'a, 'b> WNDCLASSEX<'a, 'b> {
	/// Returns the `lpszMenuName` field.
	pub fn lpszMenuName(&self) -> IdStr {
		if HIDWORD(self.lpszMenuName as u64) == 0
			&& HIWORD(LODWORD(self.lpszMenuName as u64)) == 0 // https://stackoverflow.com/a/9806654/6923555
		{
			IdStr::Id(LOWORD(LODWORD(self.lpszMenuName as u64)) as i32)
		} else {
			IdStr::Str(WString::from_wchars_nullt(self.lpszMenuName))
		}
	}

	/// Sets the `lpszMenuName` field.
	pub fn set_lpszMenuName(&mut self, menu_name: &'a IdStr) {
		self.lpszMenuName = menu_name.as_ptr();
	}

	/// Returns the `lpszClassName` field.
	pub fn lpszClassName(&self) -> String {
		WString::from_wchars_nullt(self.lpszClassName).to_string()
	}

	/// Sets the `lpszClassName` field.
	pub fn set_lpszClassName(&mut self, buf: &'b WString) {
		self.lpszClassName = unsafe { buf.as_ptr() };
	}
}
