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

/// Custom main application window. Usually, this is the first window of your
/// application, launched directly from the `main` function.
///
/// A `CustomMain` window can be programatically created or load a dialog
/// resource from a `.rc` script.
///
/// # Examples
///
/// ## Programmaticaly creating a window
///
/// Below is a full application based on a non-dialog `CustomMain`, whose
/// instance is kept within `MyMain` struct. This is not necessary, but is
/// highly recommended, because it makes it easier to manage the window
/// contents.
///
/// The `main` function instantiates `MyMain` by calling `MyMain::new`, which
/// then calls `CustomMain::new`. Note how it receives a `CustomMainOpts`
/// argument, who defines all the options.
///
/// The window is handling the
/// [mouse click event](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
/// with a closure, and it displays the clicked coordinates in the
/// [title bar](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw).
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe::{gui, WinResult};
///
/// fn main() {
///     let my_main = MyMain::new();
///     if let Err(e) = my_main.wnd.run_main(None) {
///         eprintln!("{}", e);
///     }
/// }
///
/// pub struct MyMain {
///     wnd: gui::CustomMain,
/// }
///
/// impl MyMain {
///     pub fn new() -> MyMain {
///         let wnd = gui::CustomMain::new(
///             gui::CustomMainOpts {
///                 title: "My window".to_owned(),
///                 ..Default::default()
///             },
///         );
///
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///         let wnd = self.wnd.clone(); // clone so it can be passed into the closure
///
///         self.wnd.on().wm_l_button_down(move |params| {
///             let txt = &format!("Coords {} x {}", params.coords.x, params.coords.y);
///             wnd.hwnd().SetWindowText(txt).unwrap();
///         });
///     }
/// }
/// ```
///
/// ## Loading a window resource from a `.rc` file
///
/// A window can also be loaded from a Win32 resource file (usually an `.rc`
/// file). Below, a full aplication where `CustomMain` loads a window resource,
/// instead of creating the window programatically. Note how
/// `CustomMain::new_dlg` instead of `CustomMain::new`.
///
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe::{gui, WinResult};
///
/// fn main() {
///     let my_main = MyDlg::new();
///     if let Err(e) = my_main.dlg.run_main(None) {
///         eprintln!("{}", e);
///     }
/// }
///
/// pub struct MyDlg {
///     dlg: gui::CustomMain,
/// }
///
/// impl MyDlg {
///     pub fn new() -> MyDlg {
///         // 101 is the ID of the dialog resource in the .rc file
///         let dlg = gui::CustomMain::new_dlg(101, None, None);
///
///         let new_self = Self { dlg };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///
///     }
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
