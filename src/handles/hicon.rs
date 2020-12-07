#![allow(non_snake_case)]

use crate::ffi::{user32, Void};

handle_type! {
	/// Handle to an
	/// [icon](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hicon).
	HICON
}

impl HICON {
	/// [`DestroyIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
	/// method.
	pub fn DestroyIcon(&self) {
		unsafe { user32::DestroyIcon(self.0); }
	}
}