use std::any::Any;

use crate::aliases::{ErrResult, WinResult};
use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEventsAll;
use crate::gui::raw_control::{RawControl, WindowControlOpts};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits_sealed::{SealedBase, SealedParent};
use crate::gui::traits::{AsAny, Child, Parent, UiThread, Window};
use crate::handles::HWND;
use crate::structs::POINT;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

//------------------------------------------------------------------------------

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
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

impl Window for WindowControl {
	fn hwnd(&self) -> HWND {
		self.as_base().hwnd()
	}
}

impl SealedBase for WindowControl {
	fn as_base(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => &r.0.raw_base.base,
			RawDlg::Dlg(d) => &d.0.dlg_base.base,
		}
	}
}

impl SealedParent for WindowControl {
	fn add_to_resizer(&self,
		hchild: HWND, horz: Horz, vert: Vert) -> WinResult<()>
	{
		self.as_base().add_to_resizer(hchild, horz, vert)
	}
}

impl Parent for WindowControl {
	fn on(&self) -> &WindowEventsAll {
		self.as_base().on()
	}
}

impl Child for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.0.opts.ctrl_id,
			RawDlg::Dlg(d) => d.0.ctrl_id,
		}
	}
}

impl UiThread for WindowControl {
	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()>,
	{
		self.as_base().run_ui_thread(func);
	}
}

impl WindowControl {
	/// Instantiates a new `WindowControl` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: WindowControlOpts) -> WindowControl {
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(parent.as_base(), opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &impl Parent,
		dialog_id: u16,
		position: POINT,
		horz_resize: Horz, vert_resize: Vert,
		ctrl_id: Option<u16>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(
					parent.as_base(),
					dialog_id,
					position,
					horz_resize, vert_resize,
					ctrl_id,
				),
			),
		}
	}
}
