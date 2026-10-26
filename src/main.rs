#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dlg_main;
mod dlg_main_ev;
mod ids;
mod stats;
mod stats_repl;
mod sysdlg;

fn main() {
	dlg_main::DlgMain::create_and_run().expect("Error boundary.");
}
