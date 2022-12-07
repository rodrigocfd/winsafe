use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::privs::MAKEINTRESOURCE;
use crate::user::decl::HICON;

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG).
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
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
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG).
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
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
/// * [`HWND::TaskDialog`](crate::prelude::comctl_ole_Hwnd::TaskDialog).
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
#[derive(Clone)]
pub enum IdTdiconStr {
	/// No icon.
	None,
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Tdicon(co::TD_ICON),
	/// A resource string identifier.
	Str(String),
}

impl IdTdiconStr {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self, str_buf: &mut WString) -> *const u16 {
		match self {
			Self::None => std::ptr::null(),
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Tdicon(tdi) => MAKEINTRESOURCE(tdi.0 as _),
			Self::Str(s) => {
				*str_buf = WString::from_str(s);
				unsafe { str_buf.as_ptr() }
			},
		}
	}
}
