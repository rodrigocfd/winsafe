use std::sync::Arc;

use crate::co;
use crate::gui::dialog_base::{AfterCreate, DialogBase};
use crate::gui::events::MsgEvents;
use crate::gui::globals::{auto_ctrl_id, paint_control_borders};
use crate::gui::traits::Parent;
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
pub struct DialogControl {
	base: Arc<DialogBase>,
}

impl Parent for DialogControl {
	fn hwnd_ref(&self) -> &HWND {
		self.base.hwnd_ref()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.base.events_ref()
	}
}

impl DialogControl {
	pub fn new(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<u16>) -> DialogControl
	{
		let dlg = Self {
			base: Arc::new(
				DialogBase::new(
					Some(parent),
					dialog_id,
					AfterCreate::ReposSetid(
						position,
						ctrl_id.unwrap_or_else(|| auto_ctrl_id()),
					),
				),
			),
		};
		dlg.default_message_handlers();
		dlg
	}

	pub fn create(&self) -> Result<(), co::ERROR> {
		self.base.create_dialog_param()?; // may panic
		Ok(())
	}

	fn default_message_handlers(&self) {
		self.events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| { paint_control_borders(*self2.hwnd_ref(), p).ok(); }
		});
	}
}
