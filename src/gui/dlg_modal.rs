use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::prelude::*;

struct Obj { // actual fields of DlgModal
	dlg_base: DlgBase,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// A dialog-base modal window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModal(Pin<Arc<Obj>>);

impl DlgModal {
	pub(in crate::gui) fn new(
		parent: &impl AsRef<Base>,
		dialog_id: u16,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(Some(parent), dialog_id),
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) fn base(&self) -> &Base {
		self.0.dlg_base.base()
	}

	pub(in crate::gui) fn show_modal(&self) -> AnyResult<i32> {
		self.0.dlg_base.dialog_box_param()
			.map_err(|err| err.into())
	}

	fn default_message_handlers(&self) {
		self.base().before_user_on().wm(co::WM::INITDIALOG, move |hwnd, _| {
			let rc = hwnd.GetWindowRect()?;
			let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
			hwnd.SetWindowPos( // center modal on parent
				HwndPlace::None,
				POINT::new(
					rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
					rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
				),
				SIZE::default(),
				co::SWP::NOSIZE | co::SWP::NOZORDER,
			)?;
			Ok(())
		});

		let self2 = self.clone();
		self.base().on().wm_close(move || { // user clicked the X button
			self2.base().hwnd().EndDialog(0)?;
			Ok(())
		});
	}
}
