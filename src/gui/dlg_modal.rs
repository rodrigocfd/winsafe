use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::WindowEvents;
use crate::gui::traits::Parent;
use crate::handles::HWND;

#[derive(Clone)]
pub struct DlgModal {
	base: Arc<DlgBase>,
}

impl Parent for DlgModal {
	fn hwnd_ref(&self) -> &HWND {
		self.base.hwnd_ref()
	}

	fn user_events_ref(&self) -> &WindowEvents {
		self.base.user_events_ref()
	}

	fn privileged_events_ref(&self) -> &WindowEvents {
		self.base.privileged_events_ref()
	}
}

impl DlgModal {
	pub fn new(parent: &dyn Parent, dialog_id: i32) -> DlgModal {
		let dlg = Self {
			base: Arc::new(
				DlgBase::new(Some(parent), dialog_id),
			),
		};
		dlg.default_message_handlers();
		dlg
	}

	pub fn show_modal(&self) -> WinResult<i32> {
		self.base.dialog_box_param()
	}

	fn default_message_handlers(&self) {
		self.privileged_events_ref().wm_init_dialog({
			let self2 = self.clone();
			move |p| {
				|_| -> WinResult<bool> {
					// Center modal on parent.
					let rc = self2.hwnd_ref().GetWindowRect()?;
					let rc_parent = self2.hwnd_ref().GetParent()?.GetWindowRect()?;
					self2.hwnd_ref().SetWindowPos(
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

		self.user_events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.hwnd_ref().EndDialog(co::DLGID::CANCEL.0 as isize)
					.unwrap_or_else(|err| PostQuitMessage(err))
			}
		});
	}
}
