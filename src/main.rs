#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dlg_main;
mod file_repl;
mod ids;
mod stats;

fn main() {
	dlg_main::DlgMain::create_and_run().unwrap();
}
