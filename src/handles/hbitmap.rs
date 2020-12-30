#![allow(non_snake_case)]

use crate::ffi::gdi32;

hgdiobj_type! {
	/// Handle to a
	/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap)
	/// GDI object. Exposes methods.
	HBITMAP
}
