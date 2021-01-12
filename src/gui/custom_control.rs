use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::{Child, Parent};
use crate::gui::window_control::{CustomControlOpts, WindowControl};
use crate::handles::HWND;

#[derive(Clone)]
enum WndDlg {
	Wnd(WindowControl),
}

//------------------------------------------------------------------------------

/// Custom child control.
#[derive(Clone)]
pub struct CustomControl(WndDlg);

unsafe impl Send for CustomControl {}
unsafe impl Sync for CustomControl {}

impl Child for CustomControl {
	fn create(&self) -> Result<(), co::ERROR> {
		match &self.0 {
			WndDlg::Wnd(w) => w.create(),
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

	// /// Instantiates a new `CustomControl` object, to be loaded from a dialog
	// /// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	// pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> CustomControl {

	// }

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		match &self.0 {
			WndDlg::Wnd(w) => w.hwnd(),
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
			WndDlg::Wnd(w) => w.on(),
		}
	}
}
