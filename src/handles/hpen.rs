#![allow(non_snake_case)]

use crate::handles::HGDIOBJ;

handle_type! {
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen).
	/// Exposes methods.
	HPEN
}

convert_hgdiobj!(HPEN);