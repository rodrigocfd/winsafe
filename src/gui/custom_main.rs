use crate::aliases::WinResult;
use crate::co;
use crate::funcs::{InitCommonControls, IsWindowsVistaOrGreater, SetProcessDPIAware};
use crate::gui::dialog_main::DialogMain;
use crate::gui::events::MsgEvents;
use crate::gui::privs::{create_ui_font, delete_ui_font};
use crate::gui::traits::Parent;
use crate::gui::window_main::{CustomMainOpts, WindowMain};
use crate::handles::HWND;

#[derive(Clone)]
enum WndDlg {
	Wnd(WindowMain),
	Dlg(DialogMain),
}

//------------------------------------------------------------------------------

/// Custom main application window.
///
/// # Examples
///
/// A full application with a non-dialog `CustomMain`:
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe as w;
///
/// fn main() {
///   let my_main = MyMain::new();
///   if let Err(e) = my_main.run() {
///     eprintln!("{}", e);
///   }
/// }
///
/// pub struct MyMain {
///   wnd: w::gui::CustomMain,
/// }
///
/// impl MyMain {
///   pub fn new() -> MyMain {
///     let wnd = w::gui::CustomMain::new(
///       w::gui::CustomMainOpts {
///         title: "My window".to_owned(),
///         ..Default::default()
///       },
///     );
///
///     let me = Self { wnd };
///     me.events();
///     me
///   }
///
///   pub fn run(&self) -> w::WinResult<i32> {
///     self.wnd.run_main(None)
///   }
///
///   fn events(&self) {
///
///   }
/// }
/// ```
///
/// A full application with a `CustomMain` loaded from a dialog resource:
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe as w;
///
/// fn main() {
///   let my_dlg = MyDlg::new();
///   if let Err(e) = my_dlg.run() {
///     eprintln!("{}", e);
///   }
/// }
///
/// pub struct MyDlg {
///   dlg: w::gui::CustomMain,
/// }
///
/// impl MyDlg {
///   pub fn new() -> MyDlg {
///     // 101 is the ID of the dialog resource in the .rc file
///     let dlg = w::gui::CustomMain::new_dlg(101, None, None);
///
///     let me = Self { dlg };
///     me.events();
///     me
///   }
///
///   pub fn run(&self) -> w::WinResult<i32> {
///     self.dlg.run_main(None)
///   }
///
///   fn events(&self) {
///
///   }
/// }
/// ```
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

	fn user_events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.user_events_ref(),
			WndDlg::Dlg(d) => d.user_events_ref(),
		}
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		match &self.0 {
			WndDlg::Wnd(w) => w.privileged_events_ref(),
			WndDlg::Dlg(d) => d.privileged_events_ref(),
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
		self.user_events_ref()
	}

	/// Physically creates the window, then runs the main application loop. This
	/// method will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_main(&self, cmd_show: Option<co::SW>) -> WinResult<()> {
		if IsWindowsVistaOrGreater()? {
			SetProcessDPIAware()?;
		}

		InitCommonControls();
		create_ui_font()?;

		match &self.0 {
			WndDlg::Wnd(w) => w.run_main(cmd_show)?,
			WndDlg::Dlg(d) => d.run_main(cmd_show)?,
		}

		delete_ui_font() // cleanup
	}
}
