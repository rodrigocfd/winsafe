#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, ptr_as_opt};
use crate::structs::LOGFONT;
use crate::WString;

hgdiobj_type! {
	/// Handle to a
	/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
	HFONT
}

impl HFONT {
	/// [`CreateFont`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	pub fn CreateFont(
		cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32,
		cWeight: co::FW, bItalic: bool, bUnderline: bool, bStrikeOut: bool,
		iCharSet: co::CHARSET,
		iOutPrecision: co::OUT_PRECIS, iClipPrecision: co::CLIP,
		iQuality: co::QUALITY, iPitchAndFamily: co::PITCH,
		pszFaceName: &str) -> WinResult<HFONT>
	{
		match ptr_as_opt(
			unsafe {
				gdi32::CreateFontW(
					cHeight, cWidth, cEscapement, cOrientation,
					cWeight.0 as i32,
					bItalic as u32, bUnderline as u32, bStrikeOut as u32,
					iCharSet.0 as u32,
					iOutPrecision.0 as u32, iClipPrecision.0 as u32,
					iQuality.0 as u32, iPitchAndFamily.0 as u32,
					WString::from_str(pszFaceName).as_ptr(),
				)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	pub fn CreateFontIndirect(lplf: &LOGFONT) -> WinResult<HFONT> {
		match ptr_as_opt(
			unsafe {gdi32::CreateFontIndirectW(lplf as *const _ as *const _) }
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}
}
