#![allow(non_snake_case)]

use crate::ffi::{gdi32, HANDLE};

handle_type! {
	/// Handle to a
	/// [GDI object](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hgdiobj).
	/// Exposes methods.
	HGDIOBJ
}

impl HGDIOBJ {
	/// [`DeleteObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
	/// method.
	pub fn DeleteObject(self) -> Result<(), ()> {
		match unsafe { gdi32::DeleteObject(self.0) } {
 			0 => Err(()),
			_ => Ok(()),
		}
	}
}