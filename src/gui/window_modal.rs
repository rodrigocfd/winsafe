use crate::aliases::WinResult;
use crate::gui::dlg_modal::DlgModal;
use crate::gui::events::WindowEvents;
use crate::gui::raw_modal::{WindowModalOpts, RawModal};
use crate::gui::traits::Parent;
use crate::handles::HWND;

#[derive(Clone)]
enum RawDlg {
	Raw(RawModal),
	Dlg(DlgModal),
}

//------------------------------------------------------------------------------

/// An user modal window, which can handle events.
///
/// A `WindowModal` window can be programatically created or load a dialog
/// resource from a `.rc` script.
#[derive(Clone)]
pub struct WindowModal(RawDlg);

unsafe impl Send for WindowModal {}
unsafe impl Sync for WindowModal {}

impl Parent for WindowModal {
	fn hwnd_ref(&self) -> &HWND {
		match &self.0 {
			RawDlg::Raw(r) => r.hwnd_ref(),
			RawDlg::Dlg(d) => d.hwnd_ref(),
		}
	}

	fn user_events_ref(&self) -> &WindowEvents {
		match &self.0 {
			RawDlg::Raw(r) => r.user_events_ref(),
			RawDlg::Dlg(d) => d.user_events_ref(),
		}
	}

	fn privileged_events_ref(&self) -> &WindowEvents {
		match &self.0 {
			RawDlg::Raw(r) => r.privileged_events_ref(),
			RawDlg::Dlg(d) => d.privileged_events_ref(),
		}
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowModalOpts) -> WindowModal {
		Self(
			RawDlg::Raw(
				RawModal::new(parent, opts),
			),
		)
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, dialog_id: i32) -> WindowModal {
		Self(
			RawDlg::Dlg(
				DlgModal::new(parent, dialog_id),
			),
		)
	}

	/// Returns the underlying handle for this window.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		match &self.0 {
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
		match &self.0 {
			RawDlg::Raw(r) => r.user_events_ref(),
			RawDlg::Dlg(d) => d.user_events_ref(),
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
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}
	}
}
