#![windows_subsystem = "windows"]

mod my_window;
use my_window::MyWindow;

fn main() {
	let my_window = MyWindow::new();
	if let Err(e) = my_window.run() {
		eprintln!("{}", e);
	}
}
