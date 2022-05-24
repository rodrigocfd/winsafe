#![allow(non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::privs::MAKEINTRESOURCE;
use crate::user::decl::{
	ATOM, HBITMAP, HICON, HMENU, HWND, NCCALCSIZE_PARAMS, POINT, RECT,
};

/// Variant parameters of a [`wm::Command`](crate::msg::wm::Command) message.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum AccelMenuCtrl {
	/// Accelerator event. Contains the accelerator command ID.
	Accel(u16),
	/// Menu item click event. Contains the menu item command ID.
	Menu(u16),
	/// Some child control event. Contains
	/// [`AccelMenuCtrlData`](crate::AccelMenuCtrlData) data.
	Ctrl(AccelMenuCtrlData),
}

impl AccelMenuCtrl {
	/// Returns the notification code and the control ID pair.
	#[must_use]
	pub fn code_id(&self) -> (co::CMD, u16) {
		match self {
			AccelMenuCtrl::Accel(id) => (co::CMD::Accelerator, *id),
			AccelMenuCtrl::Menu(id) => (co::CMD::Menu, *id),
			AccelMenuCtrl::Ctrl(data) => (data.notif_code, data.ctrl_id),
		}
	}
}

/// The data of the [`AccelMenuCtrl`](crate::AccelMenuCtrl) `Ctrl` option.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub struct AccelMenuCtrlData {
	pub notif_code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

/// Variant parameter used in
/// [window class](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-classes)
/// functions:
///
/// * [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx) `lpClassName`;
/// * [`HWND::FindWindowEx`](crate::prelude::UserHwnd::FindWindowEx) `lpszClass`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(WString),
}

impl AtomStr {
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Atom(atom) => MAKEINTRESOURCE(atom.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`bm::GetImage`](crate::msg::bm::GetImage) `image`;
/// * [`bm::GetImage`](crate::msg::bm::SetImage) `image`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum BmpIcon {
	Bmp(HBITMAP),
	Icon(HICON),
}

impl BmpIcon {
	#[must_use]
	pub fn as_isize(&self) -> isize {
		(match self {
			Self::Bmp(hbmp) => hbmp.0,
			Self::Icon(hicon) => hicon.0,
		}) as isize
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenu`](crate::prelude::UserHmenu::AppendMenu) `lpNewItem`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum BmpPtrStr {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A pointer to anything.
	Ptr(*const std::ffi::c_void),
	/// A string.
	Str(WString),
	/// Nothing.
	None,
}

impl BmpPtrStr {
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Bmp(hbmp) => hbmp.0 as _,
			Self::Ptr(lp) => *lp as _,
			Self::Str(ws) => unsafe { ws.as_ptr() },
			Self::None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`DEVMODE`](crate::DEVMODE) `dmDisplayFlags`;
/// * [`DEVMODE`](crate::DEVMODE) `dmNup`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum DispfNup {
	/// Used for displays.
	Dispf(co::DMDISPLAYFLAGS),
	/// Used for printers.
	Nup(co::DMNUP),
}

/// Variant parameter for:
///
/// * [`EnumDisplaySettings`](crate::EnumDisplaySettings) `mode_num`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum GmidxEnum {
	/// Graphics mode index.
	Gmidx(u32),
	/// Predefined enumeration.
	Enum(co::ENUM_SETTINGS),
}

impl GmidxEnum {
	#[must_use]
	pub const fn as_u32(&self) -> u32 {
		match self {
			Self::Gmidx(idx) => *idx,
			Self::Enum(es) => es.0,
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl) `hwnd_focus`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum HwndFocus {
	/// Handle to the control to receive the focus.
	Hwnd(HWND),
	/// If `true`, the next control with [`WS::TABSTOP`](crate::co::WS::TABSTOP)
	/// receives the focus; if `false`, the previous.
	FocusNext(bool),
}

/// Variant parameter for:
///
/// * [`wm::EnterIdle`](crate::msg::wm::EnterIdle) reason;
/// * [`HELPINFO`](crate::HELPINFO) `hItemHandle`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum HwndHmenu {
	Hwnd(HWND),
	Hmenu(HMENU),
}

impl HwndHmenu {
	#[must_use]
	pub fn as_isize(&self) -> isize {
		(match self {
			Self::Hwnd(hwnd) => hwnd.0,
			Self::Hmenu(hmenu) => hmenu.0,
		}) as _
	}
}

/// Variant parameter for:
///
/// * [`HWND::SetWindowPos`](crate::prelude::UserHwnd::SetWindowPos) `hwnd_insert_after`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum HwndPlace {
	/// A handle to the window to precede the positioned window in the Z order.
	Hwnd(HWND),
	/// A constant specifying where the window will be placed.
	Place(co::HWND_PLACE),
	/// Nothing.
	None,
}

impl HwndPlace {
	#[must_use]
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Hwnd(hwnd) => hwnd.0,
			Self::Place(v) => v.0 as _,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::ParentNotify`](crate::msg::wm::ParentNotify) `data32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum HwndPointId {
	/// Handle to the child window.
	Hwnd(HWND),
	/// Cursor coordinates.
	Point(POINT),
	/// Pointer identifier.
	Id(u16),
}

impl HwndPointId {
	#[must_use]
	pub fn as_isize(&self) -> isize {
		match self {
			Self::Hwnd(hwnd) => hwnd.0 as _,
			Self::Point(pt) => pt.into_u32() as _,
			Self::Id(id) => *id as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadCursor`](crate::prelude::UserHinstance::LoadCursor) `resource_id`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdIdcStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdcStr {
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idc(idc) => MAKEINTRESOURCE(idc.0),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadIcon`](crate::prelude::UserHinstance::LoadIcon) `icon_id`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdIdiStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDI`](crate::co::IDI) constant for a stock system icon.
	Idi(co::IDI),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdiStr {
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idi(idi) => MAKEINTRESOURCE(idi.0),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`HMENU::AppendMenu`](crate::prelude::UserHmenu::AppendMenu) `uIDNewItem`;
/// * [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx) `hMenu`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum IdMenu {
	/// A command ID.
	Id(u16),
	/// An [`HMENU`](crate::HMENU).
	Menu(HMENU),
	/// Nothing.
	None,
}

impl IdMenu {
	#[must_use]
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Id(id) => *id as _,
			Self::Menu(hMenu) => hMenu.0,
			Self::None => std::ptr::null_mut(),
		}
	}

	#[must_use]
	pub fn as_usize(&self) -> usize {
		match self {
			IdMenu::Id(id) => *id as _,
			IdMenu::Menu(hMenu) => hMenu.0 as _,
			IdMenu::None => 0,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::CheckMenuItem`](crate::prelude::UserHmenu::CheckMenuItem) `uIDCheckItem`;
/// * [`HMENU::CheckMenuRadioItem`](crate::prelude::UserHmenu::CheckMenuRadioItem) `first`, `last`, `check`;
/// * [`HMENU::DeleteMenu`](crate::prelude::UserHmenu::DeleteMenu) `uPosition`;
/// * [`HMENU::EnableMenuItem`](crate::prelude::UserHmenu::EnableMenuItem) `uIDEnableItem`;
/// * [`HMENU::GetMenuDefaultItem`](crate::prelude::UserHmenu::GetMenuDefaultItem) return value;
/// * [`HMENU::GetMenuItemInfo`](crate::prelude::UserHmenu::GetMenuItemInfo) `item`;
/// * [`HMENU::GetMenuState`](crate::prelude::UserHmenu::GetMenuState) `uId`;
/// * [`HMENU::GetMenuString`](crate::prelude::UserHmenu::GetMenuString) `uIDItem`;
/// * [`HMENU::InsertMenuItem`](crate::prelude::UserHmenu::InsertMenuItem) `item`;
/// * [`HMENU::RemoveMenu`](crate::prelude::UserHmenu::RemoveMenu) `uPosition`;
/// * [`HMENU::SetMenuItemBitmaps`](crate::prelude::UserHmenu::SetMenuItemBitmaps) `uPosition`;
/// * [`HMENU::SetMenuItemInfo`](crate::prelude::UserHmenu::SetMenuItemInfo) `item`;
/// * [`HWND::HiliteMenuItem`](crate::prelude::UserHwnd::HiliteMenuItem) `uIDHiliteItem`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum IdPos {
	/// A command ID.
	Id(u16),
	/// Zero-based position.
	Pos(u32),
}

impl IdPos {
	/// Returns whether value is `Pos`.
	#[must_use]
	pub fn is_by_pos(self) -> bool {
		match self {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		}
	}

	/// Returns the ID or the position as a plain `u32`.
	#[must_use]
	pub fn id_or_pos_u32(self) -> u32 {
		match self {
			IdPos::Id(id) => id as _,
			IdPos::Pos(pos) => pos,
		}
	}

	/// Returns [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) if value is `Id`, or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) if value is `Pos`.
	#[must_use]
	pub fn mf_flag(self) -> co::MF {
		match self {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenuEnum`](crate::prelude::UserHmenu::AppendMenuEnum) `entry`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum MenuEnum<'a> {
	/// A selectable entry item, with command ID and text.
	Entry(u16, &'a str),
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu(HMENU, &'a str),
}

/// Variant parameter for:
///
/// * [`wm::NcCalcSize`](crate::msg::wm::NcCalcSize) `data`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum NccspRect<'a, 'b> {
	/// Mutable reference to [`NCCALCSIZE_PARAMS`](crate::NCCALCSIZE_PARAMS).
	Nccsp(&'b mut NCCALCSIZE_PARAMS<'a>),
	/// Mutable reference to [`RECT`](crate::RECT).
	Rect(&'b mut RECT),
}
