use winsafe::{prelude::*, self as w, co, msg};

use crate::gather_stats;
use super::WndMain;

impl WndMain {
	pub(super) fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_init_dialog(move |_| {
			self2.txt_path.set_text("D:\\Stuff\\Core\\rs\\winsafe\\src");
			Ok(true)
		});

		let self2 = self.clone();
		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), move || {
			self2.wnd.hwnd().SendMessage(msg::wm::Close {});
			Ok(())
		});

		let self2 = self.clone();
		self.btn_run.on().bn_clicked(move || {
			let target = self2.txt_path.text();
			if !w::path::exists(&target) {
				w::task_dlg::error(self2.wnd.hwnd(), "Bad path",
					Some("Process cannot be done"),
					&format!("Path does not exist:\n{}", target))?;
			} else {
				let stats = gather_stats::process(&target)?;
				self2.txt_out.set_text(&stats);
				self2.txt_out.focus();
			}
			Ok(())
		});
	}
}
