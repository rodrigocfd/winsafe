use winsafe::{prelude::*, self as w, co, msg};

use super::WndMain;

impl WndMain {
	pub(super) fn _events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), move || {
			self2.wnd.hwnd().SendMessage(msg::wm::Close {});
			Ok(())
		});
	}
}
