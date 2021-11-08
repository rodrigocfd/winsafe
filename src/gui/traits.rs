use std::any::Any;

use crate::aliases::{ErrResult, WinResult};
use crate::enums::HwndFocus;
use crate::gui::events::{WindowEvents, WindowEventsAll};
use crate::gui::traits_sealed::{SealedBase, SealedParent};
use crate::handles::HWND;
use crate::msg::wm;

/// Used to convert a reference to the
/// [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait.
pub trait AsAny {
	/// Converts a reference to the
	/// [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait. This is
	/// useful when storing a collection of polymorphic controls, because `Any`
	/// allows downcasting.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use std::sync::Arc;
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let parent: gui::WindowMain; // initialized somewhere
	///
	/// let ctrls: Vec<Arc<dyn gui::NativeControl>> = vec![
	///     Arc::new( gui::Edit::new(&parent, gui::EditOpts::default()) ),
	///     Arc::new( gui::Button::new(&parent, gui::ButtonOpts::default()) ),
	/// ];
	///
	/// let edit = ctrls[0].as_any().downcast_ref::<gui::Edit>()?;
	/// edit.set_text("Foo")?;
	/// ```
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

/// Any child window.
pub trait Child: Window {
	/// Returns the control ID, which is defined at control creation.
	///
	/// The control ID should be unique within a parent.
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

/// Any child window which can be focused.
pub trait FocusControl: Child {
	/// Focus the control by sending a
	/// [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl) message. This is
	/// preferable to the [`HWND::SetFocus`](crate::HWND::SetFocus) method,
	/// because it takes care of border highlighting, like the native
	/// [`Button`](crate::gui::Button) control needs.
	fn focus(&self) -> WinResult<()> {
		self.hwnd().GetParent()
			.map(|hparent|
				hparent.SendMessage(wm::NextDlgCtl {
					hwnd_focus: HwndFocus::Hwnd(self.hwnd()),
				}),
			)
	}
}

/// Any child window which can get/set text.
pub trait TextControl: Child {
	/// Sets the text by calling
	/// [`HWND::SetWindowText`](crate::HWND::SetWindowText).
	fn set_text(&self, text: &str) -> WinResult<()> {
		self.hwnd().SetWindowText(text)
	}

	/// Retrieves the text by calling
	/// [`HWND::GetWindowText`](crate::HWND::GetWindowText).
	fn text(&self) -> WinResult<String> {
		self.hwnd().GetWindowText()
	}
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
