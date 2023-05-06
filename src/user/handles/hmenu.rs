#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, msg, user};
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::{
	bool_to_sysresult, ptr_to_option_handle, ptr_to_sysresult_handle,
};
use crate::prelude::{Handle, NativeBitflag, user_Hwnd};
use crate::user::decl::{
	BmpPtrStr, HBITMAP, HWND, IdMenu, IdPos, MenuEnum, MENUINFO, MENUITEMINFO,
	POINT,
};

impl_handle! { HMENU;
	/// Handle to a
	/// [menu](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
}

impl user_Hmenu for HMENU {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HMENU`](crate::HMENU).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hmenu: Handle {
	/// [`AppendMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`HMENU::AppendMenuEnum`](crate::prelude::user_Hmenu::AppendMenuEnum).
	fn AppendMenu(&self,
		flags: co::MF, new_item: IdMenu, content: BmpPtrStr) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::AppendMenuW(
					self.as_ptr(),
					flags.raw(),
					new_item.as_usize(),
					content.as_ptr(),
				)
			},
		)
	}

	/// A more convenient
	/// [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu).
	///
	/// # Examples
	///
	/// Adding multiple entries at once, with their command IDs:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HMENU, MenuEnum, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 2001;
	///     ID_FILE_SAVE
	///     ID_FILE_EXIT
	/// }
	///
	/// let hmenu: HMENU; // initialized somewhere
	/// # let hmenu = HMENU::NULL;
	///
	/// hmenu.AppendMenuEnum(&[
	///     MenuEnum::Entry(ID_FILE_OPEN, "&Open"),
	///     MenuEnum::Entry(ID_FILE_OPEN, "&Save"),
	///     MenuEnum::Separator,
	///     MenuEnum::Entry(ID_FILE_EXIT, "E&xit"),
	/// ])?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn AppendMenuEnum(&self, items: &[MenuEnum]) -> SysResult<()> {
		items.iter().map(|item| {
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
					IdMenu::Menu(hmenu),
					BmpPtrStr::from_str(*text),
				),
			}
		}).collect::<Result<Vec<_>, _>>()?;

		Ok(())
	}

	/// [`CheckMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// method.
	fn CheckMenuItem(&self,
		id_or_pos: IdPos, check: bool) -> SysResult<co::MF>
	{
		match unsafe {
			user::ffi::CheckMenuItem(
				self.as_ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag() | if check {
					co::MF::CHECKED
				} else {
					co::MF::UNCHECKED
				}).raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`CheckMenuRadioItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuradioitem)
	/// method.
	///
	/// # Panics
	///
	/// Panics if `first`, `last` and `check` don't use the same enum field.
	fn CheckMenuRadioItem(&self,
		first: IdPos, last: IdPos, check: IdPos) -> SysResult<()>
	{
		if !(first.is_by_pos() == last.is_by_pos()
			&& last.is_by_pos() == check.is_by_pos())
		{
			panic!("Different enum fields.");
		}

		bool_to_sysresult(
			unsafe {
				user::ffi::CheckMenuRadioItem(
					self.as_ptr(),
					first.id_or_pos_u32(),
					last.id_or_pos_u32(),
					check.id_or_pos_u32(),
					check.mf_flag().raw(),
				)
			},
		)
	}

	/// [`CreateMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// static method.
	///
	/// **Note:** If not attached to a window, must be paired with an
	/// [`HMENU::DestroyMenu`](crate::prelude::user_Hmenu::DestroyMenu) call.
	#[must_use]
	fn CreateMenu() -> SysResult<HMENU> {
		ptr_to_sysresult_handle(unsafe { user::ffi::CreateMenu() })
	}

	/// [`CreatePopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// static method.
	///
	/// **Note:** When a menu is attached to a window, it's automatically
	/// destroyed along with the window. However, if the menu is not attached to
	/// any window, you must call
	/// [`HMENU::DestroyMenu`](crate::prelude::user_Hmenu::DestroyMenu).
	#[must_use]
	fn CreatePopupMenu() -> SysResult<HMENU> {
		ptr_to_sysresult_handle(unsafe { user::ffi::CreatePopupMenu() })
	}

	/// [`DeleteMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// method.
	fn DeleteMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::DeleteMenu(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
				)
			},
		)
	}

	/// [`DestroyMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DestroyMenu(&mut self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { user::ffi::DestroyMenu(self.as_ptr()) },
		);
		*self = Self::INVALID;
		ret
	}

	/// [`EnableMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// method.
	///
	/// # Examples
	///
	/// Disabling a menu item:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HMENU, IdPos, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 101;
	/// }
	///
	/// let hmenu: HMENU; // initialized somewhere
	/// # let hmenu = HMENU::NULL;
	///
	/// hmenu.EnableMenuItem(
	///     IdPos::Id(ID_FILE_OPEN),
	///     false,
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Disabling multiple menu items at once:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HMENU, IdPos, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 201;
	///     ID_FILE_SAVE
	/// }
	///
	/// let hmenu: HMENU; // initialized somewhere
	/// # let hmenu = HMENU::NULL;
	///
	/// [
	///     (ID_FILE_OPEN, "Open\tCtrl+O"),
	///     (ID_FILE_SAVE, "&Save"),
	/// ].iter()
	///     .map(|(id, txt)| hmenu.EnableMenuItem(
	///         IdPos::Id(*id),
	///         false,
	///     ))
	///     .collect::<Result<Vec<_>, _>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn EnableMenuItem(&self,
		id_or_pos: IdPos, enable: bool) -> SysResult<co::MF>
	{
		match unsafe {
			user::ffi::EnableMenuItem(
				self.as_ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag()
					| if enable { co::MF::ENABLED } else { co::MF::DISABLED }).raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`GetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenudefaultitem)
	/// method.
	#[must_use]
	fn GetMenuDefaultItem(&self,
		by_pos: bool, flags: co::GMDI) -> SysResult<IdPos>
	{
		match unsafe {
			user::ffi::GetMenuDefaultItem(self.as_ptr(), by_pos as _, flags.raw())
				as i32
		} {
			-1 => Err(GetLastError()),
			n => Ok(if by_pos { IdPos::Pos(n as _) } else { IdPos::Id(n as _) }),
		}
	}

	/// [`GetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// method.
	fn GetMenuInfo(&self, mi: &mut MENUINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::GetMenuInfo(self.as_ptr(), mi as *mut _ as _) },
		)
	}

	/// [`GetMenuItemCount`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// method.
	#[must_use]
	fn GetMenuItemCount(&self) -> SysResult<u32> {
		match unsafe { user::ffi::GetMenuItemCount(self.as_ptr()) } {
			-1 => Err(GetLastError()),
			count => Ok(count as _),
		}
	}

	/// [`GetMenuItemID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// method.
	#[must_use]
	fn GetMenuItemID(&self, nPos: i32) -> Option<i32> {
		match unsafe { user::ffi::GetMenuItemID(self.as_ptr(), nPos) } {
			-1 => None,
			id => Some(id),
		}
	}

	/// [`GetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuiteminfow)
	/// method.
	fn GetMenuItemInfo(&self,
		id_or_pos: IdPos, mii: &mut MENUITEMINFO) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::GetMenuItemInfoW(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuState`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustate)
	/// method.
	#[must_use]
	fn GetMenuState(&self, id_or_pos: IdPos) -> SysResult<co::MF> {
		match unsafe {
			user::ffi::GetMenuState(
				self.as_ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
			) as i32
		} {
			-1 => Err(GetLastError()),
			mf => Ok(unsafe { co::MF::from_raw(mf as _) }),
		}
	}

	/// [`GetMenuString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustringw)
	/// method.
	#[must_use]
	fn GetMenuString(&self, id_or_pos: IdPos) -> SysResult<String> {
		const BLOCK_SZ: usize = 64; // arbitrary
		let mut buf_sz = BLOCK_SZ;

		loop {
			let mut buf = WString::new_alloc_buf(buf_sz);

			let nchars = match unsafe {
				user::ffi::GetMenuStringW(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					id_or_pos.mf_flag().raw(),
				)
			} {
				0 => return Err(GetLastError()),
				n => n,
			};

			if (nchars as usize) + 1 < buf_sz { // to break, must have at least 1 char gap
				return Ok(buf.to_string());
			}

			buf_sz += BLOCK_SZ; // increase buffer size to try again
		}
	}

	/// [`GetSubMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// method.
	#[must_use]
	fn GetSubMenu(&self, pos: u32) -> Option<HMENU> {
		ptr_to_option_handle(
			unsafe { user::ffi::GetSubMenu(self.as_ptr(), pos as _) },
		)
	}

	/// [`InsertMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// method.
	fn InsertMenuItem(&self,
		id_or_pos: IdPos, mii: &MENUITEMINFO) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::InsertMenuItemW(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
				)
			},
		)
	}

	/// [`IsMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// method.
	#[must_use]
	fn IsMenu(&self) -> bool {
		unsafe { user::ffi::IsMenu(self.as_ptr()) != 0 }
	}

	/// [`RemoveMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// method.
	fn RemoveMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::RemoveMenu(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
				)
			},
		)
	}

	/// [`SetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenudefaultitem)
	/// method.
	fn SetMenuDefaultItem(&self, id_or_pos: IdPos) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::SetMenuDefaultItem(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
				)
			},
		)
	}

	/// [`SetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// method.
	fn SetMenuInfo(&self, mi: &MENUINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::SetMenuInfo(self.as_ptr(), mi as *const _ as _) },
		)
	}

	/// [`SetMenuItemBitmaps`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuitembitmaps)
	/// method.
	fn SetMenuItemBitmaps(&self,
		id_or_pos: IdPos,
		hbmp_unchecked: Option<&HBITMAP>,
		hbmp_checked: Option<&HBITMAP>,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::SetMenuItemBitmaps(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.mf_flag().raw(),
					hbmp_unchecked.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
					hbmp_checked.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
				)
			},
		)
	}

	/// [`SetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// method.
	fn SetMenuItemInfo(&self,
		id_or_pos: IdPos, mii: &MENUITEMINFO) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::SetMenuItemInfoW(
					self.as_ptr(),
					id_or_pos.id_or_pos_u32(),
					id_or_pos.is_by_pos() as _,
					mii as *const _ as _,
				)
			},
		)
	}

	/// [`TrackPopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// method.
	///
	/// **Note:** If you just want to display a popup menu, consider the simpler
	/// [`HMENU::TrackPopupMenuAtPoint`](crate::prelude::user_Hmenu::TrackPopupMenuAtPoint).
	fn TrackPopupMenu(&self,
		flags: co::TPM, location: POINT, hwnd: &HWND) -> SysResult<Option<i32>>
	{
		let ret = unsafe {
			user::ffi::TrackPopupMenu(
				self.as_ptr(),
				flags.raw(),
				location.x, location.y,
				0,
				hwnd.as_ptr(),
				std::ptr::null(),
			)
		};

		if flags.has(co::TPM::RETURNCMD) {
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
	/// [`TrackPopupMenu`](crate::prelude::user_Hmenu::TrackPopupMenu), and
	/// performs other needed operations.
	///
	/// This method will block until the menu disappears.
	fn TrackPopupMenuAtPoint(&self,
		pos: POINT,
		hwnd_parent: &HWND,
		hwnd_coords_relative_to: &HWND,
	) -> SysResult<()>
	{
		let mut pos = pos;
		hwnd_coords_relative_to.ClientToScreen(&mut pos)?; // now relative to screen
		hwnd_parent.SetForegroundWindow();
		self.TrackPopupMenu(co::TPM::LEFTBUTTON, pos, hwnd_parent)?;
		hwnd_parent.PostMessage(msg::wm::Null {})?; // necessary according to TrackPopupMenu docs
		Ok(())
	}
}
