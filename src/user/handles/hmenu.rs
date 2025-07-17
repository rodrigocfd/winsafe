#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::{ffi, iterators::*};

handle! { HMENU;
	/// Handle to a
	/// [menu](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmenu).
}

impl HMENU {
	/// A more convenient [`HMENU::AppendMenu`](crate::HMENU::AppendMenu).
	///
	/// # Examples
	///
	/// Adding multiple entries at once, with their command IDs:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 2001;
	///     ID_FILE_SAVE
	///     ID_FILE_EXIT
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// hmenu.append_item(&[
	///     w::MenuItem::Entry { cmd_id: ID_FILE_OPEN, text: "&Open" },
	///     w::MenuItem::Entry { cmd_id: ID_FILE_OPEN, text: "&Save" },
	///     w::MenuItem::Separator,
	///     w::MenuItem::Entry { cmd_id: ID_FILE_EXIT, text: "E&xit" },
	/// ])?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn append_item(&self, items: &[MenuItem]) -> SysResult<()> {
		items
			.iter()
			.map(|item| match item {
				MenuItem::Entry { cmd_id, text: entry_text } => self.AppendMenu(
					co::MF::STRING,
					IdMenu::Id(*cmd_id),
					BmpPtrStr::from_str(*entry_text),
				),
				MenuItem::Separator => {
					self.AppendMenu(co::MF::SEPARATOR, IdMenu::None, BmpPtrStr::None)
				},
				MenuItem::Submenu { submenu, text: entry_text } => self.AppendMenu(
					co::MF::POPUP,
					IdMenu::Menu(submenu),
					BmpPtrStr::from_str(*entry_text),
				),
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(())
	}

	/// Simpler version of
	/// [`HMENU::GetMenuItemInfo`](crate::HMENU::GetMenuItemInfo), which returns
	/// a [`MenuItemInfo`](crate::MenuItemInfo) instead of the tricky
	/// [`MENUITEMINFO`](crate::MENUITEMINFO).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// let item_info = hmenu.item_info(w::IdPos::Id(0))?;
	/// match item_info {
	///     w::MenuItemInfo::Entry { cmd_id, text } =>
	///         println!("item {} {}", cmd_id, text),
	///     w::MenuItemInfo::Separator =>
	///         println!("separator"),
	///     w::MenuItemInfo::Submenu { submenu, text } =>
	///         println!("submenu {} {}", submenu, text),
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn item_info(&self, id_or_pos: IdPos) -> SysResult<MenuItemInfo> {
		let mut mii = MENUITEMINFO::default();
		mii.fMask = co::MIIM::FTYPE | co::MIIM::ID | co::MIIM::STATE | co::MIIM::SUBMENU;
		self.GetMenuItemInfo(id_or_pos, &mut mii)?;

		let nfo = if mii.fType == co::MFT::SEPARATOR {
			MenuItemInfo::Separator
		} else {
			let text = self.GetMenuString(id_or_pos)?;
			if mii.hSubMenu != HMENU::NULL {
				MenuItemInfo::Submenu { submenu: mii.hSubMenu, text }
			} else {
				MenuItemInfo::Entry { cmd_id: mii.wID as _, text }
			}
		};

		Ok(nfo)
	}

	/// Returns an iterator over all menu items, including submenus and
	/// separators.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// for item_info in hmenu.iter_items()? {
	///     let item_info = item_info?;
	///     match item_info {
	///         w::MenuItemInfo::Entry { cmd_id, text } =>
	///             println!("item {} {}", cmd_id, text),
	///         w::MenuItemInfo::Separator =>
	///             println!("separator"),
	///         w::MenuItemInfo::Submenu { submenu, text } =>
	///             println!("submenu {} {}", submenu, text),
	///     }
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_items(
		&self,
	) -> SysResult<impl DoubleEndedIterator<Item = SysResult<MenuItemInfo>> + '_> {
		HmenuIteritems::new(self)
	}

	/// Shows the popup menu anchored at the given coordinates using
	/// [`TrackPopupMenu`](crate::HMENU::TrackPopupMenu), and performs other
	/// needed operations.
	///
	/// This method will block until the menu disappears.
	pub fn track_popup_menu_at_point(
		&self,
		pos: POINT,
		hwnd_parent: &HWND,
		hwnd_coords_relative_to: &HWND,
	) -> SysResult<()> {
		let pos = hwnd_coords_relative_to.ClientToScreen(pos)?; // now relative to screen
		hwnd_parent.SetForegroundWindow();
		self.TrackPopupMenu(co::TPM::LEFTBUTTON, pos, hwnd_parent)?;
		unsafe {
			hwnd_parent.PostMessage(wm::Null {})?; // necessary according to TrackPopupMenu docs
		}
		Ok(())
	}

	/// [`AppendMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`HMENU::append_item`](crate::HMENU::append_item).
	pub fn AppendMenu(&self, flags: co::MF, new_item: IdMenu, content: BmpPtrStr) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::AppendMenuW(self.ptr(), flags.raw(), new_item.as_usize(), content.as_ptr())
		})
		.to_sysresult()
	}

	/// [`CheckMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuitem)
	/// function.
	pub fn CheckMenuItem(&self, id_or_pos: IdPos, check: bool) -> SysResult<co::MF> {
		match unsafe {
			ffi::CheckMenuItem(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag() | if check { co::MF::CHECKED } else { co::MF::UNCHECKED })
					.raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`CheckMenuRadioItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-checkmenuradioitem)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `first`, `last` and `check` don't use the same enum field.
	pub fn CheckMenuRadioItem(&self, first: IdPos, last: IdPos, check: IdPos) -> SysResult<()> {
		if !(first.is_by_pos() == last.is_by_pos() && last.is_by_pos() == check.is_by_pos()) {
			panic!("Different enum fields.");
		}

		BoolRet(unsafe {
			ffi::CheckMenuRadioItem(
				self.ptr(),
				first.id_or_pos_u32(),
				last.id_or_pos_u32(),
				check.id_or_pos_u32(),
				check.mf_flag().raw(),
			)
		})
		.to_sysresult()
	}

	/// [`CreateMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
	/// function.
	///
	/// **Note:** If not attached to a window, must be paired with an
	/// [`HMENU::DestroyMenu`](crate::HMENU::DestroyMenu) call.
	#[must_use]
	pub fn CreateMenu() -> SysResult<HMENU> {
		PtrRet(unsafe { ffi::CreateMenu() }).to_sysresult_handle()
	}

	/// [`CreatePopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createpopupmenu)
	/// function.
	///
	/// **Note:** When a menu is attached to a window, it's automatically
	/// destroyed along with the window. However, if the menu is not attached to
	/// any window, you must call
	/// [`HMENU::DestroyMenu`](crate::HMENU::DestroyMenu).
	#[must_use]
	pub fn CreatePopupMenu() -> SysResult<HMENU> {
		PtrRet(unsafe { ffi::CreatePopupMenu() }).to_sysresult_handle()
	}

	/// [`DeleteMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-deletemenu)
	/// function.
	pub fn DeleteMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::DeleteMenu(self.ptr(), id_or_pos.id_or_pos_u32(), id_or_pos.mf_flag().raw())
		})
		.to_sysresult()
	}

	/// [`DestroyMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroymenu)
	/// function.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	pub fn DestroyMenu(&mut self) -> SysResult<()> {
		let ret = BoolRet(unsafe { ffi::DestroyMenu(self.ptr()) }).to_sysresult();
		*self = Self::INVALID;
		ret
	}

	/// [`EnableMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablemenuitem)
	/// function.
	///
	/// # Examples
	///
	/// Disabling a menu item:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 101;
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// hmenu.EnableMenuItem(
	///     w::IdPos::Id(ID_FILE_OPEN),
	///     false,
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// Disabling multiple menu items at once:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, seq_ids};
	///
	/// seq_ids! {
	///     ID_FILE_OPEN = 201;
	///     ID_FILE_SAVE
	/// }
	///
	/// let hmenu: w::HMENU; // initialized somewhere
	/// # let hmenu = w::HMENU::NULL;
	///
	/// [ID_FILE_OPEN, ID_FILE_SAVE]
	///     .into_iter()
	///     .try_for_each(|id|
	///         hmenu.EnableMenuItem(
	///             w::IdPos::Id(id),
	///             false,
	///         ).map(|_| ())
	///     )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn EnableMenuItem(&self, id_or_pos: IdPos, enable: bool) -> SysResult<co::MF> {
		match unsafe {
			ffi::EnableMenuItem(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				(id_or_pos.mf_flag() | if enable { co::MF::ENABLED } else { co::MF::DISABLED })
					.raw(),
			)
		} {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			ret => Ok(unsafe { co::MF::from_raw(ret as _) }),
		}
	}

	/// [`GetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenudefaultitem)
	/// function.
	#[must_use]
	pub fn GetMenuDefaultItem(&self, by_pos: bool, flags: co::GMDI) -> SysResult<IdPos> {
		match unsafe { ffi::GetMenuDefaultItem(self.ptr(), by_pos as _, flags.raw()) as i32 } {
			-1 => Err(GetLastError()),
			n => Ok(if by_pos { IdPos::Pos(n as _) } else { IdPos::Id(n as _) }),
		}
	}

	/// [`GetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuinfo)
	/// function.
	pub fn GetMenuInfo(&self, mi: &mut MENUINFO) -> SysResult<()> {
		BoolRet(unsafe { ffi::GetMenuInfo(self.ptr(), pvoid(mi)) }).to_sysresult()
	}

	/// [`GetMenuItemCount`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemcount)
	/// function.
	#[must_use]
	pub fn GetMenuItemCount(&self) -> SysResult<u32> {
		match unsafe { ffi::GetMenuItemCount(self.ptr()) } {
			-1 => Err(GetLastError()),
			count => Ok(count as _),
		}
	}

	/// [`GetMenuItemID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemid)
	/// function.
	///
	/// If `item_index` corresponds to a submenu, returns `None`.
	#[must_use]
	pub fn GetMenuItemID(&self, item_index: i32) -> Option<u16> {
		match unsafe { ffi::GetMenuItemID(self.ptr(), item_index) } {
			-1 => None,
			id => Some(id as _),
		}
	}

	/// [`GetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuiteminfow)
	/// function.
	///
	/// This method is rather tricky, consider using
	/// [`HMENU::item_info`](crate::HMENU::item_info).
	pub fn GetMenuItemInfo(&self, id_or_pos: IdPos, mii: &mut MENUITEMINFO) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::GetMenuItemInfoW(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
				pvoid(mii),
			)
		})
		.to_sysresult()
	}

	/// [`GetMenuState`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustate)
	/// function.
	#[must_use]
	pub fn GetMenuState(&self, id_or_pos: IdPos) -> SysResult<co::MF> {
		match unsafe {
			ffi::GetMenuState(self.ptr(), id_or_pos.id_or_pos_u32(), id_or_pos.is_by_pos() as _)
				as i32
		} {
			-1 => Err(GetLastError()),
			mf => Ok(unsafe { co::MF::from_raw(mf as _) }),
		}
	}

	/// [`GetMenuString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenustringw)
	/// function.
	#[must_use]
	pub fn GetMenuString(&self, id_or_pos: IdPos) -> SysResult<String> {
		let mut buf_sz = WString::SSO_LEN; // start with no string heap allocation
		loop {
			let mut buf = WString::new_alloc_buf(buf_sz);

			let returned_chars = match unsafe {
				// char count without terminating null
				ffi::GetMenuStringW(
					self.ptr(),
					id_or_pos.id_or_pos_u32(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					id_or_pos.mf_flag().raw(),
				)
			} {
				0 => return Err(GetLastError()),
				n => n + 1, // plus terminating null count
			};

			if (returned_chars as usize) < buf_sz {
				return Ok(buf.to_string()); // to break, must have at least 1 char gap
			}

			buf_sz *= 2; // double the buffer size to try again
		}
	}

	/// [`GetSubMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsubmenu)
	/// function.
	#[must_use]
	pub fn GetSubMenu(&self, pos: u32) -> Option<HMENU> {
		PtrRet(unsafe { ffi::GetSubMenu(self.ptr(), pos as _) }).to_opt_handle()
	}

	/// [`InsertMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insertmenuitemw)
	/// function.
	pub fn InsertMenuItem(&self, id_or_pos: IdPos, mii: &MENUITEMINFO) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::InsertMenuItemW(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
				pcvoid(mii),
			)
		})
		.to_sysresult()
	}

	/// [`IsMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ismenu)
	/// function.
	#[must_use]
	pub fn IsMenu(&self) -> bool {
		unsafe { ffi::IsMenu(self.ptr()) != 0 }
	}

	/// [`RemoveMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-removemenu)
	/// function.
	pub fn RemoveMenu(&self, id_or_pos: IdPos) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::RemoveMenu(self.ptr(), id_or_pos.id_or_pos_u32(), id_or_pos.mf_flag().raw())
		})
		.to_sysresult()
	}

	/// [`SetMenuDefaultItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenudefaultitem)
	/// function.
	pub fn SetMenuDefaultItem(&self, id_or_pos: IdPos) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetMenuDefaultItem(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
			)
		})
		.to_sysresult()
	}

	/// [`SetMenuInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuinfo)
	/// function.
	pub fn SetMenuInfo(&self, mi: &MENUINFO) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetMenuInfo(self.ptr(), pcvoid(mi)) }).to_sysresult()
	}

	/// [`SetMenuItemBitmaps`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuitembitmaps)
	/// function.
	pub fn SetMenuItemBitmaps(
		&self,
		id_or_pos: IdPos,
		hbmp_unchecked: Option<&HBITMAP>,
		hbmp_checked: Option<&HBITMAP>,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetMenuItemBitmaps(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.mf_flag().raw(),
				hbmp_unchecked.map_or(std::ptr::null_mut(), |h| h.ptr()),
				hbmp_checked.map_or(std::ptr::null_mut(), |h| h.ptr()),
			)
		})
		.to_sysresult()
	}

	/// [`SetMenuItemInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenuiteminfow)
	/// function.
	pub fn SetMenuItemInfo(&self, id_or_pos: IdPos, mii: &MENUITEMINFO) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetMenuItemInfoW(
				self.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.is_by_pos() as _,
				pcvoid(mii),
			)
		})
		.to_sysresult()
	}

	/// [`TrackPopupMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu)
	/// function.
	///
	/// **Note:** If you just want to display a popup menu, consider the simpler
	/// [`HMENU::track_popup_menu_at_point`](crate::HMENU::track_popup_menu_at_point).
	pub fn TrackPopupMenu(
		&self,
		flags: co::TPM,
		location: POINT,
		hwnd: &HWND,
	) -> SysResult<Option<i32>> {
		let ret = unsafe {
			ffi::TrackPopupMenu(
				self.ptr(),
				flags.raw(),
				location.x,
				location.y,
				0,
				hwnd.ptr(),
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
}
