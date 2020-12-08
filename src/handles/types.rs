//! Assorted handle-like Win32 types.

use std::ffi::c_void;

handle_type! {
	/// Windows message
	/// [`parameter`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#wparam).
	WPARAM
}

handle_type! {
	/// Windows message
	/// [`parameter`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#lparam).
	LPARAM
}