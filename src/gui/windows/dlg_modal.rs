use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::prelude::*;

struct DlgModalObj {
	dlg_base: DlgBase,
	_pin: PhantomPinned,
}

/// A dialog-based modal window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Pin<Arc<DlgModalObj>>);

impl DlgModal {
	#[must_use]
	pub(in crate::gui) fn new(dlg_id: u16) -> Self {
		let new_self = Self(
			Arc::pin(
				DlgModalObj {
					dlg_base: DlgBase::new(dlg_id),
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.dlg_base.base().before_on().wm_init_dialog(move |_| {
			let hwnd = self2.0.dlg_base.base().hwnd();
			let rc = hwnd.GetWindowRect()?;
			let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
			hwnd.SetWindowPos( // center modal on parent
				HwndPlace::None,
				POINT::new(
					rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
					rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
				),
				SIZE::default(),
				co::SWP::NOSIZE | co::SWP::NOZORDER,
			)?;
			Ok(false) // return value is discarded
		});

		let self2 = self.clone();
		self.0.dlg_base.base().on().wm_close(move || { // user clicked the X button
			self2.0.dlg_base.base().hwnd().EndDialog(0)?;
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn dlg_base(&self) -> &DlgBase {
		&self.0.dlg_base
	}

	pub(in crate::gui) fn show_modal(&self, parent: &impl GuiParent) -> AnyResult<()> {
		let hinst = parent.hwnd().hinstance();
		self.0.dlg_base.dialog_box_param(&hinst, parent.hwnd())?;
		Ok(())
	}
}
