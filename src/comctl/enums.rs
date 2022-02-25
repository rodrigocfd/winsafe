use crate::co;
use crate::comctl::decl::HTREEITEM;
use crate::kernel::decl::{HINSTANCE, IdStr, WString};
use crate::kernel::privs::MAKEINTRESOURCE;
use crate::user::decl::{HBITMAP, HICON, POINT};

/// Variant parameter for:
///
/// * [`TBADDBITMAP`](crate::TBADDBITMAP) `nID`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone)]
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
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `hFooterIcon`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone)]
pub enum IconId {
	/// No icon.
	None,
	/// An icon handle.
	Icon(HICON),
	/// A resource ID.
	Id(u16),
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `hMainIcon`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone)]
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
/// * [`HWND::TaskDialog`](crate::prelude::ComctlOleHwnd::TaskDialog) `pszIcon`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone)]
pub enum IdTdiconStr {
	/// No icon.
	None,
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Tdicon(co::TD_ICON),
	/// A resource string identifier.
	Str(WString),
}

impl IdTdiconStr {
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::None => std::ptr::null(),
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Tdicon(tdi) => MAKEINTRESOURCE(tdi.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`TBBUTTON`](crate::TBBUTTON) `iString`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone)]
pub enum IndexStr {
	Index(u16),
	Str(WString),
}

/// Variant parameter for:
///
/// * [`hdm::SetHotDivider`](crate::msg::hdm::SetHotDivider) `value`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub enum PtIdx {
	/// X and Y coordinates of the pointer
	Pt(POINT),
	/// Index of the divider.
	Idx(u32),
}

/// Variant parameter for:
///
/// * [`tbm::AddString`](crate::msg::tbm::AddString) `texts`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub enum ResStrs {
	/// A resource string resource.
	Res(IdStr, HINSTANCE),
	/// A multi-string composed of null-separated strings. To use this field,
	/// prefer the [`ResStrs::from_strs`](crate::ResStrs::from_strs) static
	/// method.
	Strs(WString),
}

impl ResStrs {
	pub fn from_strs(texts: &[impl AsRef<str>]) -> ResStrs {
		Self::Strs(WString::from_str_vec(texts))
	}
}

/// Variant parameter for:
///
/// * [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT) `hInsertAfter`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[derive(Clone, Copy)]
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
			val => Self::Treeitem(HTREEITEM(val.0 as _)),
		}
	}

	pub fn as_isize(&self) -> isize {
		match self {
			Self::Treeitem(htreeitem) => htreeitem.0 as _,
			Self::Tvi(tvi) => tvi.0 as _,
		}
	}
}
