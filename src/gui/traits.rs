use std::any::Any;

use crate::aliases::{ErrResult, WinResult};
use crate::co;
use crate::gui::events::{WindowEvents, WindowEventsAll};
use crate::gui::traits_sealed::{SealedBase, SealedParent};
use crate::handles::HWND;

/// Used to convert a reference to the
/// [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait.
pub trait AsAny {
	/// Converts a reference to the
	/// [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait.
	fn as_any(&self) -> &dyn Any;
}

/// Any window. Exposes the underlying window handle.
pub trait Window: AsAny {
	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is physically created, what usually happens right
	/// before [`WM_CREATE`](crate::gui::prelude::EventsView::wm_create) or
	/// [`WM_INITDIALOG`](crate::gui::prelude::EventsView::wm_init_dialog)
	/// events.
	fn hwnd(&self) -> HWND;
}

/// Any window which can host child controls.
///
/// This is a sealed trait, which cannot be implemented outside `winsafe`.
pub trait Parent: Send + Window + SealedBase + SealedParent {
	/// Exposes the window events and control notifications.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	fn on(&self) -> &WindowEventsAll;
}

/// The main window of an application.
///
/// This is a sealed trait, which cannot be implemented outside `winsafe`.
pub trait Main: Parent {
	/// Physically creates the window, then runs the main application loop. This
	/// method will block until the window is closed.
	///
	/// The `cmd_show` parameter defaults to
	/// [`co::SW::SHOW`](crate::co::SW::SHOW).
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	fn run_main(&self, cmd_show: Option<co::SW>) -> ErrResult<i32>;
}

/// A modal window.
///
/// This is a sealed trait, which cannot be implemented outside `winsafe`.
pub trait Modal: Parent {
	/// Physically creates the window, then runs the modal loop. This method
	/// will block until the window is closed.
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	fn show_modal(&self) -> WinResult<i32>;
}

/// Any child window.
pub trait Child: Window {
	/// Returns the control ID.
	fn ctrl_id(&self) -> u16;
}

/// Any native control, which can be subclassed.
pub trait NativeControl: Child {
	/// Exposes the subclass events. If at least one event exists, the control
	/// will be
	/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
	///
	/// **Note:** Subclassing may impact performance, use with care.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	fn on_subclass(&self) -> &WindowEvents;
}

/// Events of a native control.
pub trait NativeControlEvents<E> {
	/// Exposes the specific control events.
	///
	/// # Panics
	///
	/// Panics if the control is already created. Events must be set before
	/// control creation.
	fn on(&self) -> &E;
}

/// Allows running code in the original UI thread.
pub trait UiThread: Window {
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
	/// use winsafe::prelude::*;
	/// use winsafe::{gui, ErrResult, GetCurrentThreadId, Sleep};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// let btn: gui::Button;
	///
	/// btn.on().bn_clicked({
	///     let wnd = wnd.clone();
	///     move || -> ErrResult<()> {
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
	///                     move || -> ErrResult<()> {
	///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
	///                         wnd.hwnd().SetWindowText("Status... 50%")?;
	///                         Ok(())
	///                     }
	///                 });
	///
	///                 println!("Parallel task keeps going at {:#x}", GetCurrentThreadId());
	///                 Sleep(2000);
	///
	///                 wnd.run_ui_thread({
	///                     let wnd = wnd.clone();
	///                     move || -> ErrResult<()> {
	///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
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
		where F: FnOnce() -> ErrResult<()>;
}
