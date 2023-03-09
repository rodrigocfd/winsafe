use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::WindowEventsAll;
use crate::gui::privs::adjust_modeless_pos;
use crate::kernel::decl::AnyResult;
use crate::prelude::{GuiEvents, user_Hwnd};
use crate::user::decl::{HWND, HwndPlace, POINT, SIZE};

struct Obj { // actual fields of DlgModeless
	dlg_base: DlgBase,
	position: POINT,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-based modeless window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModeless(Pin<Arc<Obj>>);

impl DlgModeless {
	pub(in crate::gui) fn new(
		parent: &Base,
		dialog_id: u16,
		position: POINT,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(parent), dialog_id),
					position,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers(parent);
		new_self
	}

	pub(in crate::gui) unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		self.0.dlg_base.as_base()
	}

	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		self.0.dlg_base.hwnd()
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

	fn default_message_handlers(&self, parent: &Base) {
		let self2 = self.clone();
		self.0.dlg_base.parent().unwrap().privileged_on().wm(parent.creation_msg(), move |_| {
			let hparent = self2.0.dlg_base.parent().unwrap().hwnd();
			self2.0.dlg_base.create_dialog_param()?;
			self2.0.dlg_base.hwnd().ShowWindow(co::SW::SHOW);

			let dlg_pos = adjust_modeless_pos(
				self2.0.dlg_base.parent().unwrap(), hparent, self2.0.position)?;

			self2.hwnd().SetWindowPos(
				HwndPlace::None,
				dlg_pos, SIZE::default(),
				co::SWP::NOZORDER | co::SWP::NOSIZE,
			)?;
			Ok(None) // not meaningful
		});
	}
}
