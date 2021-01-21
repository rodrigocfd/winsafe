#![windows_subsystem = "windows"]

mod my_window;
use my_window::MyWindow;

fn main() {
	if let Err(e) = MyWindow::new().run() {
		eprintln!("{}", e);
	}
}
