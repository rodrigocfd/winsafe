use std::any::Any;

use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::msg::*;
use crate::prelude::*;

/// Switches between raw and dialog implementations.
///
/// Hierarchy: `BaseWnd` -> `(Raw|Dlg)Base` -> `(Raw|Dlg)Modal` -> `WindowModal`.
#[derive(Clone)]
enum RawDlg {
	Raw(RawModal),
	Dlg(DlgModal),
}

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModal(RawDlg);

unsafe impl Send for WindowModal {}

impl AsRef<BaseWnd> for WindowModal {
	fn as_ref(&self) -> &BaseWnd {
		match &self.0 {
			RawDlg::Raw(r) => r.raw_base().base(),
			RawDlg::Dlg(d) => d.dlg_base().base(),
		}
	}
}

impl GuiWindow for WindowModal {
	fn hwnd(&self) -> &HWND {
		self.as_ref().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowModal {}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	#[must_use]
	pub fn new(opts: WindowModalOpts) -> Self {
		Self(RawDlg::Raw(RawModal::new(opts)))
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::DialogBoxParam`](crate::HINSTANCE::DialogBoxParam).
	#[must_use]
	pub fn new_dlg(dlg_id: u16) -> Self {
		Self(RawDlg::Dlg(DlgModal::new(dlg_id)))
	}

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// Note that, if the user clicks the "X" to close the modal, the default
	/// behavior is to call `EndDialog(0)`. To override this behavior, handle
	/// the modal's [`wm_close`](crate::prelude::GuiEventsWindow::wm_close)
	/// yourself.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	///
	/// Panics if the creation process fails.
	pub fn show_modal(&self, parent: &impl GuiParent) -> AnyResult<()> {
		match &self.0 {
			RawDlg::Raw(r) => r.show_modal(parent),
			RawDlg::Dlg(d) => Ok(d.show_modal(parent)),
		}
	}

	/// Exposes methods to handle window messages.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	pub fn on(&self) -> &impl GuiEventsParent {
		self.as_ref().on()
	}

	/// Closes the window by posting a [`WM_CLOSE`](crate::msg::wm::Close)
	/// message. This is the safest way to close any popup window, because
	/// you'll able to process the
	/// [`wm_close`](crate::prelude::GuiEventsWindow::wm_close) event, just like
	/// if the user clicked the window "X" button.
	pub fn close(&self) {
		unsafe {
			self.hwnd().PostMessage(wm::Close {}).unwrap();
		}
	}
}
