#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::handles::HandleGdi;
use crate::structs::{COLORREF, LOGPEN};

/// Handle to a
/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
/// GDI object.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HPEN(pub(crate) *mut std::ffi::c_void);

impl_handle!(HPEN);
impl HandleGdi for HPEN {}

impl HPEN {
	/// [`CreatePen`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// static method.
	pub fn CreatePen(
		style: co::PS, width: i32, color: COLORREF) -> WinResult<HPEN>
	{
		unsafe { gdi32::CreatePen(style.0, width, color.0).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePenIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// static method.
	pub fn CreatePenIndirect(lp: &mut LOGPEN) -> WinResult<HPEN> {
		unsafe { gdi32::CreatePenIndirect(lp as *const _ as _).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetStockObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	pub fn GetStockObject(sp: co::STOCK_PEN) -> WinResult<HPEN> {
		unsafe { gdi32::GetStockObject(sp.0).as_mut() }
			.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}
}
