#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::funcs::MAKEDWORD;
use crate::handles::{HBITMAP, HICON, HMENU, HWND};
use crate::structs::{ATOM, NCCALCSIZE_PARAMS, POINT, RECT};
use crate::WString;

/// Variant parameter used in
/// [window class](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-classes)
/// functions:
///
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`FindWindowEx`](crate::HWND::FindWindowEx) `lpszClass`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(WString),
}

impl AtomStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Str(u16) => unsafe { u16.as_ptr() },
			Self::Atom(atom) => atom.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`BM_GETIMAGE`](crate::msg::bm::GetImage) `image`.
pub enum BitmapIcon {
	Bitmap(HBITMAP),
	Icon(HICON),
}

impl BitmapIcon {
	pub fn as_isize(&self) -> isize {
		(match self {
			Self::Bitmap(hbmp) => hbmp.ptr,
			Self::Icon(hicon) => hicon.ptr,
		}) as isize
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `lpNewItem`.
pub enum BitmapPtrStr {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bitmap(HBITMAP),
	/// A string.
	Str(WString),
	/// A pointer to anything.
	Param(*const c_void),
	/// Nothing.
	None,
}

impl BitmapPtrStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Bitmap(hbmp) => hbmp.ptr as _,
			Self::Str(u16) => unsafe { u16.as_ptr() },
			Self::Param(lp) => *lp as _,
			Self::None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`PostMessage`](crate::PostMessage) `hWnd`.
pub enum BroadNull {
	Broadcast,
	Null,
}

impl From<BroadNull> for *mut c_void {
	fn from(v: BroadNull) -> Self {
		match v {
			BroadNull::Broadcast => 0xffff as _,
			BroadNull::Null => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`WM_NEXTDLGCTL`](crate::msg::wm::NextDlgCtl) `hwnd_focus`.
pub enum HwndFocus {
	Hwnd(HWND),
	FocusPrev(bool),
}

/// Variant parameter for:
///
/// * [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) reason.
/// * [`HELPINFO`](crate::HELPINFO) `hItemHandle`.
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
/// * [`SetWindowPos`](crate::HWND::SetWindowPos) `hWndInsertAfter`.
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
	pub fn as_ptr(&self) -> *mut c_void {
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
/// * [`LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`.
pub enum IdIdcStr {
	/// A resource ID.
	Id(i32),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A resource identified by a string.
	Str(WString),
}

impl IdIdcStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => *id as _,
			Self::Idc(idc) => idc.0 as _,
			Self::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.
pub enum IdIdiStr {
	/// A resource ID.
	Id(i32),
	/// A [`co::IDI`](crate::co::IDI) constant for a stock system icon.
	Idi(co::IDI),
	/// A resource identified by a string.
	Str(WString),
}

impl IdIdiStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => *id as _,
			Self::Idi(idi) => idi.0 as _,
			Self::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdMenu {
	/// A command ID.
	Id(i32),
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
	pub fn as_ptr(&self) -> *mut c_void {
		match self {
			Self::Id(id) => *id as _,
			Self::Menu(hMenu) => hMenu.ptr,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HiliteMenuItem`](crate::HWND::HiliteMenuItem) `uIDHiliteItem`;
/// * [`InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`.
#[derive(Copy, Clone)]
pub enum IdPos {
	/// A command ID.
	Id(u16),
	/// Zero-based position.
	Pos(u32),
}

impl IdPos {
	pub fn is_by_pos(self) -> bool {
		match self {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		}
	}

	pub fn id_or_pos_u32(self) -> u32 {
		match self {
			IdPos::Id(id) => id as _,
			IdPos::Pos(pos) => pos,
		}
	}

	pub fn mf_flag(self) -> co::MF {
		match self {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpTemplateName`.
/// * [`LoadAccelerators`](crate::HINSTANCE::LoadAccelerators) `lpTableName`.
/// * [`WNDCLASSEX`](crate::WNDCLASSEX) `lpszMenuName`;
pub enum IdStr {
	/// A resource ID.
	Id(i32),
	/// A resource identified by a string.
	Str(WString),
}

impl IdStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => *id as _,
			Self::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`LB_SETSEL`](crate::msg::lb::SetSel) `index`;
/// * [`LVM_FINDITEM`](crate::msg::lvm::FindItem) `index`.
#[derive(Copy, Clone)]
pub enum IndexAll {
	/// A single item with the given index.
	Index(u32),
	/// All items, same of using -1 as the index.
	All,
}

impl From<IndexAll> for isize {
	fn from(v: IndexAll) -> Self {
		match v {
			IndexAll::All => -1,
			IndexAll::Index(i) => i as _,
		}
	}
}

impl From<IndexAll> for usize {
	fn from(v: IndexAll) -> Self {
		isize::from(v) as _
	}
}

impl From<IndexAll> for u32 {
	fn from(v: IndexAll) -> Self {
		isize::from(v) as _
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
pub enum RegistryValue {
	/// Binary value, defined as [`REG::BINARY`](crate::co::REG::BINARY).
	Binary(Vec<u8>),
	/// An `u32` integer value, defined as [`REG::DWORD`](crate::co::REG::DWORD).
	Dword(u32),
	/// An `u64` integer value, defined as [`REG::QWORD`](crate::co::REG::QWORD).
	Qword(u64),
	/// String value, defined as [`REG::SZ`](crate::co::REG::SZ).
	Sz(WString),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl RegistryValue {
	/// Converts the internal value to a `*const c_void`.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			Self::Binary(b) => b.as_ptr() as _,
			Self::Dword(n) => *n as _,
			Self::Qword(n) => *n as _,
			Self::Sz(u16) => unsafe { u16.as_ptr() as _ },
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
			Self::Sz(u16) => (u16.len() + 1) * std::mem::size_of::<u16>(), // including terminating null
			Self::None => 0,
		}
	}
}
