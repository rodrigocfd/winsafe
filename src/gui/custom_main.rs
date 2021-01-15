use crate::co;
use crate::funcs::{InitCommonControls, IsWindowsVistaOrGreater, SetProcessDPIAware};
use crate::gui::dialog_main::DialogMain;
use crate::gui::events::MsgEvents;
use crate::gui::globals::{create_ui_font, delete_ui_font};
use crate::gui::parent::Parent;
use crate::gui::window_main::{CustomMainOpts, WindowMain};
use crate::handles::HWND;

#[derive(Clone)]
enum WndDlg {
	Wnd(WindowMain),
	Dlg(DialogMain),
}

//------------------------------------------------------------------------------

/// Custom main application window.
#[derive(Clone)]
pub struct CustomMain(WndDlg);

unsafe impl Send for CustomMain {}
unsafe impl Sync for CustomMain {}

impl Parent for CustomMain {
	fn hwnd_ref(&self) -> &HWND {
		match &self.0 {
			WndDlg::Wnd(w) => w.hwnd_ref(),
			WndDlg::Dlg(d) => d.hwnd_ref(),
		}
	}

	fn events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.events_ref(),
			WndDlg::Dlg(d) => d.events_ref(),
		}
	}

	fn add_child_to_be_created(&self,
		func: Box<dyn Fn() -> Result<(), co::ERROR> + 'static>)
	{
		match &self.0 {
			WndDlg::Wnd(w) => w.add_child_to_be_created(func),
			WndDlg::Dlg(d) => d.add_child_to_be_created(func),
		}
	}
}

impl CustomMain {
	/// Instantiates a new `CustomMain` object, to be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(opts: CustomMainOpts) -> CustomMain {
		Self(
			WndDlg::Wnd(
				WindowMain::new(opts),
			),
		)
	}

	/// Instantiates a new `CustomMain` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		dialog_id: i32,
		icon_id: Option<i32>,
		accel_table_id: Option<i32>) -> CustomMain
	{
		Self(
			WndDlg::Dlg(
				DialogMain::new(dialog_id, icon_id, accel_table_id),
			),
		)
	}

	/// Returns the underlying handle for this window.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.hwnd_ref()
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before window
	/// creation.
	pub fn on(&self) -> &MsgEvents {
		self.events_ref()
	}

	/// Physically creates the window, then runs the main application loop. This
	/// method will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_as_main(&self,
		cmd_show: Option<co::SW>) -> Result<i32, co::ERROR>
	{
		if IsWindowsVistaOrGreater()? {
			SetProcessDPIAware()?;
		}

		InitCommonControls();
		create_ui_font()?;

		let maybe_res = match &self.0 {
			WndDlg::Wnd(w) => w.run_as_main(cmd_show),
			WndDlg::Dlg(d) => d.run_as_main(cmd_show),
		};

		delete_ui_font(); // cleanup
		maybe_res
	}
}
