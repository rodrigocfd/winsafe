#![allow(non_snake_case)]

use crate::ffi::gdi32;

hgdiobj_type! {
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object. Exposes methods.
	HPEN
}