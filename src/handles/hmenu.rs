#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{BitmapPtrStr, EntrySeparatorSubmenu, IdMenu, IdPos};
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::HWND;
use crate::msg;
use crate::privs::bool_to_winresult;
use crate::structs::{MENUINFO, MENUITEMINFO, POINT};
use crate::various::WString;

pub_struct_handle! {
	/// Handle to a
	/// [menu](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
	HMENU
}

impl HMENU {
	/// [`AppendMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`AppendMenuEnum`](crate::HMENU::AppendMenuEnum).
	pub fn AppendMenu(self, uFlags: co::MF,
		uIDNewItem: IdMenu, lpNewItem: BitmapPtrStr) -> WinResult<()>
	{
		let mut buf_lpNewItem = WString::default();
		bool_to_winresult(
			unsafe {
				user32::AppendMenuW(
					self.ptr,
					uFlags.0,
					uIDNewItem.into(),
					lpNewItem.as_ptr(&mut buf_lpNewItem),
				)
			},
		)
	}

	/// A more convenient [`AppendMenu`](crate::HMENU::AppendMenu).
	///
	/// # Examples
	///
	/// Adding a new menu entry, with its command ID:
	///
	/// ```rust,ignore
	/// use winsafe::{EntrySeparatorSubmenu, HMENU};
	///
	/// let my_hmenu: HMENU; // initialized somewhere
	///
	/// const ID_FILE_OPEN: i32 = 2001;
	///
	/// my_hmenu.AppendMenuEnum(
	///    &EntrySeparatorSubmenu::Entry(ID_FILE_OPEN, "&Open"),
	/// ).unwrap();
	/// ```
	///
	/// Adding multiple entries at once:
	///
	/// ```rust,ignore
	/// use winsafe::{EntrySeparatorSubmenu, HMENU};
	///
	/// let my_hmenu: HMENU; // initialized somewhere
	///
	/// const ID_FILE_OPEN: i32 = 2001;
	/// const ID_FILE_SAVE: i32 = 2002;
	/// const ID_FILE_EXIT: i32 = 2003;
	///
	/// [
	///     EntrySeparatorSubmenu::Entry(ID_FILE_OPEN, "&Open"),
	///     EntrySeparatorSubmenu::Entry(ID_FILE_OPEN, "&Save"),
	///     EntrySeparatorSubmenu::Separator,
	///     EntrySeparatorSubmenu::Entry(ID_FILE_EXIT, "E&xit"),
	/// ].iter()
	///     .for_each(|e| file_menu.AppendMenuEnum(e).unwrap());
	/// ```
	pub fn AppendMenuEnum(self, item: &EntrySeparatorSubmenu) -> WinResult<()> {
		match item {
			EntrySeparatorSubmenu::Entry(cmd_id, text) => self.AppendMenu(
				co::MF::STRING,
				IdMenu::Id(*cmd_id),
				BitmapPtrStr::Str((*text).to_owned()),
			),
			EntrySeparatorSubmenu::Separator => self.AppendMenu(
				co::MF::SEPARATOR,
				IdMenu::None,
				BitmapPtrStr::None,
			),
			EntrySeparatorSubmenu::Submenu(hmenu, text) => self.AppendMenu(
				co::MF::POPUP,
				IdMenu::Menu(*hmenu),
				BitmapPtrStr::Str((*text).to_owned()),
			),
		}
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
	/// **Note:** If not attached to a window, must be paired with an
	/// [`HMENU::DestroyMenu`](crate::HMENU::DestroyMenu) call.
	pub fn CreateMenu() -> WinResult<HMENU> {
		unsafe { user32::CreateMenu().as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`CreatePopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with an
	/// [`HMENU::DestroyMenu`](crate::HMENU::DestroyMenu) call.
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
	/// # Examples
	///
	/// Disabling a menu item:
	///
	/// ```rust,ignore
	/// use winsafe::{HMENU, IdPos};
	///
	/// const ID_FILE_OPEN: i32 = 101;
	///
	/// let hmenu: HMENU; // initialized somewhere
	///
	/// hmenu.EnableMenuItem(
	///     IdPos::Id(ID_FILE_OPEN),
	///     false,
	/// ).unwrap();
	/// ```
	///
	/// Disabling multiple menu items at once:
	///
	/// ```rust,ignore
	/// use winsafe::{HMENU, IdPos};
	///
	/// const ID_FILE_OPEN: i32 = 201;
	/// const ID_FILE_SAVE: i32 = 202;
	///
	/// let hmenu: HMENU; // initialized somewhere
	///
	/// [
	///     (ID_FILE_OPEN, "Open\tCtrl+O"),
	///     (ID_FILE_SAVE, "&Save"),
	/// ].iter()
	///     .for_each(|(id, txt)| hmenu.EnableMenuItem(
	///         IdPos::Id(*id),
	///         false,
	///     ).unwrap());
	/// ```
	pub fn EnableMenuItem(self,
		uIDEnableItem: IdPos, uEnable: bool) -> WinResult<co::MF>
	{
		match unsafe {
			user32::EnableMenuItem(
				self.ptr,
				uIDEnableItem.id_or_pos_u32(),
				(uIDEnableItem.mf_flag()
					| if uEnable { co::MF::ENABLED } else { co::MF::DISABLED }).0,
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
			unsafe { user32::GetMenuInfo(self.ptr, lpmi as *mut _ as _) },
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
					lpmi as *const _ as _,
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

	/// [`SetMenuDefaultItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenudefaultitem)
	/// method.
	pub fn SetMenuDefaultItem(self, uItem: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::SetMenuDefaultItem(
					self.ptr,
					uItem.id_or_pos_u32(),
					uItem.is_by_pos() as _,
				)
			},
		)
	}

	/// [`SetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	pub fn SetMenuInfo(self, mii: &MENUINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetMenuInfo(self.ptr, mii as *const _ as _) },
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
					lpmii as *const _ as _,
				)
			},
		)
	}

	/// [`TrackPopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method.
	///
	/// **Note:** If you just want to display a popup menu, consider the simpler
	/// [`TrackPopupMenuAtPoint`](crate::HMENU::TrackPopupMenuAtPoint).
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

	/// Shows the popup menu anchored at the given coordinates using
	/// [`TrackPopupMenu`](crate::HMENU::TrackPopupMenu), and performs other
	/// needed operations.
	///
	/// This method will block until the menu disappears.
	pub fn TrackPopupMenuAtPoint(self,
		mut pos: POINT, hParent: HWND, hCoordsRelativeTo: HWND) -> WinResult<()>
	{
		hCoordsRelativeTo.ClientToScreen(&mut pos)?; // now relative to screen
		hParent.SetForegroundWindow();
		self.TrackPopupMenu(co::TPM::LEFTBUTTON, pos.x, pos.y, hParent)?;
		hParent.PostMessage(msg::wm::Null {})?; // necessary according to TrackPopupMenu docs
		Ok(())
	}
}
