use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEventsAll;
use crate::gui::gui_traits_sealed::{GuiSealedBase, GuiSealedParent};
use crate::gui::raw_control::{RawControl, WindowControlOpts};
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::{ErrResult, WinResult};
use crate::prelude::{AsAny, GuiChild, GuiParent, GuiThread, GuiWindow};
use crate::user::decl::{HWND, POINT};

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

//------------------------------------------------------------------------------

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct WindowControl {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowControl {}

impl AsAny for WindowControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindow for WindowControl {
	fn hwnd(&self) -> HWND {
		self.as_base().hwnd()
	}
}

impl GuiSealedBase for WindowControl {
	fn as_base(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => &r.0.raw_base.base,
			RawDlg::Dlg(d) => &d.0.dlg_base.base,
		}
	}
}

impl GuiSealedParent for WindowControl {
	fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>
	{
		self.as_base().add_to_resizer(hchild, horz, vert)
	}
}

impl GuiParent for WindowControl {
	fn on(&self) -> &WindowEventsAll {
		self.as_base().on()
	}
}

impl GuiChild for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.0.opts.ctrl_id,
			RawDlg::Dlg(d) => d.0.ctrl_id,
		}
	}
}

impl GuiThread for WindowControl {
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

impl WindowControl {
	/// Instantiates a new `WindowControl` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx).
	#[must_use]
	pub fn new(
		parent: &impl GuiParent,
		opts: WindowControlOpts) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(parent.as_base(), opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::UserHwnd::GetDlgItem).
	///
	/// If the parent window is a dialog, position is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		dialog_id: u16,
		position: POINT,
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(
					parent.as_base(),
					dialog_id,
					position,
					resize_behavior,
					ctrl_id,
				),
			),
		}
	}
}
