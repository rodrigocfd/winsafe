use std::ptr::NonNull;
use std::sync::Arc;

use crate::co;
use crate::enums::HwndPlace;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::EventsView;
use crate::gui::privs::{
	auto_ctrl_id,
	multiply_dpi_or_dtu,
	paint_control_borders,
};
use crate::gui::resizer::{Horz, Vert};
use crate::structs::{POINT, SIZE};

/// A WindowControl with a dialog window.
#[derive(Clone)]
pub(in crate::gui) struct DlgControl(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of DlgControl
	pub(in crate::gui) dlg_base: DlgBase,
	pub(in crate::gui) ctrl_id: u16,
	position: POINT,
}

impl DlgControl {
	pub(in crate::gui) fn new(
		parent_base: &Base,
		dialog_id: u16,
		position: POINT,
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>) -> DlgControl
	{
		let dlg = Self(Arc::new(
			Obj {
				dlg_base: DlgBase::new(Some(parent_base), dialog_id),
				position,
				ctrl_id: ctrl_id.unwrap_or_else(|| auto_ctrl_id()),
			},
		));
		dlg.default_message_handlers(parent_base, resize_behavior.0, resize_behavior.1);
		dlg
	}

	fn default_message_handlers(&self,
		parent_base: &Base, horz: Horz, vert: Vert)
	{
		parent_base.privileged_on().wm(parent_base.wmcreate_or_wminitdialog(), {
			let self2 = self.clone();
			let parent_base_ptr = NonNull::from(parent_base);
			move |_| {
				// Create the control.
				self2.0.dlg_base.create_dialog_param()?;

				// Set control position within parent.
				let mut dlg_pos = self2.0.position;
				multiply_dpi_or_dtu(self2.0.dlg_base.base.parent_base().unwrap(),
					Some(&mut dlg_pos), None)?;
				self2.0.dlg_base.base.hwnd().SetWindowPos(
					HwndPlace::None,
					dlg_pos, SIZE::default(),
					co::SWP::NOZORDER | co::SWP::NOSIZE,
				)?;

				// Give the control an ID.
				self2.0.dlg_base.base.hwnd().SetWindowLongPtr(
					co::GWLP::ID,
					self2.0.ctrl_id as _,
				);

				unsafe {
					parent_base_ptr.as_ref().add_to_resizer(
						self2.0.dlg_base.base.hwnd(), horz, vert)?;
				}

				Ok(0)
			}
		});

		self.0.dlg_base.base.on().wm_nc_paint({
			let self2 = self.clone();
			move |p| paint_control_borders(self2.0.dlg_base.base.hwnd(), p)
		});
	}
}
