#![allow(non_snake_case)]

use crate::ffi::HANDLE;

use crate::handles::HGDIOBJ;

handle_type! {
	/// Handle to a
	/// [brush](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbrush).
	/// Exposes methods.
	HBRUSH
}

convert_hgdiobj!(HBRUSH);