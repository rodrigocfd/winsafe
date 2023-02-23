#![allow(non_camel_case_types, non_snake_case)]

use std::alloc::Layout;
use std::ops::{Deref, DerefMut};

use crate::co;
use crate::gdi::privs::LF_FACESIZE;
use crate::kernel::decl::IsWindowsVistaOrGreater;
use crate::user::decl::{COLORREF, POINT};

/// [`BITMAP`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmap)
/// struct.
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

/// [`BITMAPFILEHEADER`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapfileheader)
/// struct.
#[repr(C, packed(2))]
pub struct BITMAPFILEHEADER {
	bfType: u16,
	pub bfSize: u32,
	bfReserved1: u16,
	bfReserved2: u16,
	pub bfOffBits: u32,
}

impl Default for BITMAPFILEHEADER {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.bfType = 0x4d42; // BM
		obj
	}
}

impl BITMAPFILEHEADER {
	pub_fn_serialize!();
}

/// [`BITMAPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo)
/// struct.
#[repr(C)]
pub struct BITMAPINFO {
	pub bmiHeader: BITMAPINFOHEADER,
	pub bmiColors: [RGBQUAD; 1],
}

impl Default for BITMAPINFO {
	fn default() -> Self {
		Self {
			bmiHeader: BITMAPINFOHEADER::default(),
			bmiColors: [RGBQUAD::default()],
		}
	}
}

/// [`BITMAPINFOHEADER`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
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

impl BITMAPINFOHEADER {
	pub_fn_serialize!();
}

/// [`LOGBRUSH`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logbrush)
/// struct.
#[repr(C)]
pub struct LOGBRUSH {
	pub lbStyle: co::BSS,
	pub lbColor: COLORREF,
	pub lbHatch: usize, // weird field
}

impl_default!(LOGBRUSH);

/// [`LOGFONT`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw)
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
	pub_fn_string_arr_get_set!(lfFaceName, set_lfFaceName);
}

/// [`LOGPALETTE`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logpalette)
/// struct.
///
/// Note that you cannot directly instantiate this struct, because the
/// `palPalEntry` field is dynamically allocated. That's why the
/// [`new`](crate::LOGPALETTE::new) static method returns a
/// [`LOGPALETTE_wrap`](crate::LOGPALETTE_wrap) object.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{LOGPALETTE, PALETTEENTRY};
///
/// let mut log_pal = LOGPALETTE::new(0x300, &[
///     PALETTEENTRY { peRed: 1, peGreen: 2, peBlue: 3, ..Default::default() },
///     PALETTEENTRY { peRed: 10, peGreen: 20, peBlue: 30, ..Default::default() },
/// ]);
///
/// // Setting a new entry value
/// log_pal.palPalEntry_mut()[0].peRed = 255;
///
/// // Printing all entry values
/// for entry in log_pal.palPalEntry().iter() {
///     println!("{} {} {}", entry.peRed, entry.peGreen, entry.peBlue);
/// }
/// ```
#[repr(C)]
pub struct LOGPALETTE {
	pub palVersion: u16,
	palNumEntries: u16,
	palPalEntry: [PALETTEENTRY; 1],
}

impl LOGPALETTE {
	/// Returns a [`LOGPALETTE_wrap`](crate::LOGPALETTE_wrap) with an underlying
	/// `LOGPALETTE` struct, dynamically alocated with the given
	/// [`PALETTEENTRY`](crate::PALETTEENTRY) entries.
	#[must_use]
	pub fn new(palVersion: u16, entries: &[PALETTEENTRY]) -> LOGPALETTE_wrap {
		LOGPALETTE_wrap::new(palVersion, entries)
	}

	/// Returns a constant slice over the `palPalEntry` entries.
	#[must_use]
	pub const fn palPalEntry(&self) -> &[PALETTEENTRY] {
		unsafe {
			std::slice::from_raw_parts(
				self.palPalEntry.as_ptr(),
				self.palNumEntries as _,
			)
		}
	}

	/// Returns a mutable slice over the `palPalEntry` entries.
	#[must_use]
	pub fn palPalEntry_mut(&mut self) -> &mut [PALETTEENTRY] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self.palPalEntry.as_mut_ptr(),
				self.palNumEntries as _,
			)
		}
	}
}

/// Safe wrapper over [`LOGPALETTE`](crate::LOGPALETTE), which automatically
/// manages the dynamic allocation.
pub struct LOGPALETTE_wrap {
	layout: Layout,
	log_pal: *mut LOGPALETTE,
}

impl Drop for LOGPALETTE_wrap {
	fn drop(&mut self) {
		unsafe { std::alloc::dealloc(self.log_pal as _, self.layout); }
	}
}

impl Deref for LOGPALETTE_wrap {
	type Target = LOGPALETTE;

	fn deref(&self) -> &Self::Target {
		unsafe { self.log_pal.as_ref() }.unwrap()
	}
}

impl DerefMut for LOGPALETTE_wrap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.log_pal.as_mut() }.unwrap()
	}
}

impl LOGPALETTE_wrap {
	fn new(palVersion: u16, entries: &[PALETTEENTRY]) -> Self {
		// https://stackoverflow.com/q/75544466/6923555
		let layout = Layout::new::<LOGPALETTE>()
			.extend(Layout::array::<PALETTEENTRY>(entries.len() - 1).unwrap())
			.unwrap().0;
		let log_pal = unsafe { std::alloc::alloc(layout) }.cast::<LOGPALETTE>();
		if log_pal.is_null() {
			std::alloc::handle_alloc_error(layout)
		}
		let mut new_self = Self { layout, log_pal };

		let log_pal_mut = new_self.deref_mut();
		log_pal_mut.palVersion = palVersion;
		log_pal_mut.palNumEntries = entries.len() as _;

		let arr = log_pal_mut.palPalEntry_mut();
		entries.iter().enumerate().for_each(|(idx, pe)| arr[idx] = *pe);

		new_self
	}
}

/// [`LOGPEN`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logpen)
/// struct.
#[repr(C)]
pub struct LOGPEN {
	pub lopnStyle: co::PS,
	pub lopnWidth: POINT,
	pub lopnColor: COLORREF,
}

impl_default!(LOGPEN);

/// [`NONCLIENTMETRICS`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw)
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

/// [`PALETTEENTRY`](https://learn.microsoft.com/en-us/previous-versions/dd162769(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct PALETTEENTRY {
	pub peRed: u8,
	pub peGreen: u8,
	pub peBlue: u8,
	pub peFlags: co::PC,
}

/// [`RGBQUAD`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgbquad)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub struct RGBQUAD {
	pub rgbBlue: u8,
	pub rgbGreen: u8,
	pub rgbRed: u8,
	rgbReserved: u8,
}

/// [`TEXTMETRIC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-textmetricw)
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
