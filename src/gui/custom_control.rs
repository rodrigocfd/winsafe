use crate::co;
use crate::gui::dialog_control::DialogControl;
use crate::gui::events::MsgEvents;
use crate::gui::parent::Parent;
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

	fn events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.events_ref(),
			WndDlg::Dlg(d) => d.events_ref(),
		}
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> Result<(), co::ERROR> + 'static>)
	{
		match &self.0 {
			WndDlg::Wnd(w) => w.add_child_to_be_created(func),
			WndDlg::Dlg(d) => d.add_child_to_be_created(func),
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
			WndDlg::Wnd(w) => w.events_ref(),
			WndDlg::Dlg(d) => d.events_ref(),
		}
	}
}
