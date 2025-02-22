use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of DlgMain
	dlg_base: DlgBase,
	icon_id: Option<u16>,
	accel_table_id: Option<u16>,
	_pin: PhantomPinned,
}

/// A dialog-based main window.
#[derive(Clone)]
pub(in crate::gui) struct DlgMain(Pin<Arc<Obj>>);

impl DlgMain {
	#[must_use]
	pub(in crate::gui) fn new(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>,
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					dlg_base: DlgBase::new(None::<&WindowMain>, dialog_id),
					icon_id,
					accel_table_id,
					_pin: PhantomPinned,
				},
			),
		);
		new_self.default_message_handlers();
		new_self
	}

	#[must_use]
	pub(in crate::gui) fn base(&self) -> &Base {
		self.0.dlg_base.base()
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>,
	) -> AnyResult<i32>
	{
		self.0.dlg_base.create_dialog_param().unwrap();
		let hinst = HINSTANCE::GetModuleHandle(None).unwrap();
		let haccel = self.0.accel_table_id
			.map(|id| hinst.LoadAccelerators(IdStr::Id(id))) // resources are automatically freed
			.transpose()
			.unwrap();

		self.set_icon_if_any(&hinst).unwrap();
		self.base().hwnd().ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		Base::run_main_loop(haccel.as_deref(), true) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		let self2 = self.clone();
		self.base().on().wm_close(move || {
			self2.base().hwnd().DestroyWindow().ok(); // ignore errors
			Ok(())
		});

		self.base().on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}

	fn set_icon_if_any(&self, hinst: &HINSTANCE) -> SysResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			unsafe {
				self.base().hwnd().SendMessage(wm::SetIcon {
					hicon: hinst.LoadImageIcon(
						IdOicStr::Id(id), SIZE::new(16, 16), co::LR::DEFAULTCOLOR)?.leak(),
					size: co::ICON_SZ::SMALL,
				});

				self.base().hwnd().SendMessage(wm::SetIcon {
					hicon: hinst.LoadImageIcon(
						IdOicStr::Id(id), SIZE::new(32, 32), co::LR::DEFAULTCOLOR)?.leak(),
					size: co::ICON_SZ::BIG,
				});
			}
		}
		Ok(())
	}
}
