use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::WindowEventsAll;
use crate::gui::layout_arranger::{Horz, Vert};
use crate::gui::privs::{
	auto_ctrl_id, multiply_dpi_or_dtu, paint_control_borders,
};
use crate::kernel::decl::AnyResult;
use crate::prelude::{GuiEvents, user_Hwnd};
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};

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
	pub(in crate::gui) fn new(
		parent: &Base,
		dialog_id: u16,
		position: POINT,
		resize_behavior: (Horz, Vert),
		ctrl_id: Option<u16>) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(parent), dialog_id),
					position,
					ctrl_id: ctrl_id.unwrap_or_else(|| auto_ctrl_id()),
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(
			parent, resize_behavior.0, resize_behavior.1);
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.dlg_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> HWND {
		self.0.dlg_base.hwnd()
	}

	pub(in crate::gui) fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}

	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		self.0.dlg_base.on()
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.0.dlg_base.spawn_new_thread(func);
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static
	{
		self.0.dlg_base.run_ui_thread(func);
	}

	fn default_message_handlers(&self, parent: &Base, horz: Horz, vert: Vert) {
		let self2 = self.clone();
		self.0.dlg_base.parent().unwrap().privileged_on().wm(parent.creation_msg(), move |_| {
			// Create the control.
			self2.0.dlg_base.create_dialog_param();

			// Set control position within parent.
			let mut dlg_pos = self2.0.position;
			multiply_dpi_or_dtu(
				self2.0.dlg_base.parent().unwrap(), Some(&mut dlg_pos), None);
			self2.hwnd().SetWindowPos(
				HwndPlace::None,
				dlg_pos, SIZE::default(),
				co::SWP::NOZORDER | co::SWP::NOSIZE,
			).unwrap();

			// Give the control an ID.
			self2.hwnd().SetWindowLongPtr(co::GWLP::ID, self2.0.ctrl_id as _);

			self2.0.dlg_base.parent().unwrap()
				.add_to_layout_arranger(self2.hwnd(), horz, vert);
			Ok(None) // not meaningful
		});

		let self2 = self.clone();
		self.on().wm_nc_paint(move |p| {
			paint_control_borders(self2.hwnd(), p);
			Ok(())
		});
	}
}
