use winsafe::gui;

mod ids;
mod wnd_main_events;
mod wnd_main_funcs;

#[derive(Clone)]
pub struct WndMain {
	wnd:      gui::WindowMain,
	txt_path: gui::Edit,
	btn_run:  gui::Button,
	pro_load: gui::ProgressBar,
	txt_out:  gui::Edit,
}
