use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::dialog_base::{AfterCreate, DialogBase};
use crate::gui::events::MsgEvents;
use crate::gui::parent::Parent;
use crate::handles::HWND;

#[derive(Clone)]
pub struct DialogModal {
	base: Arc<DialogBase>,
}

impl Parent for DialogModal {
	fn hwnd_ref(&self) -> &HWND {
		self.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.base.events_ref()
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> WinResult<()> + 'static>)
	{
		self.base.add_child_to_be_created(func);
	}
}

impl DialogModal {
	pub fn new(parent: &dyn Parent, dialog_id: i32) -> DialogModal {
		let dlg = Self {
			base: Arc::new(
				DialogBase::new(
					Some(parent),
					dialog_id,
					AfterCreate::CenterOnParent,
				),
			),
		};
		dlg.default_message_handlers();
		dlg
	}

	pub fn show_modal(&self) -> WinResult<i32> {
		self.base.dialog_box_param()
	}

	fn default_message_handlers(&self) {
		self.events_ref().wm_close({
			let self2 = self.clone();
			move || {
				self2.hwnd_ref().EndDialog(
					u16::from(co::DLGID::CANCEL) as isize,
				).ok();
			}
		});
	}
}
