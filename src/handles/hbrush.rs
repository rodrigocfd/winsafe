#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;

hgdiobj_type! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush)
	/// GDI object. Exposes methods.
	HBRUSH
}

impl HBRUSH {
	/// Creates a brush with the given system color.
	pub fn from_sys_color(color: co::COLOR) -> HBRUSH {
		unsafe { Self::from_ptr((u32::from(color) + 1) as *mut c_void) }
	}
}
