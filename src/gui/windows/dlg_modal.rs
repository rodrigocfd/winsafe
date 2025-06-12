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
///
/// Hierarchy: `BaseWnd` -> `RawBase` -> `DlgModal`.
#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Pin<Arc<DlgModalObj>>);

impl DlgModal {
	#[must_use]
	pub(in crate::gui) fn new(dlg_id: u16) -> Self {
		let new_self = Self(Arc::pin(DlgModalObj {
			dlg_base: DlgBase::new(dlg_id),
			_pin: PhantomPinned,
		}));
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.dlg_base.base().before_on().wm_init_dialog(move |_| {
			let hwnd = self2.0.dlg_base.base().hwnd();
			let rc = hwnd.GetWindowRect().expect(DONTFAIL);
			let rc_parent = hwnd
				.GetParent()
				.expect(DONTFAIL)
				.GetWindowRect()
				.expect(DONTFAIL);

			hwnd.SetWindowPos(
				HwndPlace::None,
				POINT::with(
					rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) // center modal on parent
						- (rc.right - rc.left) / 2,
					rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2)
						- (rc.bottom - rc.top) / 2,
				),
				SIZE::default(),
				co::SWP::NOSIZE | co::SWP::NOZORDER,
			)
			.expect(DONTFAIL);

			Ok(false) // return value is discarded
		});

		let self2 = self.clone();
		self.0.dlg_base.base().on().wm_close(move || {
			self2.0.dlg_base.base().hwnd().EndDialog(0).expect(DONTFAIL); // user clicked the X button
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn dlg_base(&self) -> &DlgBase {
		&self.0.dlg_base
	}

	pub(in crate::gui) fn show_modal(&self, parent: &impl GuiParent) {
		let hinst = parent.hwnd().hinstance();
		self.0
			.dlg_base
			.dialog_box_param(&hinst, Some(parent.hwnd()));
	}
}
