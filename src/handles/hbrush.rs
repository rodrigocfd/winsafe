#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::handles::HBITMAP;
use crate::structs::{COLORREF, LOGBRUSH};

pub_struct_handle_gdi! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	HBRUSH
}

impl HBRUSH {
	/// Creates a brush with the given system color.
	///
	/// **Note:** This should be used only to initialize the
	/// [`WNDCLASSEX`](crate::WNDCLASSEX)'s `hbrBackground` field. Any other use
	/// will yield an invalid handle.
	pub const fn from_sys_color(color: co::COLOR) -> HBRUSH {
		Self { ptr: (color.0 + 1) as _ }
	}

	/// [`CreateBrushIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateBrushIndirect(lb: &LOGBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreateBrushIndirect(lb as *const _ as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateHatchBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateHatchBrush(
		hatch: co::HS, color: COLORREF) -> WinResult<HBRUSH>
	{
		unsafe { gdi32::CreateHatchBrush(hatch.0, color.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePatternBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreatePatternBrush(hbmp: HBITMAP) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreatePatternBrush(hbmp.ptr).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateSolidBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateSolidBrush(color: COLORREF) -> WinResult<HBRUSH> {
		unsafe { gdi32::CreateSolidBrush(color.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	pub fn GetObject(self, pv: &mut LOGBRUSH) -> WinResult<()> {
		match unsafe {
			gdi32::GetObjectW(
				self.ptr,
				std::mem::size_of::<LOGBRUSH>() as _,
				pv as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetStockObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	pub fn GetStockObject(sb: co::STOCK_BRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi32::GetStockObject(sb.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetSysColorBrush`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// static method.
	pub fn GetSysColorBrush(index: co::COLOR) -> WinResult<HBRUSH> {
		unsafe { gdi32::GetSysColorBrush(index.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}
}
