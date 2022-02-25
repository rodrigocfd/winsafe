#![allow(non_snake_case)]

use crate::co;
use crate::gdi::privs::LF_FACESIZE;
use crate::kernel::decl::IsWindowsVistaOrGreater;
use crate::user::decl::{COLORREF, POINT};

/// [`BITMAP`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmap)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
#[repr(C)]
pub struct BITMAP {
	pub bmType: i32,
	pub bmWidth: i32,
	pub bmHeight: i32,
	pub bmWidthBytes: i32,
	pub bmPlanes: u16,
	pub bmBitsPixel: u16,
	pub bmBits: *mut u8,
}

impl_default!(BITMAP);

/// [`BITMAPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
#[repr(C)]
pub struct BITMAPINFO {
	pub bmiHeader: BITMAPINFOHEADER,
	pub bmiColors: [RGBQUAD; 1],
}

/// [`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
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

/// [`LOGBRUSH`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logbrush)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
#[repr(C)]
pub struct LOGBRUSH {
	pub lbStyle: co::BSS,
	pub lbColor: COLORREF,
	pub lbHatch: usize, // weird field
}

impl_default!(LOGBRUSH);

/// [`LOGFONT`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
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
	pub_fn_string_arr_get_set!(lfFaceName, set_lfFaceName);
}

/// [`LOGPEN`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logpen)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
#[repr(C)]
pub struct LOGPEN {
	pub lopnStyle: co::PS,
	pub lopnWidth: POINT,
	pub lopnColor: COLORREF,
}

impl_default!(LOGPEN);

/// [`NONCLIENTMETRICS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
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

/// [`RGBQUAD`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgbquad)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct RGBQUAD {
	pub rgbBlue: u8,
	pub rgbGreen: u8,
	pub rgbRed: u8,
	rgbReserved: u8,
}

/// [`TEXTMETRIC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-textmetricw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
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
