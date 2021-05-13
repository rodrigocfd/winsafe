//! Assorted Win32 structs.

#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::c_void;
use std::marker::PhantomData;

use crate::aliases::{CCHOOKPROC, WNDPROC};
use crate::co;
use crate::enums::{HwndHmenu, HwndPlace, IdStr};
use crate::funcs::{IsWindowsVistaOrGreater, HIDWORD, HIWORD, LODWORD, LOWORD};
use crate::handles::{HBITMAP, HBRUSH, HCURSOR, HDC, HEVENT, HICON, HINSTANCE, HMENU, HWND};
use crate::privs::LF_FACESIZE;
use crate::structs::{ATOM, COLORREF};
use crate::unions::{ColorrefDib, ColorrefHbitmap};
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

/// [`ACL`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-acl)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct ACL {
	pub AclRevision: u8,
	pub Sbz1: u8,
	pub AclSize: u16,
	pub AceCount: u16,
	pub Sbz2: u16,
}

/// [`ALTTABINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-alttabinfo)
/// struct.
#[repr(C)]
pub struct ALTTABINFO {
	cbSize: u32,
	pub cItems: i32,
	pub cColumns: i32,
	pub cRows: i32,
	pub iColFocus: i32,
	pub iRowFocus: i32,
	pub cxItem: i32,
	pub cyItem: i32,
	pub ptStart: POINT,
}

impl_default_with_size!(ALTTABINFO, cbSize);

/// [`BITMAPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo)
/// struct.
#[repr(C)]
pub struct BITMAPINFO {
	pub bmiHeader: BITMAPINFOHEADER,
	pub bmiColors: [RGBQUAD; 1],
}

/// [`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
/// struct.
#[repr(C)]
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

impl_default_with_size!(BITMAPINFOHEADER, biSize);

/// [`BY_HANDLE_FILE_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information)
/// method.
#[repr(C)]
#[derive(Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
	pub dwFileAttributes: co::FILE_ATTRIBUTE,
	pub ftCreationTime: FILETIME,
	pub ftLastAccessTime: FILETIME,
	pub ftLastWriteTime: FILETIME,
	pub dwVolumeSerialNumber: u32,
	pub nFileSizeHigh: u32,
	pub nFileSizeLow: u32,
	pub nNumberOfLinks: u32,
	pub nFileIndexHigh: u32,
	pub nFileIndexLow: u32,
}

/// [`CHOOSECOLOR`](https://docs.microsoft.com/en-us/windows/win32/api/commdlg/ns-commdlg-choosecolorw-r1)
/// struct.
#[repr(C)]
pub struct CHOOSECOLOR<'a, 'b> {
	pub lStructSize: u32,
	pub hwndOwner: HWND,
	pub hInstance: HWND,
	pub rgbResult: COLORREF,
	lpCustColors: *mut [COLORREF; 16],
	pub Flags: co::CC,
	pub lCustData: isize,
	pub lpfnHook: Option<CCHOOKPROC>,
	lpTemplateName: *mut u16,
	m_lpCustColors: PhantomData<&'a COLORREF>,
	m_lpTemplateName: PhantomData<&'b u16>,
}

impl_default_with_size!(CHOOSECOLOR, lStructSize, 'a, 'b);

impl<'a, 'b> CHOOSECOLOR<'a, 'b> {
	/// Returns the `lpCustColors` field.
	pub fn lpCustColors(&self) -> Option<&mut [COLORREF; 16]> {
		unsafe { self.lpCustColors.as_mut() }
	}

	// Sets the `lpCustColors` field.
	pub fn set_lpCustColors(&mut self, buf: &'a mut [COLORREF; 16]) {
		self.lpCustColors = buf;
	}

	string_get_set!('b, lpTemplateName, set_lpTemplateName);
}

/// [`CREATESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
/// struct.
#[repr(C)]
pub struct CREATESTRUCT<'a, 'b> {
	pub lpCreateParams: isize,
	pub hInstance: HINSTANCE,
	pub hMenu: HMENU,
	pub hwndParent: HWND,
	pub cy: i32,
	pub cx: i32,
	pub y: i32,
	pub x: i32,
	pub style: co::WS,
	lpszName: *mut u16,
	lpszClass: *mut u16,
	pub dwExStyle: co::WS_EX,
	m_lpszName: PhantomData<&'a u16>,
	m_lpszClass: PhantomData<&'b u16>,
}

impl_default_zero!(CREATESTRUCT, 'a, 'b);

impl<'a, 'b> CREATESTRUCT<'a, 'b> {
	string_get_set!('a, lpszName, set_lpszName);
	string_get_set!('b, lpszClass, set_lpszClass);
}

/// [`FILETIME`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct FILETIME {
	pub dwLowDateTime: u32,
	pub dwHighDateTime: u32,
}

/// [`HELPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-helpinfo)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct HELPINFO {
	cbSize: u32,
	pub iContextType: co::HELPINFO,
	pub iCtrlId: i32,
	hItemHandle: usize, // HWND|HMENU
	pub dwContextId: u32,
	pub MousePos: POINT,
}

impl HELPINFO {
	pub fn hItemHandle(&self) -> HwndHmenu {
		match self.iContextType {
			co::HELPINFO::WINDOW => HwndHmenu::Hwnd(HWND { ptr: self.hItemHandle as _ }),
			_ => HwndHmenu::Hmenu(HMENU { ptr: self.hItemHandle as _ }),
		}
	}
}

/// [`LOGBRUSH`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logbrush)
/// struct.
#[repr(C)]
pub struct LOGBRUSH {
	pub lbStyle: co::BSS,
	pub lbColor: ColorrefDib,
	pub lbHatch: ColorrefHbitmap,
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
	lfFaceName: [u16; LF_FACESIZE],
}

impl LOGFONT {
	/// Returns the `lfFaceName` field.
	pub fn lfFaceName(&self) -> String {
		WString::from_wchars_slice(&self.lfFaceName).to_string()
	}

	/// Sets the `lfFaceName` field.
	pub fn set_lfFaceName(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.lfFaceName);
	}
}

/// [`MEMORYSTATUSEX`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex)
/// struct.
#[repr(C)]
pub struct MEMORYSTATUSEX {
	dwLength: u32,
	pub dwMemoryLoad: u32,
	pub ullTotalPhys: u64,
	pub ullAvailPhys: u64,
	pub ullTotalPageFile: u64,
	pub ullAvailPageFile: u64,
	pub ullTotalVirtual: u64,
	pub ullAvailVirtual: u64,
	pub ullAvailExtendedVirtual: u64,
}

impl_default_with_size!(MEMORYSTATUSEX, dwLength);

/// [`MENUINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuinfo)
/// struct.
#[repr(C)]
pub struct MENUINFO {
	cbSize: u32,
	pub fMask: co::MIM,
	pub dwStyle: co::MNS,
	pub cyMax: u32,
	pub hbrBack: HBRUSH,
	pub dwContextHelpID: u32,
	pub dwMenuData: usize,
}

impl_default_with_size!(MENUINFO, cbSize);

/// [`MENUITEMINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-menuiteminfow)
/// struct.
#[repr(C)]
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

impl_default_with_size!(MENUITEMINFO, cbSize);

/// [`MINMAXINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-minmaxinfo)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct MINMAXINFO {
	ptReserved: POINT,
	pub ptMaxSize: POINT,
	pub ptMaxPosition: POINT,
	pub ptMinTrackSize: POINT,
	pub ptMaxTrackSize: POINT,
}

/// [`MSG`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct MSG {
	pub hwnd: HWND,
	pub message: co::WM,
	pub wParam: usize,
	pub lParam: isize,
	pub time: u32,
	pub pt: POINT,
	lPrivate: u32,
}

impl_default_zero!(MSG);

/// [`MSLLHOOKSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msllhookstruct)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct MSLLHOOKSTRUCT {
	pub pt: POINT,
	pub mouseData: u32,
	pub flags: co::LLMHF,
	pub time: u32,
	pub dwExtraInfo: u64,
}

/// [`NCCALCSIZE_PARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nccalcsize_params)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NCCALCSIZE_PARAMS<'a> {
	pub rgrc: [RECT; 3],
	lppos: *mut WINDOWPOS,
	m_lppos: PhantomData<&'a WINDOWPOS>,
}

impl<'a> NCCALCSIZE_PARAMS<'a> {
	/// Returns the `lppos` field.
	pub fn lppos(&self) -> Option<&mut WINDOWPOS> {
		unsafe { self.lppos.as_mut() }
	}

	/// Sets the `lppos` field.
	pub fn set_lppos(&mut self, lppos: &'a mut WINDOWPOS) {
		self.lppos = lppos;
	}
}

/// [`NMHDR`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: HWND,
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
		obj.cbSize = std::mem::size_of::<Self>() as _;

		let is_vista = IsWindowsVistaOrGreater()
			.unwrap_or_else(|err| panic!("{}", err)); // should never happen

		if !is_vista {
			obj.cbSize -= std::mem::size_of::<i32>() as u32
		}
		obj
	}
}

/// [`OSVERSIONINFOEX`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-osversioninfoexw)
/// struct.
#[repr(C)]
pub struct OSVERSIONINFOEX {
	dwOSVersionInfoSize: u32,
	pub dwMajorVersion: u32,
	pub dwMinorVersion: u32,
	pub dwBuildNumber: u32,
	pub dwPlatformId: co::VER_PLATFORM,
	szCSDVersion: [u16; 128],
	pub wServicePackMajor: u16,
	pub wServicePackMinor: u16,
	pub wSuiteMask: co::VER_SUITE,
	pub wProductType: co::VER_NT,
	wReserved: u8,
}

impl_default_with_size!(OSVERSIONINFOEX, dwOSVersionInfoSize);

impl OSVERSIONINFOEX {
	/// Returns the `szCSDVersion` field.
	pub fn szCSDVersion(&self) -> String {
		WString::from_wchars_slice(&self.szCSDVersion).to_string()
	}

	/// Sets the `szCSDVersion` field.
	pub fn get_szCSDVersion(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szCSDVersion);
	}
}

/// [`OVERLAPPED`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped)
/// struct.
#[repr(C)]
pub struct OVERLAPPED {
	pub Internal: usize,
	pub InternalHigh: usize,
	pub Pointer: usize,
	pub hEvent: HEVENT,
}

impl_default_zero!(OVERLAPPED);

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
/// struct.
#[repr(C)]
pub struct PAINTSTRUCT {
	pub hdc: HDC,
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
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct POINT {
	pub x: i32,
	pub y: i32,
}

impl std::fmt::Display for POINT {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "x {}, y {}", self.x, self.y)
	}
}

impl POINT {
	/// Creates a new `POINT`.
	pub fn new(x: i32, y: i32) -> POINT {
		Self { x, y }
	}
}

/// [`RECT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RECT {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

impl std::fmt::Display for RECT {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "left {}, top {}, right {}, bottom {}",
			self.left, self.top, self.right, self.bottom)
	}
}

impl RECT {
	/// Creates a new `RECT`.
	pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> RECT {
		Self { left, top, right, bottom }
	}
}

/// [`RGBQUAD`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgbquad)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct RGBQUAD {
	pub rgbBlue: u8,
	pub rgbGreen: u8,
	pub rgbRed: u8,
	rgbReserved: u8,
}

/// [`SCROLLINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-scrollinfo)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct SCROLLINFO {
	cbSize: u32,
	pub fMask: co::SIF,
	pub nMin: i32,
	pub nMax: i32,
	pub nPage: u32,
	pub nPos: i32,
	pub nTrackPos: i32,
}

impl_default_with_size!(SCROLLINFO, cbSize);

/// [`SECURITY_ATTRIBUTES`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/aa379560(v=vs.85))
/// struct.
#[repr(C)]
pub struct SECURITY_ATTRIBUTES<'a> {
	nLength: u32,
	lpSecurityDescriptor: *mut SECURITY_DESCRIPTOR,
	pub bInheritHandle: i32,
	m_lpSecurityDescriptor: PhantomData<&'a SECURITY_DESCRIPTOR>,
}

impl_default_with_size!(SECURITY_ATTRIBUTES, nLength, 'a);

impl<'a> SECURITY_ATTRIBUTES<'a> {
	/// Returns the `lpSecurityDescriptor` field.
	pub fn lpSecurityDescriptor(&self) -> Option<&mut SECURITY_DESCRIPTOR> {
		unsafe { self.lpSecurityDescriptor.as_mut() }
	}

	/// Sets the `lppos` field.
	pub fn set_lpSecurityDescriptor(&mut self, sd: &'a mut SECURITY_DESCRIPTOR) {
		self.lpSecurityDescriptor = sd;
	}
}

/// [`SECURITY_DESCRIPTOR`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-security_descriptor)
/// struct.
#[repr(C)]
pub struct SECURITY_DESCRIPTOR {
	pub Revision: u8,
   pub Sbz1: u8,
   pub Control: u16,
   pub Owner: *mut c_void,
   pub Group: *mut c_void,
   pub Sacl: *mut ACL,
   pub Dacl: *mut ACL,
}

/// [`SIZE`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-size)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SIZE {
	pub cx: i32,
	pub cy: i32,
}

impl std::fmt::Display for SIZE {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "cx {}, cy {}", self.cx, self.cy)
	}
}

impl SIZE {
	/// Creates a new `SIZE`.
	pub fn new(cx: i32, cy: i32) -> SIZE {
		Self { cx, cy }
	}
}

/// [`STYLESTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-stylestruct)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
#[derive(Default)]
pub struct STYLESTRUCT {
	styleOld: u32, // both fields contain WS and WS_EX mixed
	styleNew: u32,
}

impl STYLESTRUCT {
	/// Returns the [`WS`](crate::co::WS) of `styleOld` field.
	pub fn styleOld_WS(&self) -> co::WS {
		co::WS(self.styleOld)
	}

	/// Returns the [`WS_EX`](crate::co::WS_EX) of `styleOld` field.
	pub fn styleOld_WSEX(&self) -> co::WS_EX {
		co::WS_EX(self.styleOld)
	}

	/// Returns the [`WS`](crate::co::WS) of `styleNew` field.
	pub fn styleNew_WS(&self) -> co::WS {
		co::WS(self.styleNew)
	}

	/// Returns the [`WS_EX`](crate::co::WS_EX) of `styleNew` field.
	pub fn styleNew_WSEX(&self) -> co::WS_EX {
		co::WS_EX(self.styleNew)
	}
}

/// [`SYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct SYSTEMTIME {
	pub wYear: u16,
	pub wMonth: u16,
	pub wDayOfWeek: u16,
	pub wDay: u16,
	pub wHour: u16,
	pub wMinute: u16,
	pub wSecond: u16,
	pub wMilliseconds: u16,
}

/// [`TEXTMETRIC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-textmetricw)
/// struct.
#[repr(C)]
#[derive(Default, Clone)]
pub struct TEXTMETRIC {
	pub tmHeight: i32,
	pub tmAscent: i32,
	pub tmDescent: i32,
	pub tmInternalLeading: i32,
	pub tmExternalLeading: i32,
	pub tmAveCharWidth: i32,
	pub tmMaxCharWidth: i32,
	pub tmWeight: i32,
	pub tmOverhang: i32,
	pub tmDigitizedAspectX: i32,
	pub tmDigitizedAspectY: i32,
	pub tmFirstChar: u16,
	pub tmLastChar: u16,
	pub tmDefaultChar: u16,
	pub tmBreakChar: u16,
	pub tmItalic: u8,
	pub tmUnderlined: u8,
	pub tmStruckOut: u8,
	pub tmPitchAndFamily: u8,
	pub tmCharSet: u8,
}

/// [`TIME_ZONE_INFORMATION`](https://docs.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-time_zone_information)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct TIME_ZONE_INFORMATION {
	pub bias: i32,
	standardName: [u16; 32],
	pub standardDate: SYSTEMTIME,
	pub standardBias: i32,
	daylightName: [u16; 32],
	pub daylightDate: SYSTEMTIME,
	pub daylightBias: i32,
}

impl TIME_ZONE_INFORMATION {
	/// Returns the `standardName` field.
	pub fn standardName(&self) -> String {
		WString::from_wchars_slice(&self.standardName).to_string()
	}

	/// Sets the `standardName` field.
	pub fn set_standardName(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.standardName);
	}

	/// Returns the `daylightName` field.
	pub fn daylightName(&self) -> String {
		WString::from_wchars_slice(&self.daylightName).to_string()
	}

	/// Sets the `daylightName` field.
	pub fn set_daylightName(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.daylightName);
	}
}

/// [`TRACKMOUSEEVENT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-trackmouseevent)
/// struct.
#[repr(C)]
pub struct TRACKMOUSEEVENT {
	cbSize: u32,
	pub dwFlags: co::TME,
	pub hwndTrack: HWND,
	pub dwHoverTime: u32,
}

impl_default_with_size!(TRACKMOUSEEVENT, cbSize);

/// [`WINDOWINFO`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowinfo)
/// struct.
#[repr(C)]
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

impl_default_with_size!(WINDOWINFO, cbSize);

/// [`WINDOWPLACEMENT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)
/// struct.
#[repr(C)]
pub struct WINDOWPLACEMENT {
	length: u32,
	pub flags: co::WPF,
	pub showCmd: co::SW,
	pub ptMinPosition: POINT,
	pub ptMaxPosition: POINT,
	pub rcNormalPosition: RECT,
	pub rcDevice: RECT,
}

impl_default_with_size!(WINDOWPLACEMENT, length);

/// [`WINDOWPOS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowpos)
/// struct.
#[repr(C)]
pub struct WINDOWPOS {
	pub hwnd: HWND,
	hwndInsertAfter: isize,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub flags: co::SWP,
}

impl_default_zero!(WINDOWPOS);

impl WINDOWPOS {
	/// Returns the `hwndInsertAfter` field.
	pub fn hwndInsertAfter(&self) -> HwndPlace {
		match self.hwndInsertAfter {
			0 | 1 | -1 | -2 => HwndPlace::Place(co::HWND_PLACE(self.hwndInsertAfter)),
			_ => HwndPlace::Hwnd(HWND { ptr: self.hwndInsertAfter as _ }),
		}
	}

	/// Sets the `hwndInsertAfter` field.
	pub fn set_hwndInsertAfter(&mut self, hwnd: HwndPlace) {
		self.hwndInsertAfter = match hwnd {
			HwndPlace::Hwnd(h) => h.ptr as _,
			HwndPlace::Place(v) => v.into(),
			HwndPlace::None => 0,
		};
	}
}

/// [`WNDCLASSEX`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw)
/// struct.
#[repr(C)]
pub struct WNDCLASSEX<'a, 'b> {
	cbSize: u32,
	pub style: co::CS,
	pub lpfnWndProc: Option<WNDPROC>,
	pub cbClsExtra: i32,
	pub cbWndExtra: i32,
	pub hInstance: HINSTANCE,
	pub hIcon: HICON,
	pub hCursor: HCURSOR,
	pub hbrBackground: HBRUSH,
	lpszMenuName: *mut u16,
	lpszClassName: *mut u16,
	pub hIconSm: HICON,
	m_lpszMenuName: PhantomData<&'a u16>,
	m_lpszClassName: PhantomData<&'b u16>,
}

impl_default_with_size!(WNDCLASSEX, cbSize, 'a, 'b);

impl<'a, 'b> WNDCLASSEX<'a, 'b> {
	/// Returns the `lpszMenuName` field.
	pub fn lpszMenuName(&self) -> Option<IdStr> {
		unsafe { self.lpszMenuName.as_mut() }
			.map(|lp| {
				let lp2 = lp as *mut _; // https://stackoverflow.com/a/9806654/6923555
				if HIDWORD(lp2 as _) == 0 && HIWORD(LODWORD(lp2 as _)) == 0 {
					IdStr::Id(LOWORD(LODWORD(lp2 as _)) as _)
				} else {
					IdStr::Str(WString::from_wchars_nullt(lp))
				}
			})
	}

	/// Sets the `lpszMenuName` field.
	pub fn set_lpszMenuName(&mut self, menu_name: &'a mut IdStr) {
		self.lpszMenuName = menu_name.as_mut_ptr();
	}

	string_get_set!('b, lpszClassName, set_lpszClassName);
}
