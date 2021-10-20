use crate::aliases::ErrResult;
use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEventsAll;
use crate::gui::raw_control::{WindowControlOpts, RawControl};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{
	baseref_from_parent,
	Child,
	Parent,
	ParentEvents,
	UiThread,
	Window,
};
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowControl {
	raw_dlg: RawDlg,
}

impl_send_sync!(WindowControl);
impl_debug!(WindowControl);
impl_parent!(WindowControl);

impl_window!(WindowControl);
impl_uithread!(WindowControl);
impl_parentevents!(WindowControl);

impl Child for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.ctrl_id(),
			RawDlg::Dlg(d) => d.ctrl_id(),
		}
	}
}

impl WindowControl {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: WindowControlOpts) -> WindowControl {
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &impl Parent,
		dialog_id: u16,
		position: POINT,
		horz_resize: Horz, vert_resize: Vert,
		ctrl_id: Option<u16>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(
					baseref_from_parent(parent),
					dialog_id,
					position,
					horz_resize, vert_resize,
					ctrl_id,
				),
			),
		}
	}

	fn_base_ref!();
}
