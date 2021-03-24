use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEvents;
use crate::gui::raw_control::{WindowControlOpts, RawControl};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::structs::POINT;

/// An user child window, which can handle events.
///
/// A `WindowControl` window can be programatically created or load a dialog
/// resource from a `.rc` script.
#[derive(Clone)]
pub struct WindowControl {
	raw_dlg: RawDlg,
}

#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

unsafe impl Send for WindowControl {}
unsafe impl Sync for WindowControl {}

impl Parent for WindowControl {
	fn hwnd_ref(&self) -> &HWND {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.hwnd_ref(),
			RawDlg::Dlg(d) => d.hwnd_ref(),
		}
	}

	fn user_events_ref(&self) -> &WindowEvents {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.user_events_ref(),
			RawDlg::Dlg(d) => d.user_events_ref(),
		}
	}

	fn privileged_events_ref(&self) -> &WindowEvents {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.privileged_events_ref(),
			RawDlg::Dlg(d) => d.privileged_events_ref(),
		}
	}
}

impl Child for WindowControl {
	fn hctrl_ref(&self) -> &HWND {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.hwnd_ref(),
			RawDlg::Dlg(d) => d.hwnd_ref(),
		}
	}
}

impl WindowControl {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowControlOpts) -> WindowControl {
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(parent, opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<u16>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(parent, dialog_id, position, ctrl_id),
			),
		}
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		match &self.raw_dlg {
			RawDlg::Raw(r) => *r.hwnd_ref(),
			RawDlg::Dlg(d) => *d.hwnd_ref(),
		}
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before window
	/// creation.
	pub fn on(&self) -> &WindowEvents {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.user_events_ref(),
			RawDlg::Dlg(d) => d.user_events_ref(),
		}
	}
}
