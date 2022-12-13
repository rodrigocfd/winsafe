use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEventsAll;
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::raw_control::{RawControl, WindowControlOpts};
use crate::kernel::decl::AnyResult;
use crate::prelude::{GuiChild, GuiParent, GuiThread, GuiWindow};
use crate::user::decl::{HWND, POINT};

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

//------------------------------------------------------------------------------

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowControl(RawDlg);

unsafe impl Send for WindowControl {}

impl GuiWindow for WindowControl {
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

impl GuiParent for WindowControl {
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

impl GuiThread for WindowControl {
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

impl GuiChild for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.0 {
			RawDlg::Raw(r) => r.ctrl_id(),
			RawDlg::Dlg(d) => d.ctrl_id(),
		}
	}
}

impl WindowControl {
	/// Instantiates a new `WindowControl` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(
		parent: &impl GuiParent, opts: WindowControlOpts) -> WindowControl
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Raw(
				RawControl::new(parent_ref, opts),
			),
		)
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
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
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Dlg(
				DlgControl::new(
					parent_ref,
					dialog_id,
					position,
					resize_behavior,
					ctrl_id,
				),
			),
		)
	}
}
