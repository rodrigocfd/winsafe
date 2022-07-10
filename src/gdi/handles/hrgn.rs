#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::gdi_Hgdiobj;
use crate::user::decl::{HRGN, RECT, SIZE};

impl gdi_Hgdiobj for HRGN {}
impl gdi_Hrgn for HRGN {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HRGN`](crate::HRGN).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait gdi_Hrgn: gdi_Hgdiobj {
	/// [`CreateRectRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgn)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject) call.
	#[must_use]
	fn CreateRectRgn(bounds: RECT) -> WinResult<HRGN> {
		unsafe {
			gdi::ffi::CreateRectRgn(
				bounds.left, bounds.top, bounds.right, bounds.bottom,
			).as_mut()
		}.map(|ptr| HRGN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRectRgnIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgnindirect)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject) call.
	#[must_use]
	fn CreateRectRgnIndirect(rc: RECT) -> WinResult<HRGN> {
		unsafe { gdi::ffi::CreateRectRgnIndirect(&rc as *const _ as _).as_mut() }
			.map(|ptr| HRGN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRoundRectRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createroundrectrgn)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject) call.
	#[must_use]
	fn CreateRoundRectRgn(
		bounds: RECT, size: SIZE) -> WinResult<HRGN>
	{
		unsafe {
			gdi::ffi::CreateRoundRectRgn(
				bounds.left, bounds.top, bounds.right, bounds.top,
				size.cx, size.cy,
			).as_mut()
		}.map(|ptr| HRGN(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`OffsetClipRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetcliprgn)
	/// method.
	fn OffsetClipRgn(self, x: i32, y: i32) -> WinResult<co::REGION> {
		match unsafe { gdi::ffi::OffsetClipRgn(self.as_ptr(), x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`OffsetRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetrgn)
	/// method.
	fn OffsetRgn(self, x: i32, y: i32) -> WinResult<co::REGION> {
		match unsafe { gdi::ffi::OffsetRgn(self.as_ptr(), x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`PtInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptinregion)
	/// method.
	#[must_use]
	fn PtInRegion(self, x: i32, y: i32) -> bool {
		unsafe { gdi::ffi::PtInRegion(self.as_ptr(), x, y) != 0 }
	}

	/// [`RectInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectinregion)
	/// method.
	#[must_use]
	fn RectInRegion(self, rc: &RECT) -> bool {
		unsafe { gdi::ffi::RectInRegion(self.as_ptr(), rc as *const _ as _) != 0 }
	}
}