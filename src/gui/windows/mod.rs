//! Windows and dialogs.

mod base;
mod dlg_base;
mod dlg_control;
mod dlg_main;
mod dlg_modal;
mod dlg_modeless;
mod layout_arranger;
mod msg_error;
mod raw_base;
mod raw_control;
mod raw_main;
mod raw_modal;
mod raw_modeless;
mod window_control;
mod window_main;
mod window_message_only;
mod window_modal;
mod window_modeless;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base::Base;
	pub(in crate::gui) use super::dlg_base::DlgBase;
	pub(in crate::gui) use super::dlg_control::DlgControl;
	pub(in crate::gui) use super::dlg_main::DlgMain;
	pub(in crate::gui) use super::dlg_modal::DlgModal;
	pub(in crate::gui) use super::dlg_modeless::DlgModeless;
	pub(in crate::gui) use super::layout_arranger::LayoutArranger;
	pub(in crate::gui) use super::raw_base::RawBase;
	pub(in crate::gui) use super::raw_control::RawControl;
	pub(in crate::gui) use super::raw_main::RawMain;
	pub(in crate::gui) use super::raw_modal::RawModal;
	pub(in crate::gui) use super::raw_modeless::RawModeless;
}

pub mod decl {
	pub use super::base::WmRet;
	pub use super::layout_arranger::{Horz, Vert};
	pub use super::msg_error::MsgError;
	pub use super::raw_base::{Brush, Cursor, Icon};
	pub use super::raw_control::WindowControlOpts;
	pub use super::raw_main::WindowMainOpts;
	pub use super::raw_modal::WindowModalOpts;
	pub use super::raw_modeless::WindowModelessOpts;
	pub use super::window_control::WindowControl;
	pub use super::window_main::WindowMain;
	pub use super::window_message_only::WindowMessageOnly;
	pub use super::window_modal::WindowModal;
	pub use super::window_modeless::WindowModeless;
}
