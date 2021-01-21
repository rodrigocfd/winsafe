use winsafe::gui;
use winsafe::{POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd: gui::CustomMain,
	btn_hello: gui::Button,
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let wnd = gui::CustomMain::new(
			gui::CustomMainOpts {
				title: "Button click".to_owned(),
				size: SIZE::new(300, 200),
				..Default::default()
			},
		);

		let btn_hello = gui::Button::new(
			&wnd, // the parent of our button
			gui::ButtonOpts {
				text: "&Click me".to_owned(),
				position: POINT::new(20, 20),
				..Default::default()
			},
		);

		let new_self = Self { wnd, btn_hello };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		let wnd = self.wnd.clone(); // clone so it can be passed into the closure

		self.btn_hello.on().bn_clicked(move || {
			wnd.hwnd().SetWindowText("Hello, world!").unwrap();
		});
	}
}
