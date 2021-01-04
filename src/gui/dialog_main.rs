use std::cell::UnsafeCell;
use std::error::Error;
use std::sync::Arc;

use crate::co;
use crate::enums::IdStr;
use crate::funcs as f;
use crate::gui::dialog_base::DialogBase;
use crate::gui::events::MsgEvents;
use crate::gui::globals::{create_ui_font, delete_ui_font};
use crate::gui::main_loop::run_loop;
use crate::gui::parent::Parent;
use crate::handles::{HINSTANCE, HWND};
use crate::msg::WmSetIcon;
use crate::priv_funcs::str_dyn_error;

/// Main application dialog.
#[derive(Clone)]
pub struct DialogMain {
	obj: Arc<UnsafeCell<Obj>>,
}

struct Obj { // actual fields of DialogMain
	base: DialogBase,
	dialog_id: i32,
	icon_id: Option<i32>,
	accel_table_id: Option<i32>,
}

unsafe impl Send for DialogMain {}
unsafe impl Sync for DialogMain {}

cref_mref!(DialogMain);

impl Parent for DialogMain {
	fn hwnd_ref(&self) -> &HWND {
		self.cref().base.hwnd()
	}

	fn events_ref(&self) -> &MsgEvents {
		self.cref().base.on()
	}
}

impl DialogMain {
	pub fn new(
		dialog_id: i32,
		icon_id: Option<i32>,
		accel_table_id: Option<i32>) -> DialogMain
	{
		let dlg = Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: DialogBase::new(false),
					dialog_id,
					icon_id,
					accel_table_id,
				},
			)),
		};
		dlg.default_message_handlers();
		dlg
	}

	/// Returns the underlying handle for this dialog.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the dialog is created.
	pub fn hwnd(&self) -> HWND {
		*self.cref().base.hwnd()
	}

	/// Exposes the dialog events.
	///
	/// # Panics
	///
	/// Panics if the dialog is already created. Events must be set before dialog
	/// creation.
	pub fn on(&self) -> &MsgEvents {
		self.cref().base.on()
	}

	/// Creates the dialog and runs the main application loop. This function will
	/// block until the dialog is closed.
	///
	/// # Panics
	///
	/// Panics if the dialog is already created.
	pub fn run_as_main(&self,
		cmd_show: Option<co::SW>) -> Result<i32, Box<dyn Error>>
	{
		if f::IsWindowsVistaOrGreater()
			.map_err(|e| Box::new(e))?
		{
			f::SetProcessDPIAware()
				.map_err(|_| str_dyn_error("SetProcessDPIAware failed."))?;
		}

		f::InitCommonControls();
		create_ui_font()?;

		let hinst = HINSTANCE::GetModuleHandle(None)
			.map_err(|e| Box::new(e))?;
		let our_hwnd = self.cref().base.create_dialog_param( // may panic
			hinst, None, self.cref().dialog_id)?;

		let haccel = match self.cref().accel_table_id {
			None => None,
			Some(id) => Some(hinst.LoadAccelerators(IdStr::Id(id))?),
		};

		self.set_icon_if_any(hinst)?;
		our_hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		let res = run_loop(our_hwnd, haccel)?; // blocks until window is closed
		delete_ui_font(); // cleanup
		Ok(res)
	}

	/// Adds the default event processing.
	fn default_message_handlers(&self) {
		self.on().wm_close({
			let self2 = self.clone();
			move || {
				self2.hwnd().DestroyWindow();
			}
		});

		self.on().wm_nc_destroy(|| {
			f::PostQuitMessage(0);
		});
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) -> Result<(), co::ERROR> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.cref().icon_id {
			self.hwnd().SendMessage(
				WmSetIcon {
					hicon: hinst.LoadImageIcon(IdStr::Id(id), 16, 16, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::SMALL,
				},
			);

			self.hwnd().SendMessage(
				WmSetIcon {
					hicon: hinst.LoadImageIcon(IdStr::Id(id), 32, 32, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::BIG,
				},
			);
		}

		Ok(())
	}
}
