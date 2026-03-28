#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dlg_main;
mod file_repl;
mod ids;
mod stats;
mod sysdlg;

fn main() {
	dlg_main::DlgMain::create_and_run().expect("Error boundary.");
}
