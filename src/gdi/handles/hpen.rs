#![allow(non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGPEN;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::{Handle, HandleGdi};
use crate::user::decl::COLORREF;

impl_handle! { HPEN: "gdi";
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
}

impl HandleGdi for HPEN {}
impl GdiHpen for HPEN {}

/// [`HPEN`](crate::HPEN) methods from `gdi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait GdiHpen: Handle {
	/// [`CreatePen`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
	/// static method.
	#[must_use]
	fn CreatePen(
		style: co::PS, width: i32, color: COLORREF) -> WinResult<HPEN>
	{
		unsafe { gdi::ffi::CreatePen(style.0, width, color.0).as_mut() }
			.map(|ptr| HPEN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePenIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpenindirect)
	/// static method.
	#[must_use]
	fn CreatePenIndirect(lp: &mut LOGPEN) -> WinResult<HPEN> {
		unsafe { gdi::ffi::CreatePenIndirect(lp as *const _ as _).as_mut() }
			.map(|ptr| HPEN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetStockObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	#[must_use]
	fn GetStockObject(sp: co::STOCK_PEN) -> WinResult<HPEN> {
		unsafe { gdi::ffi::GetStockObject(sp.0).as_mut() }
			.map(|ptr| HPEN(ptr))
			.ok_or_else(|| GetLastError())
	}
}
