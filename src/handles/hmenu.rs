#![allow(non_snake_case)]

use crate::{BitmapPtrStr, IdMenu, IdPos};
use crate::{MENUINFO, MENUITEMINFO};
use crate::co;
use crate::ffi::{HANDLE, user32};
use crate::GetLastError;
use crate::handles::macros::{const_void, mut_void};
use crate::HWND;
use crate::Utf16;

handle_type! {
	/// Handle to a
	/// [menu](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
	/// Exposes methods.
	HMENU
}

impl HMENU {
	/// [`AppendMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// method.
	pub fn AppendMenu(self, uFlags: co::MF,
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> Result<(), co::ERROR>
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

	/// [`CheckMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// method.
	pub fn CheckMenuItem(
		self, uIDCheckItem: IdPos, uCheck: co::MF) -> Result<co::MF, ()>
	{
		match unsafe {
			user32::CheckMenuItem(self.0, uIDCheckItem.into(), uCheck.into())
		} {
			-1 => Err(()),
			ret => Ok(co::MF::from(ret as u32)),
		}
	}

	/// [`CreateMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// static method.
	pub fn CreateMenu() -> Result<HMENU, co::ERROR> {
		match ptr_as_opt!(user32::CreateMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	pub fn CreatePopupMenu() -> Result<HMENU, co::ERROR> {
		match ptr_as_opt!(user32::CreatePopupMenu()) {
			Some(p) => Ok(Self(p)),
			None => Err(GetLastError()),
		}
	}

	/// [`DeleteMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// method.
	pub fn DeleteMenu(
		self, uPosition: IdPos, uFlags: co::MF) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::DeleteMenu(self.0, uPosition.into(), uFlags.into())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`DestroyMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// method.
	pub fn DestroyMenu(self) -> Result<(), co::ERROR>
	{
		match unsafe { user32::DestroyMenu(self.0) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`EnableMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// method.
	pub fn EnableMenuItem(
		self, uIDEnableItem: IdPos, uEnable: co::MF) -> Result<co::MF, ()>
	{
		match unsafe {
			user32::EnableMenuItem(self.0, uIDEnableItem.into(), uEnable.into())
		} {
			-1 => Err(()),
			ret => Ok(co::MF::from(ret as u32)),
		}
	}

	/// [`GetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// method.
	pub fn GetMenuInfo(self, lpmi: &mut MENUINFO) -> Result<(), co::ERROR>
	{
		match unsafe { user32::GetMenuInfo(self.0, mut_void(lpmi)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
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
		ptr_as_opt!(
			user32::GetSubMenu(self.0, nPos as i32)
		).map(|p| Self(p))
	}

	/// [`InsertMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuw)
	/// method.
	pub fn InsertMenu(self, uPosition: IdPos, uFlags: co::MF,
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> Result<(), co::ERROR>
	{
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

	/// [`InsertMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// method.
	pub fn InsertMenuItem(self, item: IdPos,
		fByPosition: bool, lpmi: &MENUITEMINFO) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::InsertMenuItemW(
				self.0, item.into(), fByPosition as u32, const_void(lpmi))
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`IsMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// method.
	pub fn IsMenu(self) -> bool {
		unsafe { user32::IsMenu(self.0) != 0 }
	}

	/// [`RemoveMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// method.
	pub fn RemoveMenu(
		self, uPosition: IdPos, uFlags: co::MF) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::RemoveMenu(self.0, uPosition.into(), uFlags.into())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	pub fn SetMenuInfo(self, mii: &MENUINFO) -> Result<(), co::ERROR>
	{
		match unsafe { user32::SetMenuInfo(self.0, const_void(mii)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetMenuItemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// method.
	pub fn SetMenuItemInfo(self, item: IdPos,
		fByPosition: bool, lpmii: &MENUITEMINFO) -> Result<(), co::ERROR>
	{
		match unsafe {
			user32::SetMenuItemInfo(
				self.0, item.into(), fByPosition as u32, const_void(lpmii))
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`TrackPopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method
	pub fn TrackPopupMenu(self, uFlags: co::TPM,
		x: i32, y: i32, hWnd: HWND) -> Result<Option<i32>, co::ERROR>
	{
		let ret = unsafe {
			user32::TrackPopupMenu(self.0, uFlags.into(),
			x, y, 0, hWnd.as_ptr(), std::ptr::null())
		};

		if (uFlags & co::TPM::RETURNCMD) != co::TPM::default() {
			match ret {
				0 => match GetLastError() {
					co::ERROR::SUCCESS => Ok(None), // success, user cancelled the menu
					error => Err(error),
				},
				id => Ok(Some(id)), // success, menu item identifier
			}
		} else {
			match ret {
				0 => Err(GetLastError()),
				_ => Ok(None),
			}
		}
	}
}