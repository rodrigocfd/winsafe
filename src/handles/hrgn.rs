#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::structs::RECT;

pub_struct_handle_gdi! {
	/// Handle to a
	/// [region](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
	/// GDI object.
	HRGN
}

impl HRGN {
	/// [`CreateRectRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgn)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> WinResult<HRGN> {
		unsafe { gdi32::CreateRectRgn(x1, y1, x2, y2).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRectRgnIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgnindirect)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRectRgnIndirect(lprect: &RECT) -> WinResult<HRGN> {
		unsafe { gdi32::CreateRectRgnIndirect(lprect as *const _ as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRoundRectRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createroundrectrgn)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRoundRectRgn(
		x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> WinResult<HRGN>
	{
		unsafe { gdi32::CreateRoundRectRgn(x1, y1, x2, y2, w, h).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`OffsetClipRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetcliprgn)
	/// method.
	pub fn OffsetClipRgn(self, x: i32, y: i32) -> WinResult<co::REGION> {
		match unsafe { gdi32::OffsetClipRgn(self.ptr, x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`OffsetRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetrgn)
	/// method.
	pub fn OffsetRgn(self, x: i32, y: i32) -> WinResult<co::REGION> {
		match unsafe { gdi32::OffsetRgn(self.ptr, x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`PtInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptinregion)
	/// method.
	pub fn PtInRegion(self, x: i32, y: i32) -> bool {
		unsafe { gdi32::PtInRegion(self.ptr, x, y) != 0 }
	}

	/// [`RectInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectinregion)
	/// method.
	pub fn RectInRegion(self, lprect: &RECT) -> bool {
		unsafe {
			gdi32::RectInRegion(self.ptr, lprect as *const _ as _) != 0
		}
	}
}
