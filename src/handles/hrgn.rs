#![allow(non_snake_case)]

use crate::co;
use crate::ffi::gdi32;
use crate::funcs_priv::const_void;
use crate::funcs::GetLastError;
use crate::structs::RECT;

hgdiobj_type! {
	/// Handle to a
	/// [region](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn)
	/// GDI object. Exposes methods.
	HRGN
}

impl HRGN {
	/// [`OffsetClipRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetcliprgn)
	/// method.
	pub fn OffsetClipRgn(self, x: i32, y: i32) -> Result<co::REGION, co::ERROR> {
		match unsafe { gdi32::OffsetClipRgn(self.ptr, x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`OffsetRgn`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetrgn)
	/// method.
	pub fn OffsetRgn(self, x: i32, y: i32) -> Result<co::REGION, co::ERROR> {
		match unsafe { gdi32::OffsetRgn(self.ptr, x, y) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION::from(ret)),
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
		unsafe { gdi32::RectInRegion(self.ptr, const_void(lprect)) != 0 }
	}
}
