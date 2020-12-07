#![allow(non_snake_case)]

use crate::{ATOM, HMENU, Utf16};
use crate::ffi::{Void};

/// Wraps a variant parameter.
///
/// Used in:
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
pub enum AtomOrStr<'a> {
	Atom(ATOM),
	Str(&'a str),
}

impl<'a> AtomOrStr<'a> {
	/// [`MAKEINTRESOURCE`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
	/// macro. Uses an external [`Utf16`](crate::Utf16) buffer to keep the
	/// string, if needed.
	pub fn MAKEINTRESOURCE(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			AtomOrStr::Str(name) => {
				*buf16 = Utf16::from_str(name); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			}
			AtomOrStr::Atom(atom) => atom.as_ptr(),
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdOrMenu {
	Id(i32),
	Menu(HMENU),
	None,
}

impl IdOrMenu {
	/// Useful to pass as [`HMENU`](crate::HMENU).
	pub fn as_ptr(&self) -> *const Void {
		match self {
			IdOrMenu::Id(id) => *id as *const Void,
			IdOrMenu::Menu(hMenu) => unsafe { hMenu.as_ptr() },
			IdOrMenu::None => std::ptr::null(),
		}
	}
}