use std::ops::Deref;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Exposes window
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
/// including the creation messages:
/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// and
/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog).
///
/// You cannot directly instantiate this object, it is created internally by the
/// window.
pub struct WindowEventsAll {
	window_events: WindowEvents,
}

impl Deref for WindowEventsAll {
	type Target = WindowEvents;

	fn deref(&self) -> &Self::Target {
		&self.window_events
	}
}

impl WindowEventsAll {
	#[must_use]
	pub(in crate::gui) const fn new(wnd_ty: WndTy) -> Self {
		Self { window_events: WindowEvents::new(wnd_ty) }
	}

	/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
	/// message, sent only to non-dialog windows. Dialog windows must handle
	/// [`wm_init_dialog`](crate::gui::events::WindowEvents::wm_init_dialog)
	/// instead.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui, msg};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// wnd.on().wm_create(
	///     move |p: msg::wm::Create| -> w::AnyResult<i32> {
	///         println!("Client area: {}x{}",
	///             p.createstruct.cx,
	///             p.createstruct.cy,
	///         );
	///         Ok(0)
	///     },
	/// );
	/// ```
	pub fn wm_create<F>(&self, func: F) -> &Self
	where
		F: Fn(wm::Create) -> AnyResult<i32> + 'static,
	{
		self.wm(co::WM::CREATE, move |p| Ok(func(unsafe { wm::Create::from_generic_wm(p) })? as _));
		self
	}

	pub_fn_wm_withparm_boolret! { wm_init_dialog, co::WM::INITDIALOG, wm::InitDialog;
		/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
		/// message, sent only to dialog windows. Non-dialog windows must handle
		/// [`wm_create`](crate::gui::events::WindowEvents::wm_create) instead.
		///
		/// Return `true` to set the focus to the first control in the dialog.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui, msg};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		///
		/// wnd.on().wm_init_dialog(
		///     move |p: msg::wm::InitDialog| -> w::AnyResult<bool> {
		///         println!("Focused HWND: {}", p.hwnd_focus);
		///         Ok(true)
		///     },
		/// );
		/// ```
	}
}
