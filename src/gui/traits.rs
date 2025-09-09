use std::any::Any;

use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;

/// Any window. Exposes the underlying window handle.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiWindow: Send {
	/// Returns the underlying handle for this window.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the window is physically created, what usually happens right
	/// before
	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// or
	/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
	/// events.
	#[must_use]
	fn hwnd(&self) -> &HWND;

	/// Converts a reference to the [`Any`] trait. This is useful when storing a
	/// collection of polymorphic controls, because `Any` allows downcasting.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::sync::Arc;
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let parent: gui::WindowMain; // initialized somewhere
	/// # let parent = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let ctrls: Vec<Arc<dyn GuiControl>> = vec![
	///     Arc::new( gui::Edit::new(&parent, gui::EditOpts::default()) ),
	///     Arc::new( gui::Button::new(&parent, gui::ButtonOpts::default()) ),
	/// ];
	///
	/// let edit = ctrls[0].as_any() // retrieve 1st element, which is an Edit
	///     .downcast_ref::<gui::Edit>()
	///     .expect("This Edit downcast should never fail.");
	///
	/// edit.set_text("Foo");
	/// ```
	#[must_use]
	fn as_any(&self) -> &dyn Any;
}

/// Any window which can host child controls.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
#[allow(private_bounds)]
pub trait GuiParent: GuiWindow + Clone + AsRef<BaseWnd> {
	/// This method calls [`std::thread::spawn`], but it allows the returning of
	/// an error value. This error value will be forwarded to the original UI
	/// thread, allowing it to be caught at
	/// [`WindowMain::run_main`](crate::gui::WindowMain::run_main).
	///
	/// It's a way to ensure that, upon an unexpected error, you application
	/// will be terminated gracefully.
	///
	/// # Examples
	///
	/// The example below shows the event of a
	/// [button click](crate::gui::events::ButtonEvents::bn_clicked) which
	/// spawns a new thread.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// let btn: gui::Button;
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let btn = gui::Button::new(&wnd, gui::ButtonOpts::default());
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Click event at {:#x}", w::GetCurrentThreadId());
	///
	///         wnd.spawn_thread({
	///             let wnd = wnd.clone();
	///             move || {
	///                 println!("This is another thread: {:#x}", w::GetCurrentThreadId());
	///                 if 1 != 2 {
	///                     Err("Unexpected condition, goodbye.".into())
	///                 } else {
	///                     Ok(())
	///                 }
	///             }
	///         });
	///
	///         Ok(())
	///     }
	/// });
	/// ```
	fn spawn_thread<F>(&self, func: F)
	where
		F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.as_ref().spawn_thread(func)
	}

	/// Runs a closure synchronously in the window's original UI thread,
	/// allowing UI updates without the risk of a deadlock.
	///
	/// # Rationale
	///
	/// If you perform a very long task in the UI thread, the UI freezes until
	/// the task is complete – this may cause the impression that your
	/// application crashed. That's why long tasks should be performed in
	/// parallel threads. However, at some point you'll want to update the UI to
	/// reflect the task progress, but if you update the UI from another thread
	/// (different from the original UI thread), the UI may deadlock, and you
	/// application crashes.
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// let btn: gui::Button;
	/// # let btn = gui::Button::new(&wnd, gui::ButtonOpts::default());
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Click event at {:#x}", w::GetCurrentThreadId());
	///
	///         std::thread::spawn({
	///             let wnd = wnd.clone();
	///             move || {
	///                 println!("Parallel task starts at {:#x}", w::GetCurrentThreadId());
	///                 w::Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || -> w::AnyResult<()> {
	///                         println!("Updating UI at {:#x}", w::GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 50%")?;
	///                         Ok(())
	///                     }
	///                 });
	///
	///                 println!("Parallel task keeps going at {:#x}", w::GetCurrentThreadId());
	///                 w::Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || -> w::AnyResult<()> {
	///                         println!("Updating UI at {:#x}", w::GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 100%")?;
	///                         Ok(())
	///                     }
	///                 });
	///             }
	///         });
	///
	///         Ok(())
	///     }
	/// });
	/// ```
	fn run_ui_thread<F>(&self, func: F)
	where
		F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		self.as_ref().run_ui_thread(func)
	}
}

/// Any child control.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiControl: GuiWindow {
	/// Returns the control ID, which is defined at control creation.
	///
	/// The control ID should be unique within a parent.
	#[must_use]
	fn ctrl_id(&self) -> u16;

	/// If the control belongs to an ordinary window, simply calls
	/// [`HWND:SetFocus`](crate::HWND::SetFocus).
	///
	/// If the control belongs to a dialog window, sends a
	/// [`wm::NextDlgCtl`] message. This is preferable to the `HWND::SetFocus`
	/// because it takes care of border highlighting, like the native
	/// [`Button`](crate::gui::Button) control needs.
	fn focus(&self) -> SysResult<()> {
		let hparent = self.hwnd().GetParent()?;
		if hparent.is_dialog() {
			unsafe {
				hparent.SendMessage(wm::NextDlgCtl {
					hwnd_focus: HwndFocus::Hwnd(self.hwnd().raw_copy()),
				});
			}
		} else {
			self.hwnd().SetFocus();
		}
		Ok(())
	}
}
