#![allow(non_snake_case)]

use crate::ffi::gdi32;

hgdiobj_type! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush)
	/// GDI object. Exposes methods.
	HBRUSH
}