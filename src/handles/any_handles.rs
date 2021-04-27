#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::gdi32;
use crate::privs::bool_to_winresult;

hgdiobj_type! {
	/// Handle to a
	/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	HBITMAP
}

hgdiobj_type! {
	/// Handle to a
	/// [pen](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hpen)
	/// GDI object.
	HPEN
}

handle_type! {
	/// Handle to an
	/// [event](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw).
	/// Originally just a `HANDLE`.
	HEVENT
}

handle_type! {
	/// Handle to an
	/// [tree view item](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-controls).
	HTREEITEM
}
