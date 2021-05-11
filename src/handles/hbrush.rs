#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::handles::HBITMAP;
use crate::privs::ref_as_pcvoid;
use crate::structs::{COLORREF, LOGBRUSH};

hgdiobj_type! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	HBRUSH
}

impl HBRUSH {
	/// Creates a brush with the given system color.
	pub const fn from_sys_color(color: co::COLOR) -> HBRUSH {
		Self { ptr: (color.0 + 1) as _ }
	}

	/// [`CreateBrushIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateBrushIndirect(plbrush: &LOGBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreateBrushIndirect(ref_as_pcvoid(plbrush)).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateHatchBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateHatchBrush(
		iHatch: co::HS, color: COLORREF) -> WinResult<HBRUSH>
	{
		unsafe { gdi32::CreateHatchBrush(iHatch.0, color.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePatternBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreatePatternBrush(hbm: HBITMAP) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreatePatternBrush(hbm.ptr).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateSolidBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateSolidBrush(color: COLORREF) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreateSolidBrush(color.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetSysColorBrush`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// static method.
	pub fn GetSysColorBrush(nIndex: co::COLOR) -> WinResult<HBRUSH> {
		unsafe { gdi32::GetSysColorBrush(nIndex.0).as_mut() }
		.map(|ptr| Self { ptr })
		.ok_or_else(|| GetLastError())
	}
}
