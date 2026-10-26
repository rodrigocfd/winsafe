use std::rc::Rc;
use winsafe::{self as w, cmd, co, evp, evt, gui, prelude::*};

use crate::ids;

#[derive(Clone)]
pub struct DlgMain {
	pub wnd: gui::WindowMain,
	pub txt_path: gui::Edit,
	pub btn_run: gui::Button,
	pub pro_load: gui::ProgressBar,
	pub txt_out: gui::Edit,
	pub mono_font: Rc<w::DeleteObjectGuard<w::HFONT>>,
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
		evp!(self, wm_init_dialog, on_init_dialog);
		cmd!(self, co::DLGID::CANCEL, on_esc);
		evt!(self, self.btn_run, bn_clicked, on_btn_run);
	}
}
