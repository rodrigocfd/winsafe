#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{BmpPtrStr, IdMenu, IdPos, MenuEnum};
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HWND};
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
	/// [`HMENU::AppendMenuEnum`](crate::HMENU::AppendMenuEnum).
	pub fn AppendMenu(self, flags: co::MF,
		new_item: IdMenu, content: BmpPtrStr) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::AppendMenuW(
					self.ptr,
					flags.0,
					new_item.as_usize(),
					content.as_ptr(),
				)
			},
		)
	}

	/// A more convenient [`HMENU::AppendMenu`](crate::HMENU::AppendMenu).
	///
	/// # Examples
	///
	/// Adding a new menu entry, with its command ID:
	///
	/// ```rust,ignore
	/// use winsafe::{HMENU, MenuEnum};
	///
	/// let my_hmenu: HMENU; // initialized somewhere
	///
	/// const ID_FILE_OPEN: i32 = 2001;
	///
	/// my_hmenu.AppendMenuEnum(
	///    &MenuEnum::Entry(ID_FILE_OPEN, "&Open"),
	/// )?;
	/// ```
	///
	/// Adding multiple entries at once:
	///
	/// ```rust,ignore
	/// use winsafe::{HMENU, MenuEnum};
	///
	/// let my_hmenu: HMENU; // initialized somewhere
	///
	/// const ID_FILE_OPEN: i32 = 2001;
	/// const ID_FILE_SAVE: i32 = 2002;
	/// const ID_FILE_EXIT: i32 = 2003;
	///
	/// [
	///     MenuEnum::Entry(ID_FILE_OPEN, "&Open"),
	///     MenuEnum::Entry(ID_FILE_OPEN, "&Save"),
	///     MenuEnum::Separator,
	///     MenuEnum::Entry(ID_FILE_EXIT, "E&xit"),
	/// ].iter()
	///     .for_each(|e| file_menu.AppendMenuEnum(e)?);
	/// ```
	pub fn AppendMenuEnum(self, item: &MenuEnum) -> WinResult<()> {
		match item {
			MenuEnum::Entry(cmd_id, text) => self.AppendMenu(
				co::MF::STRING,
				IdMenu::Id(*cmd_id),
				BmpPtrStr::from_str(*text),
			),
			MenuEnum::Separator => self.AppendMenu(
				co::MF::SEPARATOR,
				IdMenu::None,
				BmpPtrStr::None,
			),
			MenuEnum::Submenu(hmenu, text) => self.AppendMenu(
				co::MF::POPUP,
				IdMenu::Menu(*hmenu),
				BmpPtrStr::from_str(*text),
			),
		}
	}

	/// [`CheckMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// method.
	pub fn CheckMenuItem(self,
		id_or_pos: IdPos, check: bool) -> WinResult<co::MF>
	{
		match unsafe {
			user32::CheckMenuItem(
				self.ptr,
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag() | if check {
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

	/// [`CheckMenuRadioItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuradioitem)
	/// method.
	///
	/// # Panics
	///
	/// Panics if `first`, `last` and `check` don't use the same enum field.
	pub fn CheckMenuRadioItem(self,
		first: IdPos, last: IdPos, check: IdPos) -> WinResult<()>
	{
		if !(first.is_by_pos() == last.is_by_pos()
			&& last.is_by_pos() == check.is_by_pos())
		{
			panic!("Different enum fields.");
		}

		bool_to_winresult(
			unsafe {
				user32::CheckMenuRadioItem(
					self.ptr,
					first.id_or_pos_u32(),
					last.id_or_pos_u32(),
					check.id_or_pos_u32(),
					check.mf_flag().0,
				)
			},
		)
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
	pub fn DeleteMenu(self, id_or_pos: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::DeleteMenu(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().0,
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
	/// )?;
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
	///     .map(|(id, txt)| hmenu.EnableMenuItem(
	///         IdPos::Id(*id),
	///         false,
	///     ))
	///     .collect::<Result<Vec<co::MF>, _>>()?;
	/// ```
	pub fn EnableMenuItem(self,
		id_or_pos: IdPos, enable: bool) -> WinResult<co::MF>
	{
		match unsafe {
			user32::EnableMenuItem(
				self.ptr,
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag()
					| if enable { co::MF::ENABLED } else { co::MF::DISABLED }).0,
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(co::MF(ret as _)),
		}
	}

	/// [`GetMenuDefaultItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenudefaultitem)
	/// method.
	pub fn GetMenuDefaultItem(self,
		by_pos: bool, flags: co::GMDI) -> WinResult<IdPos>
	{
		match unsafe {
			user32::GetMenuDefaultItem(self.ptr, by_pos as _, flags.0) as i32
		} {
			-1 => Err(GetLastError()),
			n => Ok(if by_pos { IdPos::Pos(n as _) } else { IdPos::Id(n as _) }),
		}
	}

	/// [`GetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// method.
	pub fn GetMenuInfo(self, mi: &mut MENUINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetMenuInfo(self.ptr, mi as *mut _ as _) },
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

	/// [`GetMenuItemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuiteminfow)
	/// method.
	pub fn GetMenuItemInfo(self,
		id_or_pos: IdPos, mii: &mut MENUITEMINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetMenuItemInfoW(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuItemID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// method.
	pub fn GetMenuItemID(self, nPos: i32) -> Option<i32> {
		match unsafe { user32::GetMenuItemID(self.ptr, nPos) } {
			-1 => None,
			id => Some(id),
		}
	}

	/// [`GetMenuState`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustate)
	/// method.
	pub fn GetMenuState(self, id_or_pos: IdPos) -> WinResult<co::MF> {
		match unsafe {
			user32::GetMenuState(
				self.ptr,
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
			) as i32
		} {
			-1 => Err(GetLastError()),
			mf => Ok(co::MF(mf as _)),
		}
	}

	/// [`GetMenuString`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustringw)
	/// method.
	pub fn GetMenuString(self, id_or_pos: IdPos) -> WinResult<String> {
		const BLOCK: usize = 64; // arbitrary
		let mut buf_sz = BLOCK;

		let mut buf = WString::default();

		loop {
			buf.realloc_buffer(buf_sz);

			let nchars = match unsafe {
				user32::GetMenuStringW(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					buf.as_mut_ptr(),
					buf.buffer_size() as _,
					id_or_pos.mf_flag().0,
				)
			} {
				0 => return Err(GetLastError()),
				n => n,
			};

			if (nchars as usize) + 1 < buf_sz { // to break, must have at least 1 char gap
				break;
			}

			buf_sz += BLOCK; // increase buffer size to try again
		}

		Ok(buf.to_string())
	}

	/// [`GetSubMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// method.
	pub fn GetSubMenu(self, pos: u32) -> Option<HMENU> {
		unsafe { user32::GetSubMenu(self.ptr, pos as _).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`InsertMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// method.
	pub fn InsertMenuItem(self,
		id_or_pos: IdPos, mii: &MENUITEMINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::InsertMenuItemW(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
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
	pub fn RemoveMenu(self, id_or_pos: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::RemoveMenu(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().0,
				)
			},
		)
	}

	/// [`SetMenuDefaultItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenudefaultitem)
	/// method.
	pub fn SetMenuDefaultItem(self, id_or_pos: IdPos) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::SetMenuDefaultItem(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
				)
			},
		)
	}

	/// [`SetMenuInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	pub fn SetMenuInfo(self, mi: &MENUINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetMenuInfo(self.ptr, mi as *const _ as _) },
		)
	}

	/// [`SetMenuItemBitmaps`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuitembitmaps)
	/// method.
	pub fn SetMenuItemBitmaps(self,
		id_or_pos: IdPos,
		hbmp_unchecked: Option<HBITMAP>,
		hbmp_checked: Option<HBITMAP>) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetMenuItemBitmaps(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().0,
					hbmp_unchecked.map_or(std::ptr::null_mut(), |h| h.ptr),
					hbmp_checked.map_or(std::ptr::null_mut(), |h| h.ptr),
				)
			},
		)
	}

	/// [`SetMenuItemInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// method.
	pub fn SetMenuItemInfo(self,
		id_or_pos: IdPos, mii: &MENUITEMINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetMenuItemInfoW(
					self.ptr,
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
				)
			},
		)
	}

	/// [`TrackPopupMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method.
	///
	/// **Note:** If you just want to display a popup menu, consider the simpler
	/// [`HMENU::TrackPopupMenuAtPoint`](crate::HMENU::TrackPopupMenuAtPoint).
	pub fn TrackPopupMenu(self,
		flags: co::TPM, location: POINT, hwnd: HWND) -> WinResult<Option<i32>>
	{
		let ret = unsafe {
			user32::TrackPopupMenu(
				self.ptr,
				flags.0,
				location.x, location.y,
				0,
				hwnd.ptr,
				std::ptr::null(),
			)
		};

		if (flags & co::TPM::RETURNCMD) != co::TPM::default() {
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
		pos: POINT,
		hwnd_parent: HWND,
		hwnd_coords_relative_to: HWND) -> WinResult<()>
	{
		let mut pos = pos;
		hwnd_coords_relative_to.ClientToScreen(&mut pos)?; // now relative to screen
		hwnd_parent.SetForegroundWindow();
		self.TrackPopupMenu(co::TPM::LEFTBUTTON, pos, hwnd_parent)?;
		hwnd_parent.PostMessage(msg::wm::Null {})?; // necessary according to TrackPopupMenu docs
		Ok(())
	}
}
