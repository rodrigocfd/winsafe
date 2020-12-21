use crate::handles::HWND;

/// Base to all native child controls.
#[derive(Clone)]
pub struct NativeControlBase {
	hwnd: HWND,
	ctrl_id: u16,
}

impl NativeControlBase {
	pub fn auto_ctrl_id() -> u16 {
		0
	}

	pub fn new_with_id(ctrl_id: u16) -> NativeControlBase {
		Self {
			hwnd: HWND::default(),
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