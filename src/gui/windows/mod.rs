mod base_wnd;
mod dlg_base;
mod dlg_control;
mod dlg_main;
mod dlg_modal;
mod dlg_modeless;
mod layout;
mod raw_base;
mod raw_control;
mod raw_main;
mod raw_modal;
mod raw_modeless;
mod raw_opts;
mod window_control;
mod window_main;
mod window_message_only;
mod window_modal;
mod window_modeless;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base_wnd::BaseWnd;
	pub(in crate::gui) use super::dlg_base::DlgBase;
	pub(in crate::gui) use super::dlg_control::DlgControl;
	pub(in crate::gui) use super::dlg_main::DlgMain;
	pub(in crate::gui) use super::dlg_modal::DlgModal;
	pub(in crate::gui) use super::dlg_modeless::DlgModeless;
	pub(in crate::gui) use super::layout::Layout;
	pub(in crate::gui) use super::raw_base::RawBase;
	pub(in crate::gui) use super::raw_control::RawControl;
	pub(in crate::gui) use super::raw_main::RawMain;
	pub(in crate::gui) use super::raw_modal::RawModal;
	pub(in crate::gui) use super::raw_modeless::RawModeless;
}

pub mod decl {
	pub use super::layout::{Horz, Vert};
	pub use super::raw_opts::*;
	pub use super::window_control::WindowControl;
	pub use super::window_main::WindowMain;
	pub use super::window_message_only::WindowMessageOnly;
	pub use super::window_modal::WindowModal;
	pub use super::window_modeless::WindowModeless;
}
