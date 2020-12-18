use crate::handles::HWND;

/// Base to all native child controls.
#[derive(Clone)]
pub struct NativeControlBase {
	hwnd: HWND,
}

impl NativeControlBase {
	pub fn new() -> NativeControlBase {
		Self {
			hwnd: HWND::default(),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}
}