use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEvents;
use crate::gui::raw_control::{WindowControlOpts, RawControl};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.rc` script.
///
/// Implements [`Parent`](crate::gui::Parent) and [`Child`](crate::gui::Child)
/// traits.
#[derive(Clone)]
pub struct WindowControl {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowControl {}
unsafe impl Sync for WindowControl {}

impl Parent for WindowControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Child for WindowControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl WindowControl {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowControlOpts) -> WindowControl {
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<i32>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(
					baseref_from_parent(parent), dialog_id, position, ctrl_id,
				),
			),
		}
	}

	pub(crate) fn base_ref(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.base_ref(),
			RawDlg::Dlg(d) => d.base_ref(),
		}
	}

	/// Returns the underlying handle for this control.
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
}
