#![allow(non_snake_case)]

use crate::co;
use crate::funcs::{LOBYTE, LOWORD};

/// [`ATOM`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#atom)
/// returned by [`RegisterClassEx`](crate::RegisterClassEx).
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ATOM(pub(crate) u16);

impl std::fmt::Display for ATOM {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl ATOM {
	/// Useful to pass the atom as class name.
	pub const fn as_ptr(self) -> *const u16 {
		self.0 as _
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

impl std::fmt::Display for COLORREF {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "R {}, G {}, B {}",
			self.GetRValue(), self.GetGValue(), self.GetBValue())
	}
}

impl COLORREF {
	/// Creates a new `COLORREF` object with the given color intensities.
	pub const fn new(red: u8, green: u8, blue: u8) -> COLORREF {
		Self(red as u32 | ((green as u32) << 8) | ((blue as u32) << 16))
	}

	/// Retrieves the red intensity. Originally
	/// [`GetRValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getrvalue)
	/// macro.
	pub const fn GetRValue(self) -> u8 {
		LOBYTE(LOWORD(self.0))
	}

	/// Retrieves the green intensity. Originally
	/// [`GetGValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getgvalue)
	/// macro.
	pub const fn GetGValue(self) -> u8 {
		LOBYTE(LOWORD(self.0 >> 8))
	}

	/// Retrieves the blue intensity. Originally
	/// [`GetBValue`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getbvalue)
	/// macro.
	pub const fn GetBValue(self) -> u8 {
		LOBYTE(LOWORD(self.0 >> 16))
	}
}

/// [`LANGID`](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifiers)
/// language identifier.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LANGID(pub(crate) u16);

impl LANGID {
	pub const SYSTEM_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::SYS_DEFAULT);
	pub const USER_DEFAULT: Self = Self::new(co::LANG::NEUTRAL, co::SUBLANG::DEFAULT);

	/// Creates a new `LANGID`. Originally
	/// [`MAKELANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	pub const fn new(lang: co::LANG, sublang: co::SUBLANG) -> LANGID {
		Self((sublang.0 << 10) | lang.0)
	}

	/// Returns the primary language ID. Originally
	/// [`PRIMARYLANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-primarylangid)
	/// macro.
	pub const fn primary_lang_id(self) -> co::LANG {
		co::LANG(self.0 & 0x3ff)
	}

	/// Returns the sublanguage ID. Originally
	/// [`SUBLANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sublangid)
	/// macro.
	pub const fn sub_lang_id(self) -> co::SUBLANG {
		co::SUBLANG(self.0 >> 10)
	}
}

/// [`LCID`](https://docs.microsoft.com/en-us/windows/win32/intl/locale-identifiers)
/// locale identifier.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LCID(pub(crate) u32);

impl LCID {
	pub const SYSTEM_DEFAULT: Self = Self::new(LANGID::SYSTEM_DEFAULT, co::SORT::DEFAULT);
	pub const USER_DEFAULT: Self = Self::new(LANGID::USER_DEFAULT, co::SORT::DEFAULT);

	/// Creates a new `LCID`. Originally
	/// [`MAKELCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelcid)
	/// macro.
	pub const fn new(lang_id: LANGID, sort_id: co::SORT) -> LCID {
		Self(((sort_id.0 as u32) << 16) | lang_id.0 as u32)
	}

	/// Returns the language identifier. Originally
	/// [`LANGIDFROMLCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-langidfromlcid)
	/// macro.
	pub const fn lang_id(self) -> LANGID {
		LANGID(self.0 as _)
	}

	/// Returns the sort ID. Originally
	/// [`SORTIDFROMLCID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-sortidfromlcid)
	/// macro.
	pub const fn sort_id(self) -> co::SORT {
		co::SORT(((self.0 >> 16) & 0xf) as _)
	}
}
