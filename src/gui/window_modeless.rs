use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_modeless::DlgModeless;
use crate::gui::events::WindowEventsAll;
use crate::gui::raw_modeless::{RawModeless, WindowModelessOpts};
use crate::kernel::decl::AnyResult;
use crate::prelude::{GuiParent, GuiThread, GuiWindow, GuiWindowText, Handle};
use crate::user::decl::{HWND, POINT};

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawModeless), Dlg(DlgModeless) }

//------------------------------------------------------------------------------

/// An user modeless window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowModeless(RawDlg);

unsafe impl Send for WindowModeless {}

impl GuiWindow for WindowModeless {
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

impl GuiWindowText for WindowModeless {}

impl GuiParent for WindowModeless {
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

impl GuiThread for WindowModeless {
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

impl WindowModeless {
	/// Instantiates a new `WindowModeless` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `WindowModeless` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: WindowModelessOpts) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a modeless window after the parent window is created.");
		}

		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Raw(
				RawModeless::new(parent_ref, opts),
			),
		)
	}

	/// Instantiates a new `WindowModeless` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// If the parent window is a dialog, position is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `WindowModeless` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		dialog_id: u16,
		position: POINT,
	) -> Self
	{
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a modeless window after the parent window is created.");
		}

		let parent_ref = unsafe { Base::from_guiparent(parent) };
		Self(
			RawDlg::Dlg(
				DlgModeless::new(
					parent_ref,
					dialog_id,
					position,
				),
			),
		)
	}
}
