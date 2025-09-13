use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

/// Variant parameter for:
///
/// * [`stm::GetImage`](crate::msg::stm::GetImage)
/// * [`stm::SetImage`](crate::msg::stm::SetImage)
pub enum BmpIconCurMeta {
	/// Bitmap.
	Bmp(HBITMAP),
	/// Icon.
	Icon(HICON),
	/// Cursor.
	Cur(HCURSOR),
	/// Enhanced metafile.
	Meta(HDC),
}

impl BmpIconCurMeta {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		unsafe {
			use BmpIconCurMeta::*;
			std::mem::transmute(match self {
				Bmp(hbmp) => hbmp.ptr(),
				Icon(hicon) => hicon.ptr(),
				Cur(hcur) => hcur.ptr(),
				Meta(hdc) => hdc.ptr(),
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`TBADDBITMAP`](crate::TBADDBITMAP)
pub enum BmpIdbRes {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A system-defined [`co::IDB`](crate::co::IDB) bitmap.
	Idb(co::IDB),
	/// A bitmap resource.
	Res(IdStr, HINSTANCE),
}

/// Variant parameter for:
///
/// * [`TBREPLACEBITMAP`](crate::TBREPLACEBITMAP)
pub enum BmpInstId {
	/// Bitmap handle.
	Bmp(HBITMAP),
	/// Module handle and resource ID.
	InstId(HINSTANCE, u16),
}

/// Variant parameter for:
///
/// * [`HIMAGELIST::DrawEx`](crate::HIMAGELIST::DrawEx)
pub enum ClrDefNone {
	/// A RGB color value.
	Clr(COLORREF),
	/// No color.
	None,
	/// The default color.
	Default,
}

impl ClrDefNone {
	/// Converts the contents into an `u32`.
	#[must_use]
	pub const fn as_u32(&self) -> u32 {
		use ClrDefNone::*;
		match self {
			Clr(c) => c.raw(),
			None => CLR_NONE,
			Default => CLR_DEFAULT,
		}
	}
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG)
pub enum IconId<'a> {
	/// No icon.
	None,
	/// An icon handle.
	Icon(&'a HICON),
	/// A resource ID.
	Id(u16),
}

impl<'a> Default for IconId<'a> {
	fn default() -> Self {
		Self::None
	}
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG)
pub enum IconIdTd<'a> {
	/// No icon.
	None,
	/// An icon handle.
	Icon(&'a HICON),
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Td(co::TD_ICON),
}

impl<'a> Default for IconIdTd<'a> {
	fn default() -> Self {
		Self::None
	}
}

/// Variant parameter for:
///
/// * [`HWND::TaskDialog`](crate::HWND::TaskDialog)
#[derive(Clone)]
pub enum IconRes<'a> {
	/// No icon.
	None,
	/// Handle to the instance to load the icon from, and the icon resource
	/// identifier.
	Res(&'a HINSTANCE, IdStr),
	/// The [`co::TD_ICON::WARNING`](crate::co::TD_ICON::WARNING) constant.
	Warn,
	/// The [`co::TD_ICON::ERROR`](crate::co::TD_ICON::ERROR) constant.
	Error,
	/// The [`co::TD_ICON::INFORMATION`](crate::co::TD_ICON::INFORMATION)
	/// constant.
	Info,
	/// The [`co::TD_ICON::SHIELD`](crate::co::TD_ICON::SHIELD) constant.
	Shield,
}

impl<'a> IconRes<'a> {
	/// Returns the `HINSTANCE`, if any, and a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> (HINSTANCE, *const u16) {
		use IconRes::*;
		match self {
			None => (HINSTANCE::NULL, std::ptr::null()),
			Res(hinst, id_str) => (unsafe { hinst.raw_copy() }, id_str.as_ptr()),
			Warn => (HINSTANCE::NULL, MAKEINTRESOURCE(co::TD_ICON::WARNING.raw() as _)),
			Error => (HINSTANCE::NULL, MAKEINTRESOURCE(co::TD_ICON::ERROR.raw() as _)),
			Info => (HINSTANCE::NULL, MAKEINTRESOURCE(co::TD_ICON::INFORMATION.raw() as _)),
			Shield => (HINSTANCE::NULL, MAKEINTRESOURCE(co::TD_ICON::SHIELD.raw() as _)),
		}
	}
}

/// Variant type for:
///
/// * [`tbm::ChangeBitmap`](crate::msg::tbm::ChangeBitmap)
#[derive(Clone, Copy)]
pub enum IdxCbNone {
	/// Index of an image in the toolbar's image list.
	Idx(u32),
	/// Toolbar will send `TBN_GETDISPINFO` notifications.
	Cb,
	/// Button doesn't have an image.
	None,
}

impl From<IdxCbNone> for isize {
	fn from(v: IdxCbNone) -> Self {
		use IdxCbNone::*;
		match v {
			Idx(idx) => idx as _,
			Cb => I_IMAGECALLBACK,
			None => I_IMAGENONE,
		}
	}
}

/// Variant parameter for:
///
/// * [`TBBUTTON`](crate::TBBUTTON)
#[derive(Clone)]
pub enum IdxStr {
	/// Index of button string.
	Idx(u16),
	/// A string buffer.
	Str(WString),
}

/// Variant parameter for:
///
/// * [`hdm::SetHotDivider`](crate::msg::hdm::SetHotDivider)
pub enum PtIdx {
	/// X and Y coordinates of the pointer
	Pt(POINT),
	/// Index of the divider.
	Idx(u32),
}

/// Variant parameter for:
///
/// * [`tbm::AddString`](crate::msg::tbm::AddString).
pub enum ResStrs {
	/// A resource string resource.
	Res(IdStr, HINSTANCE),
	/// A multi-string composed of null-separated strings. To use this field,
	/// prefer the [`ResStrs::from_strs`](crate::ResStrs::from_strs) static
	/// method.
	Strs(WString),
}

impl ResStrs {
	/// Constructs the enum from a list of strings.
	#[must_use]
	pub fn from_strs(texts: &[impl AsRef<str>]) -> Self {
		Self::Strs(WString::from_str_vec(texts))
	}
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG)
pub enum Tdn {
	ButtonClicked(u16),
	Created,
	Destroyed,
	DialogConstructed,
	ExpandoButtonClicked(bool),
	Help,
	HyperlinkClicked(String),
	Navigated,
	RadioButtonClicked(u16),
	Timer(u32),
	VerificationClicked(bool),
}

impl Tdn {
	/// Constructs the enum from its message data.
	///
	/// # Panics
	///
	/// Panics if `tdn` value is invalid.
	#[must_use]
	pub unsafe fn from_msg(tdn: co::TDN, wp: usize, lp: isize) -> Self {
		match tdn {
			co::TDN::BUTTON_CLICKED => Self::ButtonClicked(wp as _),
			co::TDN::CREATED => Self::Created,
			co::TDN::DESTROYED => Self::Destroyed,
			co::TDN::DIALOG_CONSTRUCTED => Self::DialogConstructed,
			co::TDN::EXPANDO_BUTTON_CLICKED => Self::ExpandoButtonClicked(wp != 0),
			co::TDN::HELP => Self::Help,
			co::TDN::HYPERLINK_CLICKED => {
				Self::HyperlinkClicked(unsafe { WString::from_wchars_nullt(lp as _) }.to_string())
			},
			co::TDN::NAVIGATED => Self::Navigated,
			co::TDN::RADIO_BUTTON_CLICKED => Self::RadioButtonClicked(wp as _),
			co::TDN::TIMER => Self::Timer(wp as _),
			co::TDN::VERIFICATION_CLICKED => Self::VerificationClicked(wp != 0),
			_ => panic!("Invalid TDN value."),
		}
	}
}

/// Variant parameter for:
///
/// * [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT)
pub enum TreeitemTvi {
	/// Handle to a tree view item.
	Treeitem(HTREEITEM),
	/// One of the predefined values.
	Tvi(co::TVI),
}

impl From<TreeitemTvi> for isize {
	fn from(v: TreeitemTvi) -> Self {
		use TreeitemTvi::*;
		match v {
			Treeitem(htreeitem) => htreeitem.ptr() as _,
			Tvi(tvi) => tvi.raw(),
		}
	}
}

impl TreeitemTvi {
	/// Constructs the enum from an `isize`.
	#[must_use]
	pub const fn from_isize(val: isize) -> Self {
		match unsafe { co::TVI::from_raw(val) } {
			co::TVI::FIRST => Self::Tvi(co::TVI::FIRST),
			co::TVI::LAST => Self::Tvi(co::TVI::LAST),
			co::TVI::ROOT => Self::Tvi(co::TVI::ROOT),
			co::TVI::SORT => Self::Tvi(co::TVI::SORT),
			val => Self::Treeitem(unsafe { HTREEITEM::from_ptr(val.raw() as _) }),
		}
	}
}
