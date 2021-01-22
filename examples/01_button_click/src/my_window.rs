use winsafe::gui;
use winsafe::{POINT, SIZE, WinResult};

#[derive(Clone)]
pub struct MyWindow {
	wnd:       gui::CustomMain, // responsible for managing the window
	btn_hello: gui::Button,     // a button
}

impl MyWindow {
	pub fn new() -> MyWindow {
		let wnd = gui::CustomMain::new( // instantiate the window manager
			gui::CustomMainOpts {
				title: "My window title".to_owned(),
				size: SIZE::new(300, 150),
				..Default::default() // leave all other options as default
			},
		);

		let btn_hello = gui::Button::new(
			&wnd, // the window manager is the parent of our button
			gui::ButtonOpts {
				text: "&Click me".to_owned(),
				position: POINT::new(20, 20),
				..Default::default()
			},
		);

		let new_self = Self { wnd, btn_hello };
		new_self.events(); // attach our events
		new_self
	}

	pub fn run(&self) -> WinResult<()> {
		self.wnd.run_main(None) // simply let the window manager do the hard work
	}

	fn events(&self) {
		let wnd = self.wnd.clone(); // clone so it can be passed into the closure

		self.btn_hello.on().bn_clicked(move || {
			wnd.hwnd().SetWindowText("Hello, world!").unwrap();
		});
	}
}
