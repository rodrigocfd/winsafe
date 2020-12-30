use std::ffi::c_void;

use crate::co;
use crate::enums::{AtomStr, IdMenu};
use crate::gui::events::MsgEvents;
use crate::handles::{HINSTANCE, HWND};
use crate::structs::{POINT, SIZE};
use crate::WString;

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values

/// Base to all native child controls.
#[derive(Clone)]
pub struct NativeControlBase {
	hwnd: HWND,
	ctrl_id: u16, // cannot be changed
	subclass_events: MsgEvents,
}

impl NativeControlBase {
	pub fn auto_ctrl_id() -> u16 {
		unsafe {
			let new_id = BASE_CTRL_ID;
			BASE_CTRL_ID += 1;
			new_id
		}
	}

	pub fn new_with_id(ctrl_id: u16) -> NativeControlBase {
		Self {
			hwnd: unsafe { HWND::null_handle() },
			ctrl_id,
			subclass_events: MsgEvents::new(),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}

	pub fn on_subclass(&self) -> MsgEvents {
		self.subclass_events.clone()
	}

	pub fn create_window(
		&self,
		parent: HWND,
		class_name: &str,
		title: Option<&str>,
		pos: POINT,
		sz: SIZE,
		ex_styles: co::WS_EX,
		styles: co::WS) -> Result<HWND, co::ERROR>
	{
		let our_hwnd = HWND::CreateWindowEx(
			ex_styles,
			AtomStr::Str(WString::from_str(class_name)),
			title, styles,
			pos.x, pos.y, sz.cx, sz.cy,
			Some(parent), IdMenu::Id(self.ctrl_id),
			unsafe {
				HINSTANCE::from_ptr(
					parent.GetWindowLongPtr(co::GWLP::HINSTANCE) as *mut c_void
				)
			},
			None,
		)?;

		self.install_subclass_if_needed();
		Ok(our_hwnd)
	}

	fn install_subclass_if_needed(&self) {

	}
}
