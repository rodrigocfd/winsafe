use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::prelude::*;

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

	pub(in crate::gui) fn base(&self) -> &Base {
		self.0.dlg_base.base()
	}

	fn default_message_handlers(&self, parent: &Base) {
		let self2 = self.clone();
		parent.privileged_on().wm_create_or_initdialog(move |_, _| {
			self2.0.dlg_base.create_dialog_param()?;
			self2.base().hwnd().ShowWindow(co::SW::SHOW);

			let dlg_pos = adjust_modeless_pos(
				self2.base().parent().unwrap(), self2.0.position)?;

			self2.base().hwnd().SetWindowPos(
				HwndPlace::None,
				dlg_pos, SIZE::default(),
				co::SWP::NOZORDER | co::SWP::NOSIZE,
			)?;
			Ok(())
		});
	}
}
