#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::structs::{COLORREF, LOGPEN};

pub_struct_handle_gdi! {
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
	HPEN
}

impl HPEN {
	/// [`CreatePen`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// static method.
	pub fn CreatePen(
		iStyle: co::PS, cWidth: i32, color: COLORREF) -> WinResult<HPEN>
	{
		unsafe { gdi32::CreatePen(iStyle.0, cWidth, color.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePenIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// static method.
	pub fn CreatePenIndirect(plpen: &mut LOGPEN) -> WinResult<HPEN> {
		unsafe { gdi32::CreatePenIndirect(plpen as *const _ as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetStockObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	pub fn GetStockObject(i: co::STOCK_PEN) -> WinResult<HPEN> {
		unsafe { gdi32::GetStockObject(i.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}
}
