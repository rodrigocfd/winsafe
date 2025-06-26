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
	pub fn new() -> w::SysResult<Self> {
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
		Ok(new_self)
	}

	fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_init_dialog(move |_| {
			self2
				.txt_path
				.set_text(&format!("{}\\src", ids::ROOT_DIR))?;

			unsafe {
				self2.txt_out.hwnd().SendMessage(msg::wm::SetFont {
					hfont: self2.mono_font.raw_copy(), // fixed-width font for output
					redraw: true,
				});
			}

			Ok(true)
		});

		let self2 = self.clone();
		self.wnd
			.on()
			.wm_command_acc_menu(co::DLGID::CANCEL, move || {
				self2.wnd.close(); // close window on Esc
				Ok(())
			});

		let self2 = self.clone();
		self.btn_run.on().bn_clicked(move || {
			self2.txt_path.hwnd().EnableWindow(false);
			self2.btn_run.hwnd().EnableWindow(false);
			self2.txt_out.set_text("")?;

			let target_dir = self2.txt_path.text()?;
			if !w::path::exists(&target_dir) {
				w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
					hwnd_parent: Some(self2.wnd.hwnd()),
					window_title: Some("Bad path"),
					main_instruction: Some("Process cannot be done"),
					content: Some(&format!("Path does not exist:\n{}", target_dir)),
					main_icon: w::IconIdTd::Td(co::TD_ICON::ERROR),
					common_buttons: co::TDCBF::OK,
					flags: co::TDF::ALLOW_DIALOG_CANCELLATION
						| co::TDF::POSITION_RELATIVE_TO_WINDOW,
					..Default::default()
				})?;
				return Ok(()); // halt processing
			}

			self2.pro_load.set_marquee(true);
			let total_files_count = w::path::dir_walk(&target_dir).count(); // how many files to process?
			self2.pro_load.set_marquee(false);

			self2.pro_load.set_range(0, total_files_count as _); // setup progress bar
			self2.pro_load.set_position(0);

			let titlebar_text = self2.wnd.hwnd().GetWindowText()?;
			self2.wnd.hwnd().SetWindowText(&format!(
				"{} - {} files",
				titlebar_text.split('-').next().unwrap().trim_end(),
				self2.pro_load.range().1,
			))?;

			let self3 = self2.clone();
			let stats = stats::gather(&target_dir, move |pass_idx| {
				self3.pro_load.set_position(pass_idx as _); // process the files
			})?;

			self2.txt_out.set_text(&stats.to_string())?;
			self2.txt_out.focus()?;

			file_repl::ask_update_stats(
				self2.wnd.hwnd(),
				&format!("{}\\README.md", ids::ROOT_DIR),
				&stats,
			)?;

			self2.txt_path.hwnd().EnableWindow(true);
			self2.btn_run.hwnd().EnableWindow(true);
			Ok(())
		});
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}
}
