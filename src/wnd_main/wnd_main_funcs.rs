use std::rc::Rc;
use winsafe::{prelude::*, self as w, gui};

use crate::ids;
use super::WndMain;

impl WndMain {
	pub fn new() -> w::SysResult<Self> {
		use gui::{Horz as H, Vert as V};

		let wnd = gui::WindowMain::new_dlg(ids::DLG_MAIN, Some(ids::ICO_MAIN), None);
		let txt_path = gui::Edit::new_dlg(&wnd, ids::TXT_PATH, (H::Resize, V::None));
		let btn_run = gui::Button::new_dlg(&wnd, ids::BTN_RUN, (H::Repos, V::None));
		let pro_load = gui::ProgressBar::new_dlg(&wnd, ids::PRO_LOAD, (H::Resize, V::None));
		let txt_out = gui::Edit::new_dlg(&wnd, ids::TXT_OUT, (H::Resize, V::Resize));
		let mono_font = Rc::new(Self::create_mono_font()?);

		let new_self = Self { wnd, txt_path, btn_run, pro_load, txt_out, mono_font };
		new_self.events();
		Ok(new_self)
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn create_mono_font() -> w::SysResult<w::guard::GdiObjectGuard<w::HFONT>> {
		let mut lf = w::LOGFONT::default();
		lf.lfHeight = 15;
		lf.set_lfFaceName("Consolas");
		w::HFONT::CreateFontIndirect(&mut lf)
	}
}
