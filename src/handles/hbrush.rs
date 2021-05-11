#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::structs::COLORREF;

hgdiobj_type! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	HBRUSH
}

impl HBRUSH {
	/// Creates a brush with the given system color.
	pub const fn from_sys_color(color: co::COLOR) -> HBRUSH {
		Self { ptr: (color.0 + 1) as _ }
	}

	/// [`CreateSolidBrush`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HBRUSH::DeleteObject) call.
	pub fn CreateSolidBrush(color: COLORREF) -> WinResult<HBRUSH> {
		unsafe {
			gdi32::CreateSolidBrush(color.0).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}
}
