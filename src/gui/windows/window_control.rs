use std::any::Any;

use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::prelude::*;

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowControl(RawDlg);

unsafe impl Send for WindowControl {}

impl AsRef<Base> for WindowControl {
	fn as_ref(&self) -> &Base {
		match &self.0 {
			RawDlg::Raw(r) => r.base(),
			RawDlg::Dlg(d) => d.base(),
		}
	}
}

impl GuiWindow for WindowControl {
	fn hwnd(&self) -> &HWND {
		self.as_ref().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for WindowControl {}

impl GuiChild for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.0 {
			RawDlg::Raw(r) => r.ctrl_id(),
			RawDlg::Dlg(d) => d.ctrl_id(),
		}
	}
}

impl WindowControl {
	/// Instantiates a new `WindowControl` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `WindowControl` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: WindowControlOpts) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a custom child control after the parent window is created.");
		}

		Self(
			RawDlg::Raw(
				RawControl::new(parent, opts),
			),
		)
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::CreateDialogParam`](crate::prelude::user_Hinstance::CreateDialogParam).
	///
	/// If the parent window is a dialog, position is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `WindowControl` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		dialog_id: u16,
		position: POINT,
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>,
	) -> Self
	{
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a custom child control after the parent window is created.");
		}

		Self(
			RawDlg::Dlg(
				DlgControl::new(
					parent,
					dialog_id,
					position,
					resize_behavior,
					ctrl_id,
				),
			),
		)
	}
}
