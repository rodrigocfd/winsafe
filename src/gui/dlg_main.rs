use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::kernel::decl::{ErrResult, HINSTANCE, IdStr, WinResult};
use crate::msg::wm;
use crate::prelude::{GuiEventsView, KernelHinstance, UserHinstance, UserHwnd};
use crate::user::decl::PostQuitMessage;

/// A WindowMain with a dialog window.
#[derive(Clone)]
pub(in crate::gui) struct DlgMain(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of DlgMain
	pub(in crate::gui) dlg_base: DlgBase,
	icon_id: Option<u16>,
	accel_table_id: Option<u16>,
}

impl DlgMain {
	pub(in crate::gui) fn new(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>) -> DlgMain
	{
		let dlg = Self(
			Arc::new(
				Obj {
					dlg_base: DlgBase::new(None, dialog_id),
					icon_id,
					accel_table_id,
				},
			),
		);
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>) -> ErrResult<i32>
	{
		self.0.dlg_base.create_dialog_param()?;
		let hinst = HINSTANCE::GetModuleHandle(None)?;
		let haccel = self.0.accel_table_id
			.map(|id| hinst.LoadAccelerators(IdStr::Id(id))) // resources are automatically freed
			.transpose()?;

		self.set_icon_if_any(hinst)?;
		self.0.dlg_base.base.hwnd().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		Base::run_main_loop(haccel) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		self.0.dlg_base.base.on().wm_close({
			let self2 = self.clone();
			move || { self2.0.dlg_base.base.hwnd().DestroyWindow()?; Ok(()) }
		});

		self.0.dlg_base.base.on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) -> WinResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			self.0.dlg_base.base.hwnd().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 16, 16, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::SMALL,
				},
			);

			self.0.dlg_base.base.hwnd().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 32, 32, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::BIG,
				},
			);
		}

		Ok(())
	}
}
