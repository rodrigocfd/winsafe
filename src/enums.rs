#![allow(non_snake_case)]

use crate::co;
use crate::funcs::MAKEDWORD;
use crate::handles::{HBITMAP, HICON, HMENU, HTREEITEM, HWND};
use crate::privs::MAKEINTRESOURCE;
use crate::structs::{ATOM, NCCALCSIZE_PARAMS, POINT, RECT};
use crate::various::WString;

/// Variant parameters of a [`WM_COMMAND`](crate::msg::wm::Command) message.
#[derive(Copy, Clone)]
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
	pub fn code_id(&self) -> (co::CMD, u16) {
		match self {
			AccelMenuCtrl::Accel(id) => (co::CMD::Accelerator, *id),
			AccelMenuCtrl::Menu(id) => (co::CMD::Menu, *id),
			AccelMenuCtrl::Ctrl(data) => (data.notif_code, data.ctrl_id),
		}
	}
}

/// The data of the [`AccelMenuCtrl`](crate::AccelMenuCtrl) `Ctrl` option.
#[derive(Copy, Clone)]
pub struct AccelMenuCtrlData {
	pub notif_code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

/// Variant parameter used in
/// [window class](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-classes)
/// functions:
///
/// * [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`HWND::FindWindowEx`](crate::HWND::FindWindowEx) `lpszClass`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
#[derive(Clone)]
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(String),
}

impl AtomStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Atom(atom) => MAKEINTRESOURCE(atom.0 as _),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter for:
///
/// * [`BM_GETIMAGE`](crate::msg::bm::GetImage) `image`;
/// * [`BM_SETIMAGE`](crate::msg::bm::SetImage) `image`.
#[derive(Copy, Clone)]
pub enum BmpIcon {
	Bmp(HBITMAP),
	Icon(HICON),
}

impl BmpIcon {
	pub fn as_isize(&self) -> isize {
		(match self {
			Self::Bmp(hbmp) => hbmp.ptr,
			Self::Icon(hicon) => hicon.ptr,
		}) as isize
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`.
#[derive(Clone)]
pub enum BmpPtrStr {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A pointer to anything.
	Ptr(*const std::ffi::c_void),
	/// A string.
	Str(String),
	/// Nothing.
	None,
}

impl BmpPtrStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Bmp(hbmp) => hbmp.ptr as _,
			Self::Ptr(lp) => *lp as _,
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
			Self::None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`DEVMODE`](crate::DEVMODE) `dmDisplayFlags`;
/// * [`DEVMODE`](crate::DEVMODE) `dmNup`.
#[derive(Copy, Clone)]
pub enum DispfNup {
	/// Used for displays.
	Dispf(co::DMDISPLAYFLAGS),
	/// Used for printers.
	Nup(co::DMNUP),
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenuEnum`](crate::HMENU::AppendMenuEnum) `entry`.
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
/// * [`WM_NEXTDLGCTL`](crate::msg::wm::NextDlgCtl) `hwnd_focus`.
#[derive(Copy, Clone)]
pub enum HwndFocus {
	/// Handle to the control to receive the focus.
	Hwnd(HWND),
	/// If `true`, the next control with [`WS::TABSTOP`](crate::co::WS::TABSTOP)
	/// receives the focus; if `false`, the previous.
	FocusNext(bool),
}

/// Variant parameter for:
///
/// * [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) reason.
/// * [`HELPINFO`](crate::HELPINFO) `hItemHandle`.
#[derive(Copy, Clone)]
pub enum HwndHmenu {
	Hwnd(HWND),
	Hmenu(HMENU),
}

impl HwndHmenu {
	pub fn as_isize(&self) -> isize {
		(match self {
			Self::Hwnd(hwnd) => hwnd.ptr,
			Self::Hmenu(hmenu) => hmenu.ptr,
		}) as _
	}
}

/// Variant parameter for:
///
/// * [`HWND::SetWindowPos`](crate::HWND::SetWindowPos) `hWndInsertAfter`.
#[derive(Copy, Clone)]
pub enum HwndPlace {
	/// A handle to the window to precede the positioned window in the Z order.
	Hwnd(HWND),
	/// A constant specifying where the window will be placed.
	Place(co::HWND_PLACE),
	/// Nothing.
	None,
}

impl HwndPlace {
	/// Converts the internal value to a `*mut c_void`.
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Hwnd(hwnd) => hwnd.ptr,
			Self::Place(v) => v.0 as _,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`WM_PARENTNOTIFY`](crate::msg::wm::ParentNotify) `data32`.
#[derive(Copy, Clone)]
pub enum HwndPointId {
	/// Handle to the child window.
	Hwnd(HWND),
	/// Cursor coordinates.
	Point(POINT),
	/// Pointer identifier.
	Id(u16),
}

impl HwndPointId {
	pub fn as_isize(&self) -> isize {
		match self {
			Self::Hwnd(hwnd) => hwnd.ptr as _,
			Self::Point(pt) => MAKEDWORD(pt.x as _, pt.y as _) as _,
			Self::Id(id) => *id as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `hMainIcon`;
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `hFooterIcon`.
#[derive(Copy, Clone)]
pub enum IconIdTdicon {
	/// No icon.
	None,
	/// An icon handle.
	Icon(HICON),
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Tdicon(co::TD_ICON),
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`.
#[derive(Clone)]
pub enum IdIdcStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A string identifier.
	Str(String),
}

impl IdIdcStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idc(idc) => MAKEINTRESOURCE(idc.0),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.
#[derive(Clone)]
pub enum IdIdiStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDI`](crate::co::IDI) constant for a stock system icon.
	Idi(co::IDI),
	/// A string identifier.
	Str(String),
}

impl IdIdiStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idi(idi) => MAKEINTRESOURCE(idi.0),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`HMENU::AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`.
#[derive(Copy, Clone)]
pub enum IdMenu {
	/// A command ID.
	Id(u16),
	/// An [`HMENU`](crate::HMENU).
	Menu(HMENU),
	/// Nothing.
	None,
}

impl From<IdMenu> for usize {
	fn from(v: IdMenu) -> Self {
		match v {
			IdMenu::Id(id) => id as _,
			IdMenu::Menu(hMenu) => hMenu.ptr as _,
			IdMenu::None => 0,
		}
	}
}

impl IdMenu {
	/// Converts the internal value to a `*mut c_void`.
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Id(id) => *id as _,
			Self::Menu(hMenu) => hMenu.ptr,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`HMENU::CheckMenuRadioItem`](crate::HMENU::CheckMenuRadioItem) `first`, `last`, `check`;
/// * [`HMENU::DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`HMENU::EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HMENU::GetMenuDefaultItem`](crate::HMENU::GetMenuDefaultItem) return value;
/// * [`HMENU::GetMenuItemInfo`](crate::HMENU::GetMenuItemInfo) `item`;
/// * [`HMENU::GetMenuState`](crate::HMENU::GetMenuState) `uId`;
/// * [`HMENU::GetMenuString`](crate::HMENU::GetMenuString) `uIDItem`;
/// * [`HMENU::InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`HMENU::RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`HMENU::SetMenuItemBitmaps`](crate::HMENU::SetMenuItemBitmaps) `uPosition`;
/// * [`HMENU::SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`;
/// * [`HWND::HiliteMenuItem`](crate::HWND::HiliteMenuItem) `uIDHiliteItem`.
#[derive(Copy, Clone)]
pub enum IdPos {
	/// A command ID.
	Id(u16),
	/// Zero-based position.
	Pos(u32),
}

impl IdPos {
	/// Returns whether value is `Pos`.
	pub fn is_by_pos(self) -> bool {
		match self {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		}
	}

	/// Returns the ID or the position as a plain `u32`.
	pub fn id_or_pos_u32(self) -> u32 {
		match self {
			IdPos::Id(id) => id as _,
			IdPos::Pos(pos) => pos,
		}
	}

	/// Returns [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) if value is `Id`, or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) if value is `Pos`.
	pub fn mf_flag(self) -> co::MF {
		match self {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::EnumResourceNames`](crate::HINSTANCE::EnumResourceNames) `func`;
/// * [`HINSTANCE::LoadAccelerators`](crate::HINSTANCE::LoadAccelerators) `lpTableName`.
#[derive(Clone)]
pub enum IdStr {
	/// A resource ID.
	Id(u16),
	/// A string identifier.
	Str(String),
}

impl IdStr {
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter for:
///
/// * [`HWND::TaskDialog`](crate::HWND::TaskDialog) `pszIcon`.
#[derive(Clone)]
pub enum IdTdiconStr {
	/// No icon.
	None,
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Tdicon(co::TD_ICON),
	/// A string identifier.
	Str(String),
}

impl IdTdiconStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::None => std::ptr::null(),
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Tdicon(tdi) => MAKEINTRESOURCE(tdi.0),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter for:
///
/// * [`WM_NCCALCSIZE`](crate::msg::wm::NcCalcSize) `data`.
pub enum NccspRect<'a, 'b> {
	/// Mutable reference to [`NCCALCSIZE_PARAMS`](crate::NCCALCSIZE_PARAMS).
	Nccsp(&'b mut NCCALCSIZE_PARAMS<'a>),
	/// Mutable reference to [`RECT`](crate::RECT).
	Rect(&'b mut RECT),
}

/// Variant value returned by [`RegQueryValueEx`](crate::HKEY::RegQueryValueEx).
#[derive(Clone)]
pub enum RegistryValue {
	/// Binary value, defined as [`REG::BINARY`](crate::co::REG::BINARY).
	Binary(Vec<u8>),
	/// An `u32` integer value, defined as [`REG::DWORD`](crate::co::REG::DWORD).
	Dword(u32),
	/// An `u64` integer value, defined as [`REG::QWORD`](crate::co::REG::QWORD).
	Qword(u64),
	/// String value, defined as [`REG::SZ`](crate::co::REG::SZ).
	Sz(String),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl RegistryValue {
	/// Converts the internal value to a `*const c_void`.
	pub fn as_ptr(&self, buf: &mut WString) -> *const std::ffi::c_void {
		match self {
			Self::Binary(b) => b.as_ptr() as _,
			Self::Dword(n) => *n as _,
			Self::Qword(n) => *n as _,
			Self::Sz(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() as _ }
			},
			Self::None => std::ptr::null(),
		}
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	pub fn reg_type(&self) -> co::REG {
		match self {
			Self::Binary(_) => co::REG::BINARY,
			Self::Dword(_) => co::REG::DWORD,
			Self::Qword(_) => co::REG::QWORD,
			Self::Sz(_) => co::REG::SZ,
			Self::None => co::REG::NONE,
		}
	}

	/// Returns the length of the stored data.
	pub fn len(&self) -> usize {
		match self {
			Self::Binary(b) => b.len(),
			Self::Dword(_) => std::mem::size_of::<u32>(),
			Self::Qword(_) => std::mem::size_of::<u64>(),
			Self::Sz(s) => (s.len() + 1) * std::mem::size_of::<u16>(), // including terminating null
			Self::None => 0,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::EnumResourceTypes`](crate::HINSTANCE::EnumResourceTypes) `func`.
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A string identifier.
	Str(String),
}

impl RtStr {
	pub fn as_ptr(&self, buf: &mut WString) -> *const u16 {
		match self {
			Self::Rt(id) => MAKEINTRESOURCE(id.0 as _),
			Self::Str(s) => {
				*buf = WString::from_str(s);
				unsafe { buf.as_ptr() }
			},
		}
	}
}

/// Variant parameter for:
///
/// * [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT) `hInsertAfter`.
#[derive(Copy, Clone)]
pub enum TreeitemTvi {
	/// Handle to a tree view item.
	Treeitem(HTREEITEM),
	/// One of the predefined values.
	Tvi(co::TVI),
}

impl TreeitemTvi {
	pub fn from_isize(val: isize) -> TreeitemTvi {
		match co::TVI(val) {
			co::TVI::FIRST => Self::Tvi(co::TVI::FIRST),
			co::TVI::LAST => Self::Tvi(co::TVI::LAST),
			co::TVI::ROOT => Self::Tvi(co::TVI::ROOT),
			co::TVI::SORT => Self::Tvi(co::TVI::SORT),
			val => Self::Treeitem(HTREEITEM { ptr: val.0 as _ }),
		}
	}

	pub fn as_isize(&self) -> isize {
		match self {
			Self::Treeitem(htreeitem) => htreeitem.ptr as _,
			Self::Tvi(tvi) => tvi.0 as _,
		}
	}
}
