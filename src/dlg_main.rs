use std::rc::Rc;
use winsafe::{self as w, co, gui, msg, prelude::*};

use crate::{file_repl, ids, stats};

#[derive(Clone)]
pub struct DlgMain {
	pub wnd: gui::WindowMain,
	pub txt_path: gui::Edit,
	pub btn_run: gui::Button,
	pub pro_load: gui::ProgressBar,
	pub txt_out: gui::Edit,
	pub mono_font: Rc<w::guard::DeleteObjectGuard<w::HFONT>>,
}

impl DlgMain {
	pub fn create_and_run() -> w::AnyResult<i32> {
		use gui::{Horz as H, Vert as V};

		let wnd = gui::WindowMain::new_dlg(ids::DLG_MAIN, Some(ids::ICO_MAIN), None);
		let txt_path = gui::Edit::new_dlg(&wnd, ids::TXT_PATH, (H::Resize, V::None));
		let btn_run = gui::Button::new_dlg(&wnd, ids::BTN_RUN, (H::Repos, V::None));
		let pro_load = gui::ProgressBar::new_dlg(&wnd, ids::PRO_LOAD, (H::Resize, V::None));
		let txt_out = gui::Edit::new_dlg(&wnd, ids::TXT_OUT, (H::Resize, V::Resize));
		let mono_font = Rc::new({
			let mut lf = w::LOGFONT::default();
			lf.lfHeight = 15;
			lf.set_lfFaceName("Consolas");
			w::HFONT::CreateFontIndirect(&mut lf)
		}?);

		let new_self = Self {
			wnd,
			txt_path,
			btn_run,
			pro_load,
			txt_out,
			mono_font,
		};
		new_self.events();
		new_self.wnd.run_main(None)
	}

	fn events(&self) {
		let self2 = self.clone();
		self.wnd
			.on()
			.wm_init_dialog(move |_| self2.on_init_dialog());

		let self2 = self.clone();
		self.wnd
			.on()
			.wm_command_acc_menu(co::DLGID::CANCEL, move || self2.on_cancel());

		let self2 = self.clone();
		self.btn_run.on().bn_clicked(move || self2.on_btn_run());
	}

	fn on_init_dialog(&self) -> w::AnyResult<bool> {
		self.txt_path.set_text(&format!("{}\\src", ids::ROOT_DIR))?;

		unsafe {
			self.txt_out.hwnd().SendMessage(msg::wm::SetFont {
				hfont: self.mono_font.raw_copy(), // fixed-width font for output
				redraw: true,
			});
		}

		Ok(true)
	}

	fn on_cancel(&self) -> w::AnyResult<()> {
		self.wnd.close(); // close window on Esc
		Ok(())
	}

	fn on_btn_run(&self) -> w::AnyResult<()> {
		self.txt_path.hwnd().EnableWindow(false);
		self.btn_run.hwnd().EnableWindow(false);
		self.txt_out.set_text("")?;

		let target_dir = self.txt_path.text()?;
		if !w::path::exists(&target_dir) {
			w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
				hwnd_parent: Some(self.wnd.hwnd()),
				window_title: Some("Bad path"),
				main_instruction: Some("Process cannot be done"),
				content: Some(&format!("Path does not exist:\n{}", target_dir)),
				main_icon: w::IconIdTd::Td(co::TD_ICON::ERROR),
				common_buttons: co::TDCBF::OK,
				flags: co::TDF::ALLOW_DIALOG_CANCELLATION | co::TDF::POSITION_RELATIVE_TO_WINDOW,
				..Default::default()
			})?;
			return Ok(()); // halt processing
		}

		self.pro_load.set_marquee(true);
		let total_files_count = w::path::dir_walk(&target_dir).count(); // how many files to process?
		self.pro_load.set_marquee(false);

		self.pro_load.set_range(0, total_files_count as _); // setup progress bar
		self.pro_load.set_position(0);

		let titlebar_text = self.wnd.hwnd().GetWindowText()?;
		self.wnd.hwnd().SetWindowText(&format!(
			"{} - {} files",
			titlebar_text.split('-').next().unwrap().trim_end(),
			self.pro_load.range().1,
		))?;

		let stats = stats::gather(&target_dir, |pass_idx| {
			self.pro_load.set_position(pass_idx as _); // process the files
		})?;

		self.txt_out.set_text(&stats.to_string())?;
		self.txt_out.focus()?;

		file_repl::ask_update_stats(
			self.wnd.hwnd(),
			&format!("{}\\README.md", ids::ROOT_DIR),
			&stats,
		)?;

		self.txt_path.hwnd().EnableWindow(true);
		self.btn_run.hwnd().EnableWindow(true);
		Ok(())
	}
}
