#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dlg_main;
mod file_repl;
mod ids;
mod stats;

use dlg_main::DlgMain;

fn main() {
	if let Err(e) = (|| DlgMain::new()?.run())() {
		eprintln!("{}", e);
	}
}
