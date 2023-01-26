use winsafe::{prelude::*, self as w, co, msg};

use crate::stats::Stats;
use super::WndMain;

impl WndMain {
	pub(super) fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_init_dialog(move |_| {
			let target = "D:\\Stuff\\Core\\rs\\winsafe\\src";
			self2.txt_path.set_text(target);

			let total = w::path::dir_walk(target).count();
			self2.pro_load.set_range(0, total as _);

			self2.txt_out.hwnd().SendMessage(msg::wm::SetFont {
				hfont: unsafe { self2.mono_font.raw_copy() },
				redraw: true,
			});

			Ok(true)
		});

		let self2 = self.clone();
		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), move || {
			self2.wnd.hwnd().SendMessage(msg::wm::Close {});
			Ok(())
		});

		let self2 = self.clone();
		self.btn_run.on().bn_clicked(move || {
			self2.txt_path.hwnd().EnableWindow(false);
			self2.btn_run.hwnd().EnableWindow(false);
			self2.txt_out.set_text("");

			self2.wnd.set_text(&format!("{} - {} files",
				self2.wnd.text().split('-').next().unwrap().trim_end(),
				self2.pro_load.range().1));

			let target = self2.txt_path.text();
			if !w::path::exists(&target) {
				w::task_dlg::error(self2.wnd.hwnd(), "Bad path",
					Some("Process cannot be done"),
					&format!("Path does not exist:\n{}", target))?;
			} else {
				let self3 = self2.clone();
				let stats = Stats::gather(&target, move |pass| { // process the files
					self3.pro_load.set_position(pass as _);
				})?;

				self2.txt_out.set_text(&stats.format());
				self2.txt_out.focus();
			}

			self2.txt_path.hwnd().EnableWindow(true);
			self2.btn_run.hwnd().EnableWindow(true);
			Ok(())
		});
	}
}
