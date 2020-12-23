#![allow(non_snake_case)]

use crate::ffi::{gdi32, HANDLE};
use crate::handles::HGDIOBJ;
use crate::internal_defs::const_void;
use crate::structs::RECT;

handle_type! {
	/// Handle to a
	/// [region](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hrgn).
	/// Exposes methods.
	HRGN
}

convert_hgdiobj!(HRGN);

impl HRGN {
	/// [`PtInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptinregion)
	/// method.
	pub fn PtInRegion(self, x: i32, y: i32) -> bool {
		unsafe { gdi32::PtInRegion(self.0, x, y) != 0 }
	}

	/// [`RectInRegion`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectinregion)
	/// method.
	pub fn RectInRegion(self, lprect: &RECT) -> bool {
		unsafe { gdi32::RectInRegion(self.0, const_void(lprect)) != 0 }
	}
}