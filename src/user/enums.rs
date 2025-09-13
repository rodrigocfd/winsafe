#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variant parameter for:
///
/// * [`wm::Command`](crate::msg::wm::Command)
pub enum AccelMenuCtrl {
	/// Accelerator event. Contains [`co::CMD::Accel`](crate::co::CMD::Accel)
	/// and the accelerator command ID.
	Accel(u16),
	/// Menu item click event. Contains [`co::CMD::Menu`](crate::co::CMD::Menu)
	/// and the menu item command ID.
	Menu(u16),
	/// Specific child control event, with a custom `co::CMD`.
	Ctrl { notif_code: co::CMD, ctrl_id: u16, ctrl_hwnd: HWND },
}

impl AccelMenuCtrl {
	/// Returns the control ID.
	#[must_use]
	pub const fn ctrl_id(&self) -> u16 {
		use AccelMenuCtrl::*;
		match self {
			Accel(id) => *id,
			Menu(id) => *id,
			Ctrl { notif_code: _, ctrl_id, ctrl_hwnd: _ } => *ctrl_id,
		}
	}

	/// Returns the notification code.
	#[must_use]
	pub const fn code(&self) -> co::CMD {
		use AccelMenuCtrl::*;
		match self {
			Accel(_) => co::CMD::Accel,
			Menu(_) => co::CMD::Menu,
			Ctrl { notif_code, ctrl_id: _, ctrl_hwnd: _ } => *notif_code,
		}
	}
}

/// Variant parameter for:
///
/// * [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx)
/// * [`HWND::FindWindow`](crate::HWND::FindWindow)
/// * [`HWND::FindWindowEx`](crate::HWND::FindWindowEx)
/// * [`UnregisterClass`](crate::UnregisterClass)
#[derive(Clone)]
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(WString),
}

impl AtomStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use AtomStr::*;
		match self {
			Atom(atom) => MAKEINTRESOURCE(u16::from(*atom) as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`bm::GetImage`](crate::msg::bm::GetImage)
/// * [`bm::SetImage`](crate::msg::bm::SetImage)
pub enum BmpIcon {
	/// A bitmap.
	Bmp(HBITMAP),
	/// An icon.
	Icon(HICON),
}

impl BmpIcon {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		unsafe {
			use BmpIcon::*;
			std::mem::transmute(match self {
				Bmp(hbmp) => hbmp.ptr(),
				Icon(hicon) => hicon.ptr(),
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenu`](crate::HMENU::AppendMenu)
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
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use BmpPtrStr::*;
		match self {
			Bmp(hbmp) => hbmp.ptr() as _,
			Ptr(lp) => *lp as _,
			Str(ws) => ws.as_ptr(),
			None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`DEVMODE`](crate::DEVMODE)
#[derive(Clone, Copy)]
pub enum DispfNup {
	/// Used for displays.
	Dispf(co::DMDISPLAYFLAGS),
	/// Used for printers.
	Nup(co::DMNUP),
}

/// Variant parameter for:
///
/// * [`EnumDisplaySettings`](crate::EnumDisplaySettings)
#[derive(Clone, Copy)]
pub enum GmidxEnum {
	/// Graphics mode index.
	Gmidx(u32),
	/// Predefined enumeration.
	Enum(co::ENUM_SETTINGS),
}

impl From<GmidxEnum> for u32 {
	fn from(v: GmidxEnum) -> Self {
		use GmidxEnum::*;
		match v {
			Gmidx(idx) => idx,
			Enum(es) => es.raw(),
		}
	}
}

/// Variant parameter for:
///
/// * [`INPUT`](crate::INPUT)
#[derive(Clone, Copy)]
pub enum HwKbMouse {
	/// Hardware event.
	Hw(HARDWAREINPUT),
	/// Keyboard event.
	Kb(KEYBDINPUT),
	/// Mouse event.
	Mouse(MOUSEINPUT),
}

/// Variant parameter for:
///
/// * [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl)
pub enum HwndFocus {
	/// Handle to the control to receive the focus.
	Hwnd(HWND),
	/// If `true`, the next control with [`WS::TABSTOP`](crate::co::WS::TABSTOP)
	/// receives the focus; if `false`, the previous.
	FocusNext(bool),
}

/// Variant parameter for:
///
/// * [`wm::EnterIdle`](crate::msg::wm::EnterIdle)
/// * [`HELPINFO`](crate::HELPINFO)
pub enum HwndHmenu {
	/// A window.
	Hwnd(HWND),
	/// A menu.
	Hmenu(HMENU),
}

impl HwndHmenu {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		use HwndHmenu::*;
		match self {
			Hwnd(hwnd) => hwnd.ptr() as _,
			Hmenu(hmenu) => hmenu.ptr() as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HWND::SetWindowPos`](crate::HWND::SetWindowPos)
/// * [`WINDOWPOS`](crate::WINDOWPOS)
/// * [`WINDOWPOS`](crate::WINDOWPOS)
pub enum HwndPlace {
	/// A handle to the window to precede the positioned window in the Z order.
	Hwnd(HWND),
	/// A constant specifying where the window will be placed.
	Place(co::HWND_PLACE),
	/// Nothing.
	None,
}

impl HwndPlace {
	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		use HwndPlace::*;
		match self {
			Hwnd(hwnd) => hwnd.ptr(),
			Place(v) => v.raw() as _,
			None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::ParentNotify`](crate::msg::wm::ParentNotify)
pub enum HwndPointId {
	/// Handle to the child window.
	Hwnd(HWND),
	/// Cursor coordinates.
	Point(POINT),
	/// Pointer identifier.
	Id(u16),
}

impl HwndPointId {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		use HwndPointId::*;
		match self {
			Hwnd(hwnd) => hwnd.ptr() as _,
			Point(pt) => u32::from(*pt) as _,
			Id(id) => *id as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadCursor`](crate::HINSTANCE::LoadCursor)
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
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdIdcStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Idc(idc) => MAKEINTRESOURCE(idc.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadIcon`](crate::HINSTANCE::LoadIcon)
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
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		use IdIdiStr::*;
		match self {
			Id(id) => MAKEINTRESOURCE(*id as _),
			Idi(idi) => MAKEINTRESOURCE(idi.raw() as _),
			Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`HMENU::AppendMenu`](crate::HMENU::AppendMenu)
/// * [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx)
pub enum IdMenu<'a> {
	/// A command ID.
	Id(u16),
	/// An [`HMENU`](crate::HMENU).
	Menu(&'a HMENU),
	/// Nothing.
	None,
}

impl<'a> IdMenu<'a> {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut std::ffi::c_void {
		use IdMenu::*;
		match self {
			Id(id) => *id as _,
			Menu(hMenu) => hMenu.ptr(),
			None => std::ptr::null_mut(),
		}
	}

	/// Converts the raw data into an `usize`.
	#[must_use]
	pub fn as_usize(&self) -> usize {
		use IdMenu::*;
		match self {
			Id(id) => *id as _,
			Menu(hMenu) => hMenu.ptr() as _,
			None => 0,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::CheckMenuItem`](crate::HMENU::CheckMenuItem)
/// * [`HMENU::CheckMenuRadioItem`](crate::HMENU::CheckMenuRadioItem)
/// * [`HMENU::DeleteMenu`](crate::HMENU::DeleteMenu)
/// * [`HMENU::EnableMenuItem`](crate::HMENU::EnableMenuItem)
/// * [`HMENU::GetMenuDefaultItem`](crate::HMENU::GetMenuDefaultItem)
/// * [`HMENU::GetMenuItemInfo`](crate::HMENU::GetMenuItemInfo)
/// * [`HMENU::GetMenuState`](crate::HMENU::GetMenuState)
/// * [`HMENU::GetMenuString`](crate::HMENU::GetMenuString)
/// * [`HMENU::InsertMenuItem`](crate::HMENU::InsertMenuItem)
/// * [`HMENU::RemoveMenu`](crate::HMENU::RemoveMenu)
/// * [`HMENU::SetMenuItemBitmaps`](crate::HMENU::SetMenuItemBitmaps)
/// * [`HMENU::SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo)
/// * [`HWND::HiliteMenuItem`](crate::HWND::HiliteMenuItem)
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
	pub const fn is_by_pos(self) -> bool {
		use IdPos::*;
		match self {
			Id(_) => false,
			Pos(_) => true,
		}
	}

	/// Returns the ID or the position as a plain `u32`.
	#[must_use]
	pub const fn id_or_pos_u32(self) -> u32 {
		use IdPos::*;
		match self {
			Id(id) => id as _,
			Pos(pos) => pos,
		}
	}

	/// Returns [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) if value is `Id`, or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) if value is `Pos`.
	#[must_use]
	pub const fn mf_flag(self) -> co::MF {
		use IdPos::*;
		match self {
			Id(_) => co::MF::BYCOMMAND,
			Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::append_item`](crate::HMENU::append_item)
pub enum MenuItem<'a> {
	/// A selectable entry item, with command ID and text.
	Entry { cmd_id: u16, text: &'a str },
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu { submenu: &'a HMENU, text: &'a str },
}

/// Variant parameter for:
///
/// * [`HMENU::item_info`](crate::HMENU::item_info)
/// * [`HMENU::iter_items`](crate::HMENU::iter_items)
pub enum MenuItemInfo {
	/// A selectable entry item, with command ID and text.
	Entry { cmd_id: u16, text: String },
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu { submenu: HMENU, text: String },
}

/// Variant parameter for:
///
/// * [`wm::NcCalcSize`](crate::msg::wm::NcCalcSize)
pub enum NccspRect<'a, 'b> {
	/// Mutable reference to [`NCCALCSIZE_PARAMS`](crate::NCCALCSIZE_PARAMS).
	Nccsp(&'b mut NCCALCSIZE_PARAMS<'a>),
	/// Mutable reference to [`RECT`](crate::RECT).
	Rect(&'b mut RECT),
}

/// Variant parameter for:
///
/// * [`HWND::MapWindowPoints`](crate::HWND::MapWindowPoints)
pub enum PtsRc<'a> {
	/// A series of [`POINT`](crate::POINT) structs.
	Pts(&'a mut [POINT]),
	/// A single [`RECT`](crate::RECT) struct.
	Rc(&'a mut RECT),
}

/// Variant parameter for:
///
/// * [`HPROCESS::WaitForInputIdle`](crate::HPROCESS::WaitForInputIdle)
pub enum SuccessTimeout {
	/// The wait was satisfied successfully.
	Success,
	/// The wait was terminated because the time-out interval elapsed.
	Timeout,
}
