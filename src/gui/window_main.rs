use std::any::Any;

use crate::co;
use crate::comctl::decl::InitCommonControls;
use crate::ffi_types::BOOL;
use crate::gui::dlg_main::DlgMain;
use crate::gui::events::WindowEventsAll;
use crate::gui::privs::{create_ui_font, delete_ui_font};
use crate::gui::raw_main::{RawMain, WindowMainOpts};
use crate::gui::runtime_error::RunResult;
use crate::kernel::decl::{ErrResult, HPROCESS, IsWindowsVistaOrGreater};
use crate::prelude::{
	GuiParent, GuiThread, GuiWindow, GuiWindowText, kernel_Hprocess,
	user_Hprocess,
};
use crate::user::decl::{HWND, SetProcessDPIAware};

/// Keeps a raw or dialog window.
#[derive(Clone)]
enum RawDlg { Raw(RawMain), Dlg(DlgMain) }

//------------------------------------------------------------------------------

/// An user main window, which can handle events. Usually, this is the first
/// window of your application, launched directly from the `main` function. Can
/// be programmatically created or load a dialog resource from a `.res` file.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct WindowMain(RawDlg);

unsafe impl Send for WindowMain {}

impl GuiWindow for WindowMain {
	fn hwnd(&self) -> HWND {
		match &self.0 {
			RawDlg::Raw(r) => r.hwnd(),
			RawDlg::Dlg(d) => d.hwnd(),
		}
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindowText for WindowMain {}

impl GuiParent for WindowMain {
	fn on(&self) -> &WindowEventsAll {
		match &self.0 {
			RawDlg::Raw(r) => r.on(),
			RawDlg::Dlg(d) => d.on(),
		}
	}

	unsafe fn as_base(&self) -> *mut std::ffi::c_void {
		match &self.0 {
			RawDlg::Raw(r) => r.as_base(),
			RawDlg::Dlg(d) => d.as_base(),
		}
	}
}

impl GuiThread for WindowMain {
	fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		match &self.0 {
			RawDlg::Raw(r) => r.spawn_new_thread(func),
			RawDlg::Dlg(d) => d.spawn_new_thread(func),
		}
	}

	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static
	{
		match &self.0 {
			RawDlg::Raw(r) => r.run_ui_thread(func),
			RawDlg::Dlg(d) => d.run_ui_thread(func),
		}
	}
}

impl WindowMain {
	/// Instantiates a new `WindowMain` object, to be created internally with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	#[must_use]
	pub fn new(opts: WindowMainOpts) -> WindowMain {
		Self(
			RawDlg::Raw(
				RawMain::new(opts),
			),
		)
	}

	/// Instantiates a new `WindowMain` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	#[must_use]
	pub fn new_dlg(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>) -> WindowMain
	{
		Self(
			RawDlg::Dlg(
				DlgMain::new(dialog_id, icon_id, accel_table_id),
			),
		)
	}

	/// Physically creates the window, then runs the main application loop. This
	/// method will block until the window is closed.
	///
	/// The `cmd_show` parameter defaults to
	/// [`co::SW::SHOW`](crate::co::SW::SHOW).
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_main(&self, cmd_show: Option<co::SW>) -> RunResult<i32> {
		if IsWindowsVistaOrGreater().unwrap() {
			SetProcessDPIAware().unwrap();
		}

		InitCommonControls();

		let mut b_val: BOOL = 0; // false
		unsafe {
			HPROCESS::GetCurrentProcess().SetUserObjectInformation( // SetTimer() safety
				co::UOI::TIMERPROC_EXCEPTION_SUPPRESSION, &mut b_val).unwrap();
		}

		create_ui_font();

		let res = match &self.0 {
			RawDlg::Raw(r) => r.run_main(cmd_show),
			RawDlg::Dlg(d) => d.run_main(cmd_show),
		};

		delete_ui_font(); // cleanup
		res
	}
}
