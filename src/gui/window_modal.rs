use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_modal::DlgModal;
use crate::gui::events::WindowEventsAll;
use crate::gui::raw_modal::{RawModal, WindowModalOpts};
use crate::kernel::decl::AnyResult;
use crate::prelude::{GuiParent, GuiThread, GuiWindow, GuiWindowText};
use crate::user::decl::HWND;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

//------------------------------------------------------------------------------

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModal(RawDlg);

unsafe impl Send for WindowModal {}

impl GuiWindow for WindowModal {
	fn hwnd(&self) -> &HWND {
		match &self.0 {
			RawDlg::Raw(r) => r.hwnd(),
			RawDlg::Dlg(d) => d.hwnd(),
		}
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for WindowModal {}

impl GuiParent for WindowModal {
	fn on(&self) -> &WindowEventsAll {
		match &self.0 {
			RawDlg::Raw(r) => r.on(),
			RawDlg::Dlg(d) => d.on(),
		}
	}

	unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		match &self.0 {
			RawDlg::Raw(r) => r.as_base(),
			RawDlg::Dlg(d) => d.as_base(),
		}
	}
}

impl GuiThread for WindowModal {
	fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		match &self.0 {
			RawDlg::Raw(r) => r.spawn_new_thread(func),
			RawDlg::Dlg(d) => d.spawn_new_thread(func),
		}
	}

	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		match &self.0 {
			RawDlg::Raw(r) => r.run_ui_thread(func),
			RawDlg::Dlg(d) => d.run_ui_thread(func),
		}
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: WindowModalOpts) -> WindowModal {
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Raw(
				RawModal::new(parent_ref, opts),
			),
		)
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	#[must_use]
	pub fn new_dlg(parent: &impl GuiParent, dialog_id: u16) -> WindowModal {
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Dlg(
				DlgModal::new(parent_ref, dialog_id),
			),
		)
	}

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> i32 {
		match &self.0 {
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}.unwrap()
	}
}
