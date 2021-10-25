use std::any::Any;
use std::sync::Arc;

use crate::aliases::{ErrResult, WinResult};
use crate::enums::HwndFocus;
use crate::gui::{WindowControl, WindowMain, WindowModal};
use crate::gui::base::Base;
use crate::gui::events::{WindowEvents, WindowEventsAll};
use crate::handles::HWND;
use crate::msg::wm;

/// Trait to any window which can host child controls.
///
/// **Note:** This is a
/// [sealed trait](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed)
/// which cannot be implemented outside the library.
pub trait Parent: sealed_parent::SealedParent {
	/// Returns a reference to the `Any` trait, allowing downcasting.
	fn as_any(&self) -> &dyn Any;
}

pub(in crate::gui) mod sealed_parent {
	// Parent trait is a tunnel to baseref_from_parent(), which is specifically
	// implemented to the 3 Window structs. If the user ever implements Parent,
	// baseref_from_parent() would crash – that's why Parent trait is sealed.
	pub trait SealedParent {}
	impl SealedParent for super::WindowControl {}
	impl SealedParent for super::WindowMain {}
	impl SealedParent for super::WindowModal {}
}

pub(in crate::gui) fn baseref_from_parent(parent: &impl Parent) -> &Base {
	if let Some(w) = parent.as_any().downcast_ref::<WindowMain>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowModal>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowControl>() {
		w.base_ref()
	} else {
		panic!("Unknown Parent downcasting, something really bad happened.")
	}
}

/// Any window. Exposes the underlying window handle.
pub trait Window {
	/// Returns the underlying handle for this control.
	///
	/// Note that the handle is initially null, receiving an actual value only
	/// after the control is physically created, what usually happens right
	/// before [`WM_CREATE`](crate::gui::events::prelude::EventsView::wm_create)
	/// or
	/// [`WM_INITDIALOG`](crate::gui::events::prelude::EventsView::wm_init_dialog)
	/// events.
	fn hwnd(&self) -> HWND;
}

/// Allows running code in the original UI thread.
pub trait UiThread {
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

/// Exposes all parent window events and control notifications through
/// [`WindowEventsAll`](crate::gui::events::WindowEventsAll).
pub trait ParentEvents {
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
	/// Returns the control ID.
	fn ctrl_id(&self) -> u16;
}

/// Any native child control.
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

/// Retrieves a concrete native control as a trait.
pub trait AsNativeControl: NativeControl {
	fn as_native_control(&self) -> Arc<dyn NativeControl>;
}

/// Exposes the native control events.
pub trait NativeControlEvents<E>: NativeControl {
	/// Exposes the control events.
	///
	/// These event methods are just proxies to the
	/// [`WindowEventsAll`](crate::gui::events::WindowEventsAll) of the parent
	/// window, who is the real responsible for the child event handling.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	fn on(&self) -> &E;
}

/// Focus a child control with [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl)
/// message.
pub trait Focus: Child {
	/// Focuses the control by sending a
	/// [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl) message.
	fn focus(&self) -> WinResult<()> {
		self.hwnd().GetParent()
			.map(|hparent| {
				hparent.SendMessage(wm::NextDlgCtl {
					hwnd_focus: HwndFocus::Hwnd(self.hwnd()),
				})
			})
	}
}
