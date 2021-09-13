#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::structs::{RECT, SIZE};

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
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRectRgn(bounds: RECT) -> WinResult<HRGN> {
		unsafe {
			gdi32::CreateRectRgn(
				bounds.left, bounds.top, bounds.right, bounds.bottom,
			).as_mut()
		}
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRectRgnIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgnindirect)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRectRgnIndirect(rc: &RECT) -> WinResult<HRGN> {
		unsafe { gdi32::CreateRectRgnIndirect(rc as *const _ as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreateRoundRectRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createroundrectrgn)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn CreateRoundRectRgn(
		bounds: RECT, size: SIZE) -> WinResult<HRGN>
	{
		unsafe {
			gdi32::CreateRoundRectRgn(
				bounds.left, bounds.top, bounds.right, bounds.top,
				size.cx, size.cy,
			).as_mut()
		}.map(|ptr| Self { ptr })
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
	pub fn RectInRegion(self, rc: &RECT) -> bool {
		unsafe {
			gdi32::RectInRegion(self.ptr, rc as *const _ as _) != 0
		}
	}
}
