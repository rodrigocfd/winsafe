#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{BitmapPtrStr, IdMenu, IdPos};
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::privs::{bool_to_winresult, ref_as_pcvoid, ref_as_pvoid};
use crate::structs::{MENUINFO, MENUITEMINFO};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [menu](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
	HMENU
}

impl HMENU {
	/// [`AppendMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// method.
	///
	/// This method is rather tricky, consider using the wrappers:
	///
	/// * [`AppendMenuItem`](crate::HMENU::AppendMenuItem);
	/// * [`AppendMenuSeparator`](crate::HMENU::AppendMenuSeparator);
	/// * [`AppendMenuSubmenu`](crate::HMENU::AppendMenuSubmenu).
	pub fn AppendMenu(self, uFlags: co::MF,
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::AppendMenuW(
					self.ptr,
					uFlags.0,
					uIDNewItem.into(),
					lpNewItem.as_ptr(),
				)
			},
		)
	}

	/// A more convenient [`AppendMenu`](crate::HMENU::AppendMenu), which
	/// appends a new item with its command ID.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HMENU;
	///
	/// const ID_FILE_OPEN: u16 = 101;
	///
	/// let hmenu = HMENU::CreatePopupMenu().unwrap();
	///
	/// hmenu.AppendMenuItem(ID_FILE_OPEN, "&Open file...")
	///     .unwrap();
	/// ```
	pub fn AppendMenuItem(self, command_id: u16, text: &str) -> WinResult<()> {
		self.AppendMenu(
			co::MF::STRING,
			IdMenu::Id(command_id),
			BitmapPtrStr::Str(WString::from_str(text)),
		)
	}

	/// A more convenient [`AppendMenu`](crate::HMENU::AppendMenu), which
	/// appends multiple new items with their command IDs.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HMENU;
	///
	/// const ID_FILE_OPEN: u16 = 101;
	/// const ID_FILE_SAVE: u16 = 102;
	///
	/// let hmenu = HMENU::CreatePopupMenu().unwrap();
	///
	/// hmenu.AppendMenuItems(&[
	///     (ID_FILE_OPEN, "&Open file..."),
	///     (ID_FILE_SAVE, "&Save file"),
	/// ]).unwrap();
	/// ```
	pub fn AppendMenuItems(self,
		command_ids_and_texts: &[(u16, &str)]) -> WinResult<()>
	{
		for (command_id, text) in command_ids_and_texts.iter() {
			self.AppendMenuItem(*command_id, text)?;
		}
		Ok(())
	}

	/// A more convenient [`AppendMenu`](crate::HMENU::AppendMenu), which
	/// appends a separator.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HMENU;
	///
	/// let hmenu = HMENU::CreatePopupMenu().unwrap();
	///
	/// hmenu.AppendSeparator().unwrap();
	/// ```
	pub fn AppendMenuSeparator(self) -> WinResult<()> {
		self.AppendMenu(co::MF::SEPARATOR, IdMenu::None, BitmapPtrStr::None)
	}

	/// A more convenient [`AppendMenu`](crate::HMENU::AppendMenu), which
	/// appends a menu as a new submenu entry.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HMENU;
	///
	/// let hmenu_file = HMENU::CreatePopupMenu().unwrap();
	/// let hmenu_edit = HMENU::CreatePopupMenu().unwrap();
	///
	/// let hmenu = HMENU::CreatePopupMenu().unwrap();
	///
	/// hmenu.AppendMenuSubmenu(hmenu_file, "&File").unwrap();
	/// hmenu.AppendMenuSubmenu(hmenu_edit, "&Edit").unwrap();
	/// ```
	pub fn AppendMenuSubmenu(self, submenu: HMENU, text: &str) -> WinResult<()> {
		self.AppendMenu(
			co::MF::POPUP,
			IdMenu::Menu(submenu),
			BitmapPtrStr::Str(WString::from_str(text)),
		)
	}

	/// [`CheckMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// method.
	pub fn CheckMenuItem(self,
		uIDCheckItem: IdPos, uCheck: bool) -> WinResult<co::MF>
	{
		match unsafe {
			user32::CheckMenuItem(
				self.ptr,
				uIDCheckItem.id_or_pos_u32(),
				(uIDCheckItem.mf_flag() | if uCheck {
					co::MF::CHECKED
				} else {
					co::MF::UNCHECKED
				}).0,
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(co::MF(ret as _)),
		}
	}

	/// [`CreateMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with a
	/// [`DestroyMenu`](crate::HMENU::DestroyMenu) call.
	pub fn CreateMenu() -> WinResult<HMENU> {
		unsafe { user32::CreateMenu().as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with a
	/// [`DestroyMenu`](crate::HMENU::DestroyMenu) call.
	pub fn CreatePopupMenu() -> WinResult<HMENU> {
		unsafe { user32::CreatePopupMenu().as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DeleteMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// method.
	pub fn DeleteMenu(self, uPosition: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::DeleteMenu(
					self.ptr,
					uPosition.id_or_pos_u32(),
					uPosition.mf_flag().0,
				)
			},
		)
	}

	/// [`DestroyMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// method.
	pub fn DestroyMenu(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DestroyMenu(self.ptr) })
	}

	/// [`EnableMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// method.
	///
	/// You don't need to pass [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) flags, they are inferred
	/// by [`IdPos`](crate::IdPos).
	pub fn EnableMenuItem(self,
		uIDEnableItem: IdPos, uEnable: co::MF) -> WinResult<co::MF>
	{
		let mut flags = uEnable;
		flags &= !(co::MF::BYPOSITION | co::MF::BYCOMMAND); // remove if set
		flags |= uIDEnableItem.mf_flag(); // set correctly

		match unsafe {
			user32::EnableMenuItem(
				self.ptr,
				uIDEnableItem.id_or_pos_u32(),
				flags.0,
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(co::MF(ret as _)),
		}
	}

	/// [`GetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// method.
	pub fn GetMenuInfo(self, lpmi: &mut MENUINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetMenuInfo(self.ptr, ref_as_pvoid(lpmi)) },
		)
	}

	/// [`GetMenuItemCount`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// method.
	pub fn GetMenuItemCount(self) -> WinResult<u32> {
		match unsafe { user32::GetMenuItemCount(self.ptr) } {
			-1 => Err(GetLastError()),
			count => Ok(count as _),
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
		unsafe { user32::GetSubMenu(self.ptr, nPos as _).as_mut() }
			.map(|ptr| Self { ptr })
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
		flags |= uPosition.mf_flag(); // set correctly

		bool_to_winresult(
			unsafe {
				user32::InsertMenuW(
					self.ptr,
					uPosition.id_or_pos_u32(),
					flags.0,
					uIDNewItem.into(),
					lpNewItem.as_ptr(),
				)
			},
		)
	}

	/// [`InsertMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// method.
	pub fn InsertMenuItem(self,
		item: IdPos, lpmi: &MENUITEMINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::InsertMenuItemW(
					self.ptr,
					item.id_or_pos_u32(),
					item.is_by_pos() as _,
					ref_as_pcvoid(lpmi),
				)
			},
		)
	}

	/// [`IsMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// method.
	pub fn IsMenu(self) -> bool {
		unsafe { user32::IsMenu(self.ptr) != 0 }
	}

	/// [`RemoveMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// method.
	pub fn RemoveMenu(self, uPosition: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::RemoveMenu(
					self.ptr,
					uPosition.id_or_pos_u32(),
					uPosition.mf_flag().0,
				)
			},
		)
	}

	/// [`SetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	pub fn SetMenuInfo(self, mii: &MENUINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetMenuInfo(self.ptr, ref_as_pcvoid(mii)) },
		)
	}

	/// [`SetMenuItemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// method.
	pub fn SetMenuItemInfo(self,
		item: IdPos, lpmii: &MENUITEMINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetMenuItemInfoW(
					self.ptr,
					item.id_or_pos_u32(),
					item.is_by_pos() as _,
					ref_as_pcvoid(lpmii),
				)
			},
		)
	}

	/// [`TrackPopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method
	pub fn TrackPopupMenu(self, uFlags: co::TPM,
		x: i32, y: i32, hWnd: HWND) -> WinResult<Option<i32>>
	{
		let ret = unsafe {
			user32::TrackPopupMenu(
				self.ptr, uFlags.0, x, y, 0, hWnd.ptr, std::ptr::null(),
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
