use winsafe::{prelude::*, self as w, co, msg};

use crate::stats::Stats;
use super::WndMain;

impl WndMain {
	pub(super) fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_init_dialog(move |_| {
			let target_dir = "D:\\Stuff\\Core\\rs\\winsafe\\src"; // arbitrary initial dir
			self2.txt_path.set_text(target_dir);

			self2.txt_out.hwnd().SendMessage(msg::wm::SetFont { // fixed-width font for output
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

			let target_dir = self2.txt_path.text();
			if !w::path::exists(&target_dir) {
				w::task_dlg::error(self2.wnd.hwnd(), "Bad path",
					Some("Process cannot be done"),
					&format!("Path does not exist:\n{}", target_dir) )?;
				return Ok(()); // halt processing
			}

			self2.pro_load.set_marquee(true);
			let total_files_count = w::path::dir_walk(&target_dir).count(); // how many files to process?
			self2.pro_load.set_marquee(false);

			self2.pro_load.set_range(0, total_files_count as _); // setup progress bar
			self2.pro_load.set_position(0);

			self2.wnd.set_text(&format!("{} - {} files",
				self2.wnd.text().split('-').next().unwrap().trim_end(), // get app name from titlebar
				self2.pro_load.range().1) );

			let self3 = self2.clone();
			let stats = Stats::gather(&target_dir, move |pass_idx| { // process the files
				self3.pro_load.set_position(pass_idx as _);
			})?;

			self2.txt_out.set_text(&stats.format());
			self2.txt_out.focus();

			self2.txt_path.hwnd().EnableWindow(true);
			self2.btn_run.hwnd().EnableWindow(true);
			Ok(())
		});
	}
}
