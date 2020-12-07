#![allow(non_snake_case)]

use crate::co;
use crate::ffi::{user32, Void};

handle_type! {
	/// Handle to a
	/// [menu](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
	HMENU
}

impl HMENU {
	/// [`CreateMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// function.
	pub fn CreateMenu() -> Result<HMENU, co::ERROR> {
		match ptr_to_opt!(user32::CreateMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// function.
	pub fn CreatePopupMenu() -> Result<HMENU, co::ERROR> {
		match ptr_to_opt!(user32::CreatePopupMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(co::ERROR::GetLastError()),
		}
	}
}