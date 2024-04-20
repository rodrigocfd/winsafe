use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of DlgControl
	dlg_base: DlgBase,
	ctrl_id: u16,
	position: POINT,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-based custom control window.
#[derive(Clone)]
pub(in crate::gui) struct DlgControl(Pin<Arc<Obj>>);

impl DlgControl {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &impl AsRef<Base>,
		dialog_id: u16,
		position: POINT,
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(&parent), dialog_id),
					position,
					ctrl_id: ctrl_id.unwrap_or_else(|| next_auto_ctrl_id()),
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent.as_ref(), resize_behavior);
		new_self
	}

	#[must_use]
	pub(in crate::gui) fn base(&self) -> &Base {
		self.0.dlg_base.base()
	}

	#[must_use]
	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}

	fn default_message_handlers(&self,
		parent: &Base,
		resize_behavior: (Horz, Vert),
	) {
		let self2 = self.clone();
		parent.before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.0.dlg_base.create_dialog_param()?;
			let parent_base_ref = self2.base().parent().unwrap();

			let mut dlg_pos = self2.0.position;
			multiply_dpi_or_dtu(parent_base_ref, Some(&mut dlg_pos), None)?;
			self2.base().hwnd().SetWindowPos(
				HwndPlace::None,
				dlg_pos, SIZE::default(),
				co::SWP::NOZORDER | co::SWP::NOSIZE,
			)?;

			unsafe {
				self2.base().hwnd().SetWindowLongPtr(co::GWLP::ID, self2.0.ctrl_id as _); // give ID to the control
			}

			parent_base_ref.add_to_layout_arranger(self2.base().hwnd(), resize_behavior)?;
			Ok(())
		});

		self.base().before_user_on().wm(co::WM::NCPAINT, move |hwnd, p| {
			paint_control_borders(hwnd, wm::NcPaint::from_generic_wm(p))?;
			Ok(())
		});
	}
}
