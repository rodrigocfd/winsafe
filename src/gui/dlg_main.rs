use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::handles::HINSTANCE;
use crate::msg::wm;

#[derive(Clone)]
pub(in crate::gui) struct DlgMain(Arc<Obj>);

struct Obj { // actual fields of DlgMain
	base: DlgBase,
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
					base: DlgBase::new(None, dialog_id),
					icon_id,
					accel_table_id,
				},
			),
		);
		dlg.0.base.ui_thread_message_handler();
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn run_ui_thread<F: FnOnce()>(&self, func: F) {
		self.0.base.run_ui_thread(func);
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>) -> WinResult<()>
	{
		self.0.base.create_dialog_param()?; // may panic
		let hinst = self.base_ref().parent_hinstance()?;
		let haccel = self.0.accel_table_id
			.map(|id| hinst.LoadAccelerators(id)) // resources are automatically freed
			.transpose()?;

		self.set_icon_if_any(hinst)?;
		self.base_ref().hwnd_ref().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		Base::run_main_loop(haccel) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		self.base_ref().user_events_ref().wm_close({
			let self2 = self.clone();
			move || self2.base_ref().hwnd_ref().DestroyWindow()
		});

		self.base_ref().user_events_ref().wm_nc_destroy(
			|| PostQuitMessage(co::ERROR::SUCCESS));
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) -> WinResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			self.base_ref().hwnd_ref().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 16, 16, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::SMALL,
				},
			);

			self.base_ref().hwnd_ref().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 32, 32, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::BIG,
				},
			);
		}

		Ok(())
	}
}
