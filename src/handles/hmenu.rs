#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{BitmapPtrStr, IdMenu, IdPos};
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::privs::ptr_as_opt;
use crate::structs::{MENUINFO, MENUITEMINFO};

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
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> WinResult<()>
	{
		match unsafe {
			user32::AppendMenuW(
				self.ptr,
				uFlags.into(),
				uIDNewItem.into(),
				lpNewItem.as_ptr(),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`CheckMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// method.
	pub fn CheckMenuItem(self,
		uIDCheckItem: IdPos, uCheck: bool) -> WinResult<co::MF>
	{
		let mut flags = if uCheck {
			co::MF::CHECKED
		} else {
			co::MF::UNCHECKED
		};

		match uIDCheckItem {
			IdPos::Id(_) => flags |= co::MF::BYCOMMAND,
			IdPos::Pos(_) => flags |= co::MF::BYPOSITION,
		}

		match unsafe {
			user32::CheckMenuItem(self.ptr, uIDCheckItem.into(), flags.into())
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(co::MF(ret as u32)),
		}
	}

	/// [`CreateMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with a
	/// [`DestroyMenu`](crate::HMENU::DestroyMenu) call.
	pub fn CreateMenu() -> WinResult<HMENU> {
		match ptr_as_opt(unsafe { user32::CreateMenu() }) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with a
	/// [`DestroyMenu`](crate::HMENU::DestroyMenu) call.
	pub fn CreatePopupMenu() -> WinResult<HMENU> {
		match ptr_as_opt(unsafe { user32::CreatePopupMenu() }) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`DeleteMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// method.
	pub fn DeleteMenu(self, uPosition: IdPos) -> WinResult<()> {
		let flags = match uPosition {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		};

		match unsafe {
			user32::DeleteMenu(self.ptr, uPosition.into(), flags.into())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`DestroyMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// method.
	pub fn DestroyMenu(self) -> WinResult<()> {
		match unsafe { user32::DestroyMenu(self.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`EnableMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// method.
	///
	/// You don't need to pass `MF::BYCOMMAND` or `MF::BYPOSITION` flags, they
	/// are inferred by [`IdPos`](crate::IdPos).
	pub fn EnableMenuItem(self,
		uIDEnableItem: IdPos, uEnable: co::MF) -> WinResult<co::MF>
	{
		let mut flags = uEnable;
		flags &= !(co::MF::BYPOSITION | co::MF::BYCOMMAND); // remove if set
		flags |= match uIDEnableItem {
			IdPos::Id(_) => co::MF::BYCOMMAND, // set correctly
			IdPos::Pos(_) => co::MF::BYPOSITION,
		};

		match unsafe {
			user32::EnableMenuItem(self.ptr, uIDEnableItem.into(), flags.into())
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(co::MF(ret as u32)),
		}
	}

	/// [`GetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// method.
	pub fn GetMenuInfo(self, lpmi: &mut MENUINFO) -> WinResult<()> {
		match unsafe { user32::GetMenuInfo(self.ptr, lpmi as *mut _ as *mut _) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetMenuItemCount`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// method.
	pub fn GetMenuItemCount(self) -> WinResult<u32> {
		match unsafe { user32::GetMenuItemCount(self.ptr) } {
			-1 => Err(GetLastError()),
			count => Ok(count as u32),
		}
	}

	/// [`GetMenuItemID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// method.
	pub fn GetMenuItemID(self, nPos: i32) -> Option<i32> {
		match unsafe { user32::GetMenuItemID(self.ptr, nPos) } {
			-1 => None,
			id => Some(id),
		}
	}

	/// [`GetSubMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// method.
	pub fn GetSubMenu(self, nPos: u32) -> Option<HMENU> {
		ptr_as_opt(
			unsafe { user32::GetSubMenu(self.ptr, nPos as i32) },
		).map(|ptr| Self { ptr })
	}

	/// [`InsertMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuw)
	/// method.
	///
	/// You don't need to pass `MF::BYCOMMAND` or `MF::BYPOSITION` flags, they
	/// are inferred by [`IdPos`](crate::IdPos).
	pub fn InsertMenu(self, uPosition: IdPos, uFlags: co::MF,
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> WinResult<()>
	{
		let mut flags = uFlags;
		flags &= !(co::MF::BYPOSITION | co::MF::BYCOMMAND); // remove if set
		flags |= match uPosition {
			IdPos::Id(_) => co::MF::BYCOMMAND, // set correctly
			IdPos::Pos(_) => co::MF::BYPOSITION,
		};

		match unsafe {
			user32::InsertMenuW(
				self.ptr,
				uPosition.into(),
				flags.into(),
				uIDNewItem.into(),
				lpNewItem.as_ptr(),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`InsertMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// method.
	pub fn InsertMenuItem(self,
		item: IdPos, lpmi: &MENUITEMINFO) -> WinResult<()>
	{
		let byPos = match item {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		};

		match unsafe {
			user32::InsertMenuItemW(
				self.ptr,
				item.into(),
				byPos as i32,
				lpmi as *const _ as *const _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`IsMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// method.
	pub fn IsMenu(self) -> bool {
		unsafe { user32::IsMenu(self.ptr) != 0 }
	}

	/// [`RemoveMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// method.
	pub fn RemoveMenu(self, uPosition: IdPos) -> WinResult<()> {
		let byPos = match uPosition {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		};

		match unsafe {
			user32::RemoveMenu(self.ptr, uPosition.into(), byPos as u32)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	pub fn SetMenuInfo(self, mii: &MENUINFO) -> WinResult<()> {
		match unsafe {
			user32::SetMenuInfo(self.ptr, mii as *const _ as *const _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetMenuItemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// method.
	pub fn SetMenuItemInfo(self,
		item: IdPos, lpmii: &MENUITEMINFO) -> WinResult<()>
	{
		let byPos = match item {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		};

		match unsafe {
			user32::SetMenuItemInfoW(
				self.ptr, item.into(), byPos as i32, lpmii as *const _ as *const _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`TrackPopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method
	pub fn TrackPopupMenu(self, uFlags: co::TPM,
		x: i32, y: i32, hWnd: HWND) -> WinResult<Option<i32>>
	{
		let ret = unsafe {
			user32::TrackPopupMenu(
				self.ptr, uFlags.into(), x, y, 0, hWnd.ptr, std::ptr::null(),
			)
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
