use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::prelude::*;

struct DlgControlObj {
	dlg_base: DlgBase,
	ctrl_id: u16,
	_pin: PhantomPinned,
}

/// A dialog-based custom control window.
#[derive(Clone)]
pub(in crate::gui) struct DlgControl(Pin<Arc<DlgControlObj>>);

impl DlgControl {
	#[must_use]
	pub(in crate::gui) fn new(
		parent: &(impl GuiParent + 'static),
		dlg_id: u16,
		position: (i32, i32),
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>,
	) -> Self
	{
		let ctrl_id2 = ctrl_id.unwrap_or_else(|| auto_id::next());
		let new_self = Self(
			Arc::pin(
				DlgControlObj {
					dlg_base: DlgBase::new(dlg_id),
					ctrl_id: ctrl_id2,
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm(parent.as_ref().is_dlg().create_msg(), move |_| {
			let hinst = parent2.hwnd().hinstance();
			self2.0.dlg_base.create_dialog_param(&hinst)?;
			self2.0.dlg_base.base().hwnd().SetWindowPos(HwndPlace::None,
				position.into(), SIZE::default(), co::SWP::NOZORDER | co::SWP::NOSIZE,
			)?;
			unsafe {
				self2.0.dlg_base.base().hwnd()
					.SetWindowLongPtr(co::GWLP::ID, self2.0.ctrl_id as _); // give ID to the control
			}

			parent2.as_ref().add_to_layout(self2.0.dlg_base.base().hwnd(), resize_behavior)?;
			Ok(0) // ignored
		});

		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.dlg_base.base().before_on().wm_nc_paint(move |p| {
			paint_control_borders(self2.0.dlg_base.base().hwnd(), p)?;
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn dlg_base(&self) -> &DlgBase {
		&self.0.dlg_base
	}

	#[must_use]
	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}
}
