use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variant parameter for:
///
/// * [`HDC::GetCurrentObject`](crate::HDC::GetCurrentObject)
///
/// The enum values match those in [`co::CUR_OBJ`](crate::co::CUR_OBJ) constant
/// type.
pub enum CurObj {
	Bitmap(HBITMAP),
	Brush(HBRUSH),
	Font(HFONT),
	Pal(HPALETTE),
	Pen(HPEN),
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageBitmap`](crate::HINSTANCE::LoadImageBitmap)
#[derive(Clone)]
pub enum IdObmStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OBM`](crate::co::OBM) constant for an OEM bitmap.
	Obm(co::OBM),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdObmStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdObmStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Obm(obm) => MAKEINTRESOURCE(obm.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageCursor`](crate::HINSTANCE::LoadImageCursor)
#[derive(Clone)]
pub enum IdOcrStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OCR`](crate::co::OCR) constant for an OEM cursor.
	Ocr(co::OCR),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOcrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdOcrStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Ocr(ocr) => MAKEINTRESOURCE(ocr.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageIcon`](crate::HINSTANCE::LoadImageIcon)
#[derive(Clone)]
pub enum IdOicStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OIC`](crate::co::OIC) constant for an OEM icon.
	Oic(co::OIC),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOicStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdOicStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Oic(oic) => MAKEINTRESOURCE(oic.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}
