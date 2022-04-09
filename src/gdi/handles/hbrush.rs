#![allow(non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGBRUSH;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, HandleGdi};
use crate::user::decl::{COLORREF, HBITMAP, HBRUSH};

impl HandleGdi for HBRUSH {}
impl GdiHbrush for HBRUSH {}

/// [`HBRUSH`](crate::HBRUSH) methods from `gdi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait GdiHbrush: Handle {
	/// Creates a brush with the given system color.
	///
	/// **Note:** This should be used only to initialize the
	/// [`WNDCLASSEX`](crate::WNDCLASSEX)'s `hbrBackground` field. Any other use
	/// will yield an invalid handle.
	#[must_use]
	fn from_sys_color(color: co::COLOR) -> HBRUSH {
		HBRUSH((color.0 + 1) as _ )
	}

	/// [`CreateBrushIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn CreateBrushIndirect(lb: &LOGBRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::CreateBrushIndirect(lb as *const _ as _).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateHatchBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn CreateHatchBrush(
		hatch: co::HS, color: COLORREF) -> WinResult<HBRUSH>
	{
		unsafe { gdi::ffi::CreateHatchBrush(hatch.0, color.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePatternBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn CreatePatternBrush(hbmp: HBITMAP) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::CreatePatternBrush(hbmp.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateSolidBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBRUSH::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	#[must_use]
	fn CreateSolidBrush(color: COLORREF) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::CreateSolidBrush(color.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(self, pv: &mut LOGBRUSH) -> WinResult<()> {
		match unsafe {
			gdi::ffi::GetObjectW(
				self.as_ptr(),
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
	#[must_use]
	fn GetStockObject(sb: co::STOCK_BRUSH) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::GetStockObject(sb.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetSysColorBrush`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// static method.
	#[must_use]
	fn GetSysColorBrush(index: co::COLOR) -> WinResult<HBRUSH> {
		unsafe { gdi::ffi::GetSysColorBrush(index.0).as_mut() }
			.map(|ptr| HBRUSH(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`UnrealizeObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-unrealizeobject)
	/// method.
	fn UnrealizeObject(self) -> WinResult<()> {
		bool_to_winresult(unsafe { gdi::ffi::UnrealizeObject(self.as_ptr()) })
	}
}
