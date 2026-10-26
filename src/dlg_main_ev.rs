use winsafe::{self as w, msg, prelude::*};

use crate::{dlg_main::DlgMain, ids, stats, stats_repl, sysdlg};

impl DlgMain {
	pub fn on_init_dialog(&self, _: msg::WmInitDialog) -> w::AnyResult<bool> {
		self.txt_path.set_text(&format!("{}\\src", ids::ROOT_DIR))?;

		unsafe {
			self.txt_out.hwnd().SendMessage(msg::WmSetFont {
				hfont: self.mono_font.raw_copy(), // fixed-width font for output
				redraw: true,
			});
		}

		Ok(true)
	}

	pub fn on_esc(&self) -> w::AnyResult<()> {
		self.wnd.close(); // close window on Esc
		Ok(())
	}

	pub fn on_btn_run(&self) -> w::AnyResult<()> {
		self.txt_path.hwnd().EnableWindow(false);
		self.btn_run.hwnd().EnableWindow(false);
		self.txt_out.set_text("")?;

		let target_dir = w::path::rtrim_backslash(&self.txt_path.text()?).to_owned();
		if !w::path::exists(&target_dir) {
			sysdlg::err(
				self.wnd.hwnd(),
				"Bad path",
				&format!("Process cannot be done.\nPath does not exist:\n{}", target_dir),
			);
			self.txt_path.hwnd().EnableWindow(true);
			self.btn_run.hwnd().EnableWindow(true);
			self.txt_path.focus()?;
			return Ok(()); // halt processing
		}

		self.pro_load.set_marquee(true);
		let rs_files_list = match stats::rs_files_list(&target_dir) {
			Err(err) => {
				sysdlg::err(
					self.wnd.hwnd(),
					"File listing",
					&format!("Failed to list .rs files:\n{}", err.to_string()),
				);
				return Ok(()); // halt processing
			},
			Ok(f) => f,
		};
		self.pro_load.set_marquee(false);

		self.pro_load.set_range(0, rs_files_list.len() as _); // setup progress bar
		self.pro_load.set_position(0);

		let titlebar_text = self.wnd.hwnd().GetWindowText()?;
		self.wnd.hwnd().SetWindowText(&format!(
			"{} - {} files",
			titlebar_text.split('-').next().unwrap().trim_end(),
			self.pro_load.range().1,
		))?;

		let stats = stats::gather(&rs_files_list, |pass_idx| {
			self.pro_load.set_position(pass_idx as _); // process the files
		})?;

		self.txt_out.set_text(&stats.to_string())?;
		self.txt_out.focus()?;

		stats_repl::ask_update_stats(
			self.wnd.hwnd(),
			&format!("{}\\..\\README.md", target_dir),
			&stats,
		)?;

		self.txt_path.hwnd().EnableWindow(true);
		self.btn_run.hwnd().EnableWindow(true);
		Ok(())
	}
}
