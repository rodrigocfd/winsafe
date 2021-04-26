#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, ptr_as_opt, ref_as_pcvoid};
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
		ptr_as_opt(
			unsafe {
				gdi32::CreateFontW(
					cHeight, cWidth, cEscapement, cOrientation,
					cWeight.0 as _,
					bItalic as _, bUnderline as _, bStrikeOut as _,
					iCharSet.0 as _,
					iOutPrecision.0 as _, iClipPrecision.0 as _,
					iQuality.0 as _, iPitchAndFamily.0 as _,
					WString::from_str(pszFaceName).as_ptr(),
				)
			},
		).map(|ptr| Self { ptr }).ok_or_else(|| GetLastError())
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	pub fn CreateFontIndirect(lplf: &LOGFONT) -> WinResult<HFONT> {
		ptr_as_opt(
			unsafe { gdi32::CreateFontIndirectW(ref_as_pcvoid(lplf)) },
		).map(|ptr| Self { ptr }).ok_or_else(|| GetLastError())
	}
}
