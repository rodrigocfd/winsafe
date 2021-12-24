use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_modal::DlgModal;
use crate::gui::events::WindowEventsAll;
use crate::gui::gui_traits_sealed::{GuiSealedBase, GuiSealedParent};
use crate::gui::raw_modal::{RawModal, WindowModalOpts};
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::{ErrResult, WinResult};
use crate::prelude::{AsAny, GuiParent, GuiThread, GuiWindow};
use crate::user::decl::HWND;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

//------------------------------------------------------------------------------

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct WindowModal {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowModal {}

impl AsAny for WindowModal {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindow for WindowModal {
	fn hwnd(&self) -> HWND {
		self.as_base().hwnd()
	}
}

impl GuiSealedBase for WindowModal {
	fn as_base(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => &r.0.raw_base.base,
			RawDlg::Dlg(d) => &d.0.dlg_base.base,
		}
	}
}

impl GuiSealedParent for WindowModal {
	fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>
	{
		self.as_base().add_to_resizer(hchild, horz, vert)
	}
}

impl GuiParent for WindowModal {
	fn on(&self) -> &WindowEventsAll {
		self.as_base().on()
	}
}

impl GuiThread for WindowModal {
	fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.as_base().spawn_new_thread(func);
	}

	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		self.as_base().run_ui_thread(func);
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx).
	pub fn new(parent: &impl GuiParent, opts: WindowModalOpts) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Raw(
				RawModal::new(parent.as_base(), opts),
			),
		}
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::prelude::UserHwnd::GetDlgItem).
	pub fn new_dlg(parent: &impl GuiParent, dialog_id: u16) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgModal::new(parent.as_base(), dialog_id),
			),
		}
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
