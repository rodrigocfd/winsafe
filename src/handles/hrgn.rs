#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::privs::ref_as_pcvoid;
use crate::structs::RECT;

pub_struct_handle_gdi! {
	/// Handle to a
	/// [region](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
	/// GDI object.
	HRGN
}

impl HRGN {
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
			gdi32::RectInRegion(self.ptr, ref_as_pcvoid(lprect)) != 0
		}
	}
}
