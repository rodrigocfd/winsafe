//! Variant types needed for some Win32 functions.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::handles::{HBITMAP, HMENU};
use crate::structs::ATOM;
use crate::Utf16;

/// Variant parameter used in window class functions.
///
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
pub enum AtomStr<'a> {
	Atom(ATOM),
	Str(&'a str),
}

impl<'a> AtomStr<'a> {
	/// [`MAKEINTRESOURCE`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
	/// macro. Uses an external [`Utf16`](crate::Utf16) buffer to keep the
	/// string, if needed.
	pub fn MAKEINTRESOURCE(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			AtomStr::Str(name) => {
				*buf16 = Utf16::from_str(name); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			}
			AtomStr::Atom(atom) => atom.as_ptr(),
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in menu functions.
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `lpNewItem`.
pub enum BitmapPtrStr<'a> {
	Bitmap(HBITMAP),
	Str(&'a str),
	Param(*const c_void),
}

impl<'a> BitmapPtrStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			BitmapPtrStr::Bitmap(hbmp) => unsafe { hbmp.as_ptr() as *const u16 },
			BitmapPtrStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
			BitmapPtrStr::Param(lp) => *lp as *const u16,
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for [`LoadCursor`](crate::HINSTANCE::LoadCursor)
/// `lpCursorName`.
pub enum IdIdcStr<'a> {
	Id(i32),
	Idc(co::IDC),
	Str(&'a str),
}

impl<'a> IdIdcStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			IdIdcStr::Id(id) => *id as *const u16,
			IdIdcStr::Idc(idc) => usize::from(*idc) as *const u16,
			IdIdcStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.
pub enum IdIdiStr<'a> {
	Id(i32),
	Idi(co::IDI),
	Str(&'a str),
}

impl<'a> IdIdiStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			IdIdiStr::Id(id) => *id as *const u16,
			IdIdiStr::Idi(idi) => usize::from(*idi) as *const u16,
			IdIdiStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in menu functions.
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdMenu {
	Id(i32),
	Menu(HMENU),
	None,
}

impl IdMenu {
	/// Converts the internal value to a pointer.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			IdMenu::Id(id) => *id as *const c_void,
			IdMenu::Menu(hMenu) => unsafe { hMenu.as_ptr() },
			IdMenu::None => std::ptr::null(),
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in menu functions.
///
/// * [`CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HiliteMenuItem`](crate::HWND::HiliteMenuItem) `uIDHiliteItem`;
/// * [`InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`.
pub enum IdPos {
	Id(i32),
	Pos(u32),
}

impl From<IdPos> for u32 {
	fn from(v: IdPos) -> u32 {
		match v {
			IdPos::Id(id) => id as u32,
			IdPos::Pos(pos) => pos,
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for
/// [`LoadAccelerators`](crate::HINSTANCE::LoadAccelerators) `lpTableName`.
pub enum IdStr<'a> {
	Id(i32),
	Str(&'a str),
}

impl<'a> IdStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			IdStr::Id(id) => *id as *const u16,
			IdStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
		}
	}
}

//------------------------------------------------------------------------------

/// Variant value returned by [`RegQueryValueEx`](crate::HKEY::RegQueryValueEx).
pub enum RegistryValue {
	Binary(Vec<u8>),
	Dword(u32),
	Qword(u64),
	Sz(String),
	None,
}

impl RegistryValue {
	/// Converts the internal value to a pointer.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			RegistryValue::Binary(b) => b.as_ptr() as *const c_void,
			RegistryValue::Dword(n) => *n as *const c_void,
			RegistryValue::Qword(n) => *n as *const c_void,
			RegistryValue::Sz(s) => {
				unsafe { Utf16::from_str(&s).as_ptr() as *const c_void }
			},
			RegistryValue::None => std::ptr::null(),
		}
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	pub fn reg_type(&self) -> co::REG {
		match self {
			RegistryValue::Binary(_) => co::REG::BINARY,
			RegistryValue::Dword(_) => co::REG::DWORD,
			RegistryValue::Qword(_) => co::REG::QWORD,
			RegistryValue::Sz(_) => co::REG::SZ,
			RegistryValue::None => co::REG::NONE,
		}
	}

	/// Returns the length of the stored data.
	pub fn len(&self) -> usize {
		match self {
			RegistryValue::Binary(b) => b.len(),
			RegistryValue::Dword(_) => std::mem::size_of::<u32>(),
			RegistryValue::Qword(_) => std::mem::size_of::<u64>(),
			RegistryValue::Sz(s) => {
				(s.chars().count() + 1) * std::mem::size_of::<u16>() // including terminating null
			},
			RegistryValue::None => 0,
		}
	}
}