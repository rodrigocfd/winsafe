use crate::handles::HWND;

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values

/// Base to all native child controls.
#[derive(Clone)]
pub struct NativeControlBase {
	hwnd: HWND,
	ctrl_id: u16, // immutable
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
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}
}