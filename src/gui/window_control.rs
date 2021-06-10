use std::any::Any;

use crate::gui::base::Base;
use crate::gui::dlg_control::DlgControl;
use crate::gui::events::WindowEvents;
use crate::gui::raw_control::{WindowControlOpts, RawControl};
use crate::gui::traits::{baseref_from_parent, Child, Parent};
use crate::handles::HWND;
use crate::structs::POINT;

#[derive(Clone)]
enum RawDlg { Raw(RawControl), Dlg(DlgControl) }

/// An user child window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
///
/// Implements [`Parent`](crate::gui::Parent) and [`Child`](crate::gui::Child)
/// traits.
#[derive(Clone)]
pub struct WindowControl {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowControl {}
unsafe impl Sync for WindowControl {}

impl Parent for WindowControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Child for WindowControl {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl WindowControl {
	/// Instantiates a new `Button` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowControlOpts) -> WindowControl {
		Self {
			raw_dlg: RawDlg::Raw(
				RawControl::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowControl` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// Position will be adjusted to match current system DPI.
	pub fn new_dlg(
		parent: &dyn Parent,
		dialog_id: i32,
		position: POINT,
		ctrl_id: Option<i32>) -> WindowControl
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgControl::new(
					baseref_from_parent(parent), dialog_id, position, ctrl_id,
				),
			),
		}
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.base_ref(),
			RawDlg::Dlg(d) => d.base_ref(),
		}
	}

	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is created.
	pub fn hwnd(&self) -> HWND {
		*self.base_ref().hwnd_ref()
	}

	/// Exposes the window events.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	pub fn on(&self) -> &WindowEvents {
		self.base_ref().user_events_ref()
	}

	/// If you perform a very long task in the UI thread, the UI freezes until
	/// the task is complete – this may cause the impression that your
	/// application crashed. That's why long tasks are performed in parallel
	/// threads. However, at some point you'll want to update the UI to reflect
	/// the task progress, but if you update the UI from another thread
	/// (different from the original UI thread), the UI may deadlock, and you
	/// application crashes.
	///
	/// The `run_ui_thread` method allows UI updates by running a closure
	/// synchronously in the window's original UI thread.
	///
	/// This is what this `run_ui_thread` does, step-by-step:
	///
	/// 1. blocks current thread;
	/// 2. switches to the window's original UI thread;
	/// 3. runs the given `FnOnce`;
	/// 4. switches back to the first thread, which is then unblocked.
	///
	/// When working in a parallel thread, you **must** call `run_ui_thread` to
	/// update the UI.
	///
	/// # Examples
	///
	/// The example below shows the event of a
	/// [button click](crate::gui::events::ButtonEvents::bn_clicked) which
	/// starts a long task in a parallel thread. As it progresses, the status is
	/// printed at the windows's titlebar.
	///
	/// ```rust,ignore
	/// use winsafe::{Button, GetCurrentThreadId, Sleep, WindowMain};
	///
	/// let wnd: WindowMain; // initialized somewhere
	/// let btn: Button;
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || {
	///         println!("Click event at {:#x}", GetCurrentThreadId());
	///
	///         std::thread::spawn({
	///             let wnd = wnd.clone();
	///             move || {
	///                 println!("Parallel task starts at {:#x}", GetCurrentThreadId());
	///                 Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || {
	///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 50%").unwrap();
	///                     }
	///                 });
	///
	///                 println!("Parallel task keeps going at {:#x}", GetCurrentThreadId());
	///                 Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || {
	///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 100%").unwrap();
	///                     }
	///                 });
	///             }
	///         });
	///     }
	/// });
	/// ```
	pub fn run_ui_thread<F: FnOnce()>(&self, func: F) {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.run_ui_thread(func),
			RawDlg::Dlg(d) => d.run_ui_thread(func),
		}
	}
}
