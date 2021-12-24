use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::kernel::decl::WinResult;
use crate::prelude::{GuiEventsView, UserHwnd};
use crate::user::decl::{HwndPlace, POINT, SIZE};

/// A WindowModal with a dialog window.
#[derive(Clone)]
pub(in crate::gui) struct DlgModal(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of DlgModal
	pub(in crate::gui) dlg_base: DlgBase,
}

impl DlgModal {
	pub(in crate::gui) fn new(
		parent_base: &Base,
		dialog_id: u16) -> DlgModal
	{
		let dlg = Self(Arc::new(
			Obj {
				dlg_base: DlgBase::new(Some(parent_base), dialog_id),
			},
		));
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn show_modal(&self) -> WinResult<i32> {
		self.0.dlg_base.dialog_box_param()
	}

	fn default_message_handlers(&self) {
		self.0.dlg_base.base.privileged_on().wm_init_dialog({
			let self2 = self.clone();
			move |_| {
				// Center modal on parent.
				let hwnd = self2.0.dlg_base.base.hwnd();
				let rc = hwnd.GetWindowRect()?;
				let rc_parent = hwnd.GetParent()?.GetWindowRect()?;
				hwnd.SetWindowPos(
					HwndPlace::None,
					POINT::new(
						rc_parent.left + ((rc_parent.right - rc_parent.left) / 2) - (rc.right - rc.left) / 2,
						rc_parent.top + ((rc_parent.bottom - rc_parent.top) / 2) - (rc.bottom - rc.top) / 2,
					),
					SIZE::default(),
					co::SWP::NOSIZE | co::SWP::NOZORDER,
				)?;
				Ok(true)
			}
		});

		self.0.dlg_base.base.on().wm_close({
			let self2 = self.clone();
			move || self2.0.dlg_base.base.hwnd()
				.EndDialog(co::DLGID::CANCEL.0 as _)
				.map_err(|e| e.into())
		});
	}
}
