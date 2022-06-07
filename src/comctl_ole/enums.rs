use crate::co;
use crate::user::decl::HICON;

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `hFooterIcon`.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
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
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
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
