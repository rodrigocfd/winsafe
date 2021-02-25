use crate::gui::dialog_control::DialogControl;
use crate::gui::events::MsgEvents;
use crate::gui::traits::{Child, Parent};
use crate::gui::window_control::{CustomControlOpts, WindowControl};
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
enum WndDlg {
	Wnd(WindowControl),
	Dlg(DialogControl),
}

//------------------------------------------------------------------------------

/// Custom child control.
///
/// A `CustomControl` window can be programatically created or load a dialog
/// resource from a `.rc` script.
#[derive(Clone)]
pub struct CustomControl(WndDlg);

unsafe impl Send for CustomControl {}
unsafe impl Sync for CustomControl {}

impl Parent for CustomControl {
	fn hwnd_ref(&self) -> &HWND {
		match &self.0 {
			WndDlg::Wnd(w) => w.hwnd_ref(),
			WndDlg::Dlg(d) => d.hwnd_ref(),
		}
	}

	fn user_events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.user_events_ref(),
			WndDlg::Dlg(d) => d.user_events_ref(),
		}
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.privileged_events_ref(),
			WndDlg::Dlg(d) => d.privileged_events_ref(),
		}
	}
}

impl Child for CustomControl {
	fn hctrl_ref(&self) -> &HWND {
		match &self.0 {
			WndDlg::Wnd(w) => w.hwnd_ref(),
			WndDlg::Dlg(d) => d.hwnd_ref(),
		}
	}
}

impl CustomControl {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: CustomControlOpts) -> CustomControl {
		Self(
			WndDlg::Wnd(
				WindowControl::new(parent, opts),
			),
		)
	}

	/// Instantiates a new `CustomControl` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<u16>) -> CustomControl
	{
		Self(
			WndDlg::Dlg(
				DialogControl::new(parent, dialog_id, position, ctrl_id),
			),
		)
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		match &self.0 {
			WndDlg::Wnd(w) => *w.hwnd_ref(),
			WndDlg::Dlg(d) => *d.hwnd_ref(),
		}
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before window
	/// creation.
	pub fn on(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.user_events_ref(),
			WndDlg::Dlg(d) => d.user_events_ref(),
		}
	}
}
