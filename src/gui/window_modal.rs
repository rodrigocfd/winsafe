use std::any::Any;

use crate::aliases::WinResult;
use crate::gui::base::Base;
use crate::gui::dlg_modal::DlgModal;
use crate::gui::events::WindowEvents;
use crate::gui::raw_modal::{WindowModalOpts, RawModal};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;

#[derive(Clone)]
enum RawDlg { Raw(RawModal), Dlg(DlgModal) }

/// An user modal window, which can handle events. Can be programmatically
/// created or load a dialog resource from a `.res` file.
///
/// Implements [`Parent`](crate::gui::Parent) trait.
#[derive(Clone)]
pub struct WindowModal {
	raw_dlg: RawDlg,
}

unsafe impl Send for WindowModal {}
unsafe impl Sync for WindowModal {}

impl Parent for WindowModal {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl WindowModal {
	/// Instantiates a new `WindowModal` object, to be created with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: WindowModalOpts) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Raw(
				RawModal::new(baseref_from_parent(parent), opts),
			),
		}
	}

	/// Instantiates a new `WindowModal` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, dialog_id: i32) -> WindowModal {
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgModal::new(baseref_from_parent(parent), dialog_id),
			),
		}
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.base_ref(),
			RawDlg::Dlg(d) => d.base_ref(),
		}
	}

	/// Returns the underlying handle for this window.
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

	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn show_modal(&self) -> WinResult<i32> {
		match &self.raw_dlg {
			RawDlg::Raw(r) => r.show_modal(),
			RawDlg::Dlg(d) => d.show_modal(),
		}
	}
}
