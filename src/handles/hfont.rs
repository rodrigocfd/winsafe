#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::structs::LOGFONT;
use crate::WString;

pub_struct_handle_gdi! {
	/// Handle to a
	/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
	HFONT
}

impl HFONT {
	/// [`CreateFont`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::HFONT::DeleteObject) call.
	pub fn CreateFont(
		cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32,
		cWeight: co::FW, bItalic: bool, bUnderline: bool, bStrikeOut: bool,
		iCharSet: co::CHARSET,
		iOutPrecision: co::OUT_PRECIS, iClipPrecision: co::CLIP,
		iQuality: co::QUALITY, iPitchAndFamily: co::PITCH,
		pszFaceName: &str) -> WinResult<HFONT>
	{
		unsafe {
			gdi32::CreateFontW(
				cHeight, cWidth, cEscapement, cOrientation,
				cWeight.0 as _,
				bItalic as _, bUnderline as _, bStrikeOut as _,
				iCharSet.0 as _,
				iOutPrecision.0 as _, iClipPrecision.0 as _,
				iQuality.0 as _, iPitchAndFamily.0 as _,
				WString::from_str(pszFaceName).as_ptr(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::HFONT::DeleteObject) call.
	pub fn CreateFontIndirect(lplf: &LOGFONT) -> WinResult<HFONT> {
		unsafe { gdi32::CreateFontIndirectW(lplf as *const _ as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	pub fn GetObject(self, pv: &mut LOGFONT) -> WinResult<()> {
		match unsafe {
			gdi32::GetObjectW(
				self.ptr,
				std::mem::size_of::<LOGFONT>() as _,
				pv as *mut _ as _,
			)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()), // not really an error
				err => Err(err),
			},
			_ => Ok(()),
		}
	}
}
