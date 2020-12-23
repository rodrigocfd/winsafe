#![allow(non_snake_case)]

use crate::ffi::{gdi32, HANDLE};
use crate::handles::HGDIOBJ;
use crate::internal_defs::const_void;
use crate::structs::LOGFONT;

handle_type! {
	/// Handle to a
	/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
	/// Exposes methods.
	HFONT
}

convert_hgdiobj!(HFONT);

impl HFONT {
	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	pub fn CreateFontIndirect(lplf: &LOGFONT) -> Result<HFONT, ()> {
		match ptr_as_opt!(gdi32::CreateFontIndirectW(const_void(lplf))) {
			Some(p) => Ok(Self(p)),
			None => Err(()),
		}
	}
}