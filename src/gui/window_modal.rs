use std::any::Any;

use crate::aliases::WinResult;
use crate::gui::base::Base;
use crate::gui::dlg_modal::DlgModal;
use crate::gui::events::WindowEvents;
use crate::gui::raw_modal::{WindowModalOpts, RawModal};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;

#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.rc` script.
///
/// Implements [`Parent`](crate::gui::Parent) trait.
#[derive(Clone)]
pub struct WindowModal {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowModal {}
unsafe impl Sync for WindowModal {}

impl Parent for WindowModal {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowModalOpts) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Raw(
				RawModal::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, dialog_id: i32) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgModal::new(baseref_from_parent(parent), dialog_id),
			),
		}
	}

	pub(crate) fn base_ref(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.base_ref(),
			RawDlg::Dlg(d) => d.base_ref(),
		}
	}

	/// Returns the underlying handle for this window.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.base_ref().hwnd_ref()
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	pub fn on(&self) -> &WindowEvents {
		self.base_ref().user_events_ref()
	}

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> WinResult<i32> {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}
	}
}
