#![windows_subsystem = "windows"]

mod my_window;
use my_window::MyWindow;

fn main() {
	let my_window = MyWindow::new();  // instantiate our main window
	if let Err(e) = my_window.run() { // ... and run it
		eprintln!("{}", e);
	}
}
