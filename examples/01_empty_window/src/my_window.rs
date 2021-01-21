use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
	wnd: gui::CustomMain,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let wnd = gui::CustomMain::new(
			gui::CustomMainOpts {
				..Default::default()
			},
		);

		let new_self = Self { wnd };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {

	}
}
