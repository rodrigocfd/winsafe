use crate::aliases::WinResult;
use crate::gui::dialog_modal::DialogModal;
use crate::gui::events::MsgEvents;
use crate::gui::parent::Parent;
use crate::gui::window_modal::{CustomModalOpts, WindowModal};
use crate::handles::HWND;

#[derive(Clone)]
enum WndDlg {
	Wnd(WindowModal),
	Dlg(DialogModal),
}

//------------------------------------------------------------------------------

/// Custom modal window.
#[derive(Clone)]
pub struct CustomModal(WndDlg);

unsafe impl Send for CustomModal {}
unsafe impl Sync for CustomModal {}

impl Parent for CustomModal {
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
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		match &self.0 {
			WndDlg::Wnd(w) => w.add_child_to_be_created(func),
			WndDlg::Dlg(d) => d.add_child_to_be_created(func),
		}
	}
}

impl CustomModal {
	/// Instantiates a new `CustomModal` object, to be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: CustomModalOpts) -> CustomModal {
		Self(
			WndDlg::Wnd(
				WindowModal::new(parent, opts),
			),
		)
	}

	/// Instantiates a new `CustomModal` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, dialog_id: i32) -> CustomModal {
		Self(
			WndDlg::Dlg(
				DialogModal::new(parent, dialog_id),
			),
		)
	}

	/// Returns the underlying handle for this window.
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

	/// Physically creates the window, then runs the modal loop. This method will
	/// block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> WinResult<i32> {
		match &self.0 {
			WndDlg::Wnd(w) => w.show_modal(),
			WndDlg::Dlg(d) => d.show_modal(),
		}
	}
}
