use std::any::Any;

use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::prelude::*;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

//------------------------------------------------------------------------------

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModal(RawDlg);

unsafe impl Send for WindowModal {}

impl AsRef<Base> for WindowModal {
	fn as_ref(&self) -> &Base {
		match &self.0 {
			RawDlg::Raw(r) => r.base(),
			RawDlg::Dlg(d) => d.base(),
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

impl GuiWindowText for WindowModal {}

impl GuiParent for WindowModal {}

impl GuiParentPopup for WindowModal {}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: WindowModalOpts) -> Self {
		Self(
			RawDlg::Raw(
				RawModal::new(parent, opts),
			),
		)
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::DialogBoxParam`](crate::prelude::user_Hinstance::DialogBoxParam).
	#[must_use]
	pub fn new_dlg(parent: &impl GuiParent, dialog_id: u16) -> Self {
		Self(
			RawDlg::Dlg(
				DlgModal::new(parent, dialog_id),
			),
		)
	}

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// For a modal created with
	/// [`WindowModal::new`](crate::gui::WindowModal::new), the returned `i32`
	/// is always zero.
	///
	/// For a modal created with
	/// [`WindowModal::new_dlg`](crate::gui::WindowModal::new_dlg), the returned
	/// `i32` is the value passed to
	/// [`HWND::EndDialog`](crate::prelude::user_Hwnd::EndDialog). Note that, if
	/// the user clicks the "X" to close the modal, the default behavior is to
	/// call `EndDialog(0)`. To override this behavior, handle the
	/// modal's [`wm_close`](crate::gui::events::WindowEvents::wm_close)
	/// yourself.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> AnyResult<i32> {
		match &self.0 {
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}
	}
}
