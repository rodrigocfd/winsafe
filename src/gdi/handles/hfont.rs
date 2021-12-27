#![allow(non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGFONT;
use crate::kernel::decl::{GetLastError, WinResult, WString};
use crate::prelude::{Handle, HandleGdi};
use crate::user::decl::SIZE;

impl_handle! { HFONT: "gdi";
	/// Handle to a
	/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
}

impl HandleGdi for HFONT {}
impl GdiHfont for HFONT {}

/// [`HFONT`](crate::HFONT) methods from `gdi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait GdiHfont: Handle {
	/// [`CreateFont`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	fn CreateFont(
		sz: SIZE, escapement: i32, orientation: i32,
		weight: co::FW, italic: bool, underline: bool, strike_out: bool,
		char_set: co::CHARSET,
		out_precision: co::OUT_PRECIS, clip_precision: co::CLIP,
		quality: co::QUALITY, pitch_and_family: co::PITCH,
		face_name: &str) -> WinResult<HFONT>
	{
		unsafe {
			gdi::ffi::CreateFontW(
				sz.cy, sz.cx, escapement, orientation,
				weight.0 as _,
				italic as _, underline as _, strike_out as _,
				char_set.0 as _,
				out_precision.0 as _, clip_precision.0 as _,
				quality.0 as _, pitch_and_family.0 as _,
				WString::from_str(face_name).as_ptr(),
			).as_mut()
		}.map(|ptr| HFONT(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HFONT::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	fn CreateFontIndirect(lf: &LOGFONT) -> WinResult<HFONT> {
		unsafe { gdi::ffi::CreateFontIndirectW(lf as *const _ as _).as_mut() }
			.map(|ptr| HFONT(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(self, lf: &mut LOGFONT) -> WinResult<()> {
		match unsafe {
			gdi::ffi::GetObjectW(
				self.as_ptr(),
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
	fn GetStockObject(sf: co::STOCK_FONT) -> WinResult<HFONT> {
		unsafe { gdi::ffi::GetStockObject(sf.0).as_mut() }
			.map(|ptr| HFONT(ptr))
			.ok_or_else(|| GetLastError())
	}
}
