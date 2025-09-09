use std::any::Any;

use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::prelude::*;

/// Switches between raw and dialog implementations.
///
/// Hierarchy: `BaseWnd` -> `(Raw|Dlg)Base` -> `(Raw|Dlg)Control` -> `WindowControl`.
#[derive(Clone)]
enum RawDlg {
	Raw(RawControl),
	Dlg(DlgControl),
}

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
#[derive(Clone)]
pub struct WindowControl(RawDlg);

unsafe impl Send for WindowControl {}

impl AsRef<BaseWnd> for WindowControl {
	fn as_ref(&self) -> &BaseWnd {
		match &self.0 {
			RawDlg::Raw(r) => r.raw_base().base(),
			RawDlg::Dlg(d) => d.dlg_base().base(),
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

impl GuiControl for WindowControl {
	fn ctrl_id(&self) -> u16 {
		match &self.0 {
			RawDlg::Raw(r) => r.ctrl_id(),
			RawDlg::Dlg(d) => d.ctrl_id(),
		}
	}
}

impl WindowControl {
	/// Instantiates a new `WindowControl` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `WindowControl` in an event closure.
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: WindowControlOpts) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a custom child control after the parent window is created.");
		}
		Self(RawDlg::Raw(RawControl::new(parent, opts)))
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with
	/// [`HINSTANCE::CreateDialogParam`](crate::HINSTANCE::CreateDialogParam).
	///
	/// If the parent window is a dialog, position is in Dialog Template Units;
	/// otherwise in pixels, which will be multiplied to match current system
	/// DPI.
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `WindowControl` in an event closure.
	///
	/// Panics if the creation process fails.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		dlg_id: u16,
		position: (i32, i32),
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>,
	) -> Self {
		if *parent.hwnd() != HWND::NULL {
			panic!("Cannot create a custom child control after the parent window is created.");
		}
		Self(RawDlg::Dlg(DlgControl::new(parent, dlg_id, position, resize_behavior, ctrl_id)))
	}

	/// Exposes methods to handle the basic window messages, plus timer and
	/// native control notifications.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	pub fn on(&self) -> &WindowEventsAll {
		self.as_ref().on()
	}
}
