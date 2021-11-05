#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::handles::HandleGdi;
use crate::structs::{LOGFONT, SIZE};
use crate::various::WString;

/// Handle to a
/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HFONT(pub(crate) *mut std::ffi::c_void);

impl_handle!(HFONT);
impl HandleGdi for HFONT {}

impl HFONT {
	/// [`CreateFont`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::HFONT::DeleteObject) call.
	pub fn CreateFont(
		sz: SIZE, escapement: i32, orientation: i32,
		weight: co::FW, italic: bool, underline: bool, strike_out: bool,
		char_set: co::CHARSET,
		out_precision: co::OUT_PRECIS, clip_precision: co::CLIP,
		quality: co::QUALITY, pitch_and_family: co::PITCH,
		face_name: &str) -> WinResult<HFONT>
	{
		unsafe {
			gdi32::CreateFontW(
				sz.cy, sz.cx, escapement, orientation,
				weight.0 as _,
				italic as _, underline as _, strike_out as _,
				char_set.0 as _,
				out_precision.0 as _, clip_precision.0 as _,
				quality.0 as _, pitch_and_family.0 as _,
				WString::from_str(face_name).as_ptr(),
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::HFONT::DeleteObject) call.
	pub fn CreateFontIndirect(lf: &LOGFONT) -> WinResult<HFONT> {
		unsafe { gdi32::CreateFontIndirectW(lf as *const _ as _).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	pub fn GetObject(self, lf: &mut LOGFONT) -> WinResult<()> {
		match unsafe {
			gdi32::GetObjectW(
				self.0,
				std::mem::size_of::<LOGFONT>() as _,
				lf as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetStockObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	pub fn GetStockObject(sf: co::STOCK_FONT) -> WinResult<HFONT> {
		unsafe { gdi32::GetStockObject(sf.0).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}
}
