#![allow(non_snake_case)]

use crate::{BitmapPtrStr, IdMenu, IdPos};
use crate::co;
use crate::ffi::{HANDLE, user32};
use crate::GetLastError;
use crate::Utf16;

handle_type! {
	/// Handle to a
	/// [menu](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
	HMENU
}

impl HMENU {
	/// [`AppendMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// method.
	pub fn AppendMenu(
		self,
		uFlags: co::MF,
		uIDNewItem: IdMenu,
		lpNewItem: BitmapPtrStr,
) -> Result<(), co::ERROR>
	{
		let mut buf16 = Utf16::default();

		match unsafe {
			user32::AppendMenuW(
				self.0,
				uFlags.into(),
				uIDNewItem.as_ptr(),
				lpNewItem.as_ptr(&mut buf16),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`CreateMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// static method.
	pub fn CreateMenu() -> Result<HMENU, co::ERROR> {
		match ptr_to_opt!(user32::CreateMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	pub fn CreatePopupMenu() -> Result<HMENU, co::ERROR> {
		match ptr_to_opt!(user32::CreatePopupMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`GetMenuItemCount`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// method.
	pub fn GetMenuItemCount(self) -> Result<u32, co::ERROR> {
		match unsafe { user32::GetMenuItemCount(self.0) } {
			-1 => Err(GetLastError()),
			count => Ok(count as u32),
		}
	}

	/// [`GetMenuItemID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// method.
	pub fn GetMenuItemID(self, nPos: i32) -> Option<i32> {
		match unsafe { user32::GetMenuItemID(self.0, nPos) } {
			-1 => None,
			id => Some(id),
		}
	}

	/// [`GetSubMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// method.
	pub fn GetSubMenu(self, nPos: u32) -> Option<HMENU> {
		ptr_to_opt!(
			user32::GetSubMenu(self.0, nPos as i32)
		).map(|p| Self(p))
	}

	/// [`InsertMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuw)
	/// method.
	pub fn InsertMenu(
		self,
		uPosition: IdPos,
		uFlags: co::MF,
		uIDNewItem: IdMenu,
		lpNewItem: BitmapPtrStr,
	) -> Result<(), co::ERROR> {
		let mut buf16 = Utf16::default();

		match unsafe {
			user32::InsertMenuW(
				self.0,
				uPosition.into(),
				uFlags.into(),
				uIDNewItem.as_ptr(),
				lpNewItem.as_ptr(&mut buf16),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}