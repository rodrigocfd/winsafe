use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;

#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Arc<Obj>);

struct Obj { // actual fields of DlgModal
	base: DlgBase,
}

impl DlgModal {
	pub(in crate::gui) fn new(parent_base_ref: &Base, dialog_id: i32) -> DlgModal {
		let dlg = Self(
			Arc::new(
				Obj {
					base: DlgBase::new(Some(parent_base_ref), dialog_id),
				},
			),
		);
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn show_modal(&self) -> WinResult<i32> {
		self.0.base.dialog_box_param()
	}

	fn default_message_handlers(&self) {
		self.base_ref().privileged_events_ref().wm_init_dialog({
			let self2 = self.clone();
			move |p| {
				|_| -> WinResult<bool> {
					// Center modal on parent.
					let hwnd = *self2.base_ref().hwnd_ref();
					let rc = hwnd.GetWindowRect()?;
					let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
					hwnd.SetWindowPos(
						HwndPlace::None,
						rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
						rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
						0, 0,
						co::SWP::NOSIZE | co::SWP::NOZORDER,
					)?;
					Ok(true)
				}
				(p).unwrap_or_else(|err| { PostQuitMessage(err);  true })
			}
		});

		self.base_ref().user_events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.base_ref().hwnd_ref().EndDialog(co::DLGID::CANCEL.0 as _)
					.unwrap_or_else(|err| PostQuitMessage(err))
			}
		});
	}
}
