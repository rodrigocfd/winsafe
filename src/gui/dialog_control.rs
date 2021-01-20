use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::dialog_base::DialogBase;
use crate::gui::events::MsgEvents;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, paint_control_borders};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
pub struct DialogControl(Arc<Obj>);

struct Obj { // actual fields of DialogControl
	base: DialogBase,
	position: POINT,
	ctrl_id: Option<u16>,
}

impl Parent for DialogControl {
	fn hwnd_ref(&self) -> &HWND {
		self.0.base.hwnd_ref()
	}

	fn user_events_ref(&self) -> &MsgEvents {
		self.0.base.user_events_ref()
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		self.0.base.privileged_events_ref()
	}
}

impl Child for DialogControl {
	fn hctrl_ref(&self) -> &HWND {
		self.hwnd_ref()
	}
}

impl DialogControl {
	pub fn new(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<u16>) -> DialogControl
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DialogBase::new(Some(parent), dialog_id),
					position,
					ctrl_id,
				},
			),
		);
		dlg.default_message_handlers(parent);
		dlg
	}

	fn create(&self) -> WinResult<()> {
		// Create the control.
		self.0.base.create_dialog_param()?; // may panic

		// Set control position within parent.
		let mut dlg_pos = self.0.position;
		multiply_dpi(Some(&mut dlg_pos), None)?;
		self.hwnd_ref().SetWindowPos(HwndPlace::None, dlg_pos.x, dlg_pos.y,
			0, 0, co::SWP::NOZORDER | co::SWP::NOSIZE)?;

		// Give the control an ID.
		self.hwnd_ref().SetWindowLongPtr(co::GWLP::ID,
			self.0.ctrl_id.unwrap_or_else(|| auto_ctrl_id()) as isize);
		Ok(())
	}

	fn default_message_handlers(&self, parent: &dyn Parent) {
		parent.privileged_events_ref().wm_init_dialog({
			let self2 = self.clone();
			move |_| { self2.create().unwrap(); true }
		});

		self.user_events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| { paint_control_borders(*self2.hwnd_ref(), p).ok(); }
		});
	}
}
