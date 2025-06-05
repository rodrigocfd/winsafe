use std::any::Any;

use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::msg::*;
use crate::prelude::*;

#[derive(Clone)]
enum RawDlg {
	Raw(RawModeless),
	Dlg(DlgModeless),
}

/// An user modeless window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModeless(RawDlg);

unsafe impl Send for WindowModeless {}

impl AsRef<BaseWnd> for WindowModeless {
	fn as_ref(&self) -> &BaseWnd {
		match &self.0 {
			RawDlg::Raw(r) => r.raw_base().base(),
			RawDlg::Dlg(d) => d.dlg_base().base(),
		}
	}
}

impl GuiWindow for WindowModeless {
	fn hwnd(&self) -> &HWND {
		self.as_ref().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowModeless {}

impl WindowModeless {
	/// Instantiates a new `WindowModeless` object, to be created internally
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `WindowModeless` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: WindowModelessOpts) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a modeless window after the parent window is created.");
		}
		Self(RawDlg::Raw(RawModeless::new(parent, opts)))
	}

	/// Instantiates a new `WindowModeless` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::CreateDialogParam`](crate::HINSTANCE::CreateDialogParam).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `WindowModeless` in an event closure.
	#[must_use]
	pub fn new_dlg(parent: &(impl GuiParent + 'static), dlg_id: u16, position: (i32, i32)) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a modeless window after the parent window is created.");
		}
		Self(RawDlg::Dlg(DlgModeless::new(parent, dlg_id, position)))
	}

	/// Closes the window by posting a [`WM_CLOSE`](crate::msg::wm::Close)
	/// message. This is the safest way to close any popup window, because
	/// you'll able to process the
	/// [`wm_close`](crate::gui::events::WindowEvents::wm_close) event, just
	/// like if the user clicked the window "X" button.
	pub fn close(&self) {
		unsafe {
			self.hwnd().PostMessage(wm::Close {}).ok(); // ignore errors
		}
	}
}
