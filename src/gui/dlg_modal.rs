use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::structs::{POINT, SIZE};

#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Arc<Obj>);

struct Obj { // actual fields of DlgModal
	base: DlgBase,
}

impl DlgModal {
	pub(in crate::gui) fn new(
		parent_base_ref: &Base, dialog_id: u16) -> DlgModal
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DlgBase::new(Some(parent_base_ref), dialog_id),
				},
			),
		);
		dlg.0.base.ui_thread_message_handler();
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn run_ui_thread<F: FnOnce()>(&self, func: F) {
		self.0.base.run_ui_thread(func);
	}

	pub(in crate::gui) fn show_modal(&self) -> WinResult<i32> {
		self.0.base.dialog_box_param()
	}

	fn default_message_handlers(&self) {
		self.base_ref().privileged_events_ref().wm_init_dialog({
			let self2 = self.clone();
			move |_| {
				// Center modal on parent.
				let hwnd = *self2.base_ref().hwnd_ref();
				let rc = hwnd.GetWindowRect()?;
				let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
				hwnd.SetWindowPos(
					HwndPlace::None,
					POINT::new(
						rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
						rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
					),
					SIZE::default(),
					co::SWP::NOSIZE | co::SWP::NOZORDER,
				)?;
				Ok(true)
			}
		});

		self.base_ref().user_events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.base_ref().hwnd_ref()
					.EndDialog(co::DLGID::CANCEL.0 as _)?;
				Ok(())
			}
		});
	}
}
