use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

struct DlgMainObj {
	dlg_base: DlgBase,
	icon_id: Option<u16>,
	accel_tbl_id: Option<u16>,
	_pin: PhantomPinned,
}

/// A dialog-based main window.
#[derive(Clone)]
pub(in crate::gui) struct DlgMain(Pin<Arc<DlgMainObj>>);

impl DlgMain {
	#[must_use]
	pub(in crate::gui) fn new(
		dlg_id: u16,
		icon_id: Option<u16>,
		accel_tbl_id: Option<u16>,
	) -> Self {
		let new_self = Self(Arc::pin(DlgMainObj {
			dlg_base: DlgBase::new(dlg_id),
			icon_id,
			accel_tbl_id,
			_pin: PhantomPinned,
		}));
		new_self.default_message_handlers();
		new_self
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.0.dlg_base.base().on().wm_close(move || {
			self2.0.dlg_base.base().hwnd().DestroyWindow().ok(); // ignore errors
			Ok(())
		});

		self.0.dlg_base.base().on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}

	#[must_use]
	pub(in crate::gui) fn dlg_base(&self) -> &DlgBase {
		&self.0.dlg_base
	}

	pub(in crate::gui) fn run_main(
		&self,
		hinst: &HINSTANCE,
		cmd_show: Option<co::SW>,
	) -> AnyResult<i32> {
		self.0.dlg_base.create_dialog_param(hinst)?;
		if let Some(id) = self.0.icon_id {
			self.0.dlg_base.set_icon(hinst, id)?;
		}

		let haccel = self
			.0
			.accel_tbl_id
			.map(|id| hinst.LoadAccelerators(IdStr::Id(id))) // resources are automatically freed
			.transpose()?;

		self.0
			.dlg_base
			.base()
			.hwnd()
			.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));
		BaseWnd::run_main_loop(haccel.as_deref(), true) // blocks until window is closed
	}
}
