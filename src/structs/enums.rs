#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ATOM, HBITMAP, HMENU, Utf16};

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
/// Used in:
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `lpNewItem`.
pub enum BitmapOrStrOrParam<'a> {
	Bitmap(HBITMAP),
	Str(&'a str),
	Param(*const c_void),
}

impl<'a> BitmapOrStrOrParam<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			BitmapOrStrOrParam::Bitmap(hbmp) => unsafe { hbmp.as_ptr() as *const u16 },
			BitmapOrStrOrParam::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
			BitmapOrStrOrParam::Param(lp) => *lp as *const u16,
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdOrMenu {
	Id(i32),
	Menu(HMENU),
	None,
}

impl IdOrMenu {
	/// Converts the internal value to a pointer.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			IdOrMenu::Id(id) => *id as *const c_void,
			IdOrMenu::Menu(hMenu) => unsafe { hMenu.as_ptr() },
			IdOrMenu::None => std::ptr::null(),
		}
	}
}

//------------------------------------------------------------------------------

/// Used in:
/// * [`CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HiliteMenuItem`](crate::HMENU::HiliteMenuItem) `uIDHiliteItem`;
/// * [`InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`.
pub enum IdOrPos {
	Id(i32),
	Pos(u32),
}

impl From<IdOrPos> for u32 {
	fn from(v: IdOrPos) -> u32 {
		match v {
			IdOrPos::Id(id) => id as u32,
			IdOrPos::Pos(pos) => pos,
		}
	}
}