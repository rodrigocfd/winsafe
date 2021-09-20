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
/// created or load a dialog resource from a `.res` file.
///
/// Implements [`Parent`](crate::gui::Parent) trait.
#[derive(Clone)]
pub struct WindowModal {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowModal {}
unsafe impl Sync for WindowModal {}

impl_debug!(WindowModal);
impl_parent!(WindowModal);

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowModalOpts) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Raw(
				RawModal::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, dialog_id: u16) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgModal::new(baseref_from_parent(parent), dialog_id),
			),
		}
	}

	fn_base_ref!();
	pub_fn_hwnd!();
	pub_fn_on!();
	pub_fn_run_ui_thread!();

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
