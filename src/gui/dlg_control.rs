use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, paint_control_borders};
use crate::structs::POINT;

#[derive(Clone)]
pub struct DlgControl(Arc<Obj>);

struct Obj { // actual fields of DlgControl
	base: DlgBase,
	position: POINT,
	ctrl_id: Option<u16>,
}

impl DlgControl {
	pub fn new(
		parent_ref: &Base,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<u16>) -> DlgControl
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DlgBase::new(Some(parent_ref), dialog_id),
					position,
					ctrl_id,
				},
			),
		);
		dlg.default_message_handlers(parent_ref);
		dlg
	}

	pub fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	fn default_message_handlers(&self, parent_ref: &Base) {
		parent_ref.privileged_events_ref().wm(parent_ref.create_wm(), {
			let self2 = self.clone();
			move |p| {
				|_| -> WinResult<isize> {
					// Create the control.
					self2.0.base.create_dialog_param()?; // may panic

					// Set control position within parent.
					let mut dlg_pos = self2.0.position;
					multiply_dpi(Some(&mut dlg_pos), None)?;
					self2.base_ref().hwnd_ref().SetWindowPos(
						HwndPlace::None,
						dlg_pos.x, dlg_pos.y, 0, 0,
						co::SWP::NOZORDER | co::SWP::NOSIZE,
					)?;

					// Give the control an ID.
					self2.base_ref().hwnd_ref().SetWindowLongPtr(
						co::GWLP::ID,
						self2.0.ctrl_id.unwrap_or_else(|| auto_ctrl_id()) as isize,
					);
					Ok(0)
				}
				(p).unwrap_or_else(|err| { PostQuitMessage(err); 0 })
			}
		});

		self.base_ref().user_events_ref().wm_nc_paint({
			let self2 = self.clone();
			move |p| paint_control_borders(*self2.base_ref().hwnd_ref(), p)
				.unwrap_or_else(|err| PostQuitMessage(err))
		});
	}
}
