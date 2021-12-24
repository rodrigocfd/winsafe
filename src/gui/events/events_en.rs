use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;

/// Exposes edit control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct EditEvents(BaseEventsProxy);

impl EditEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_cmd_ret0! { en_align_ltr_ec, co::EN::ALIGN_LTR_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
		///
		/// Sent when the user has changed the edit control direction to
		/// left-to-right.
	}

	pub_fn_cmd_ret0! { en_align_rtl_ec, co::EN::ALIGN_RTL_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
		///
		/// Sent when the user has changed the edit control direction to
		/// right-to-left.
	}

	pub_fn_cmd_ret0! { en_change, co::EN::CHANGE,
		/// [`EN_CHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-change)
		/// command notification.
		///
		/// Sent when the user has taken an action that may have altered text in
		/// an edit control. Unlike the
		/// [`EN_UPDATE`](crate::gui::events::EditEvents::en_update)
		/// notification code, this notification code is sent after the system
		/// updates the screen.
		///
		/// # Examples
		///
		/// ```rust,no_run
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, ErrResult};
		///
		/// let txt: gui::Edit; // initialized somewhere
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let txt = gui::Edit::new(&wnd, gui::EditOpts::default());
		///
		/// txt.on().en_change({
		///     let txt = txt.clone(); // pass into the closure
		///     move || -> ErrResult<()> {
		///         println!("Text: {}", txt.hwnd().GetWindowText()?);
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_cmd_ret0! { en_err_space, co::EN::ERRSPACE,
		/// [`EN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
		///
		/// Sent when an edit control cannot allocate enough memory to meet a
		/// specific request.
	}

	pub_fn_cmd_ret0! { en_h_scroll, co::EN::HSCROLL,
		/// [`EN_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
		///
		/// Sent when the user clicks an edit control's horizontal scroll bar.
		/// Notified before the screen is updated.
	}

	pub_fn_cmd_ret0! { en_kill_focus, co::EN::KILLFOCUS,
		/// [`EN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
		///
		/// Sent when an edit control loses the keyboard focus.
	}

	pub_fn_cmd_ret0! { en_max_text, co::EN::MAXTEXT,
		/// [`EN_MAXTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
		///
		/// Sent when the current text insertion has exceeded the specified
		/// number of characters for the edit control. The text insertion has
		/// been truncated.
		///
		/// This notification code is also sent when an edit control does not
		/// have the [`ES_AUTOHSCROLL`](crate::co::ES::AUTOHSCROLL) style and
		/// the number of characters to be inserted would exceed the width of
		/// the edit control.
		///
		/// This notification code is also sent when an edit control does not
		/// have the [`ES_AUTOVSCROLL`](crate::co::ES::AUTOVSCROLL) style and
		/// the total number of lines resulting from a text insertion would
		/// exceed the height of the edit control.
	}

	pub_fn_cmd_ret0! { en_set_focus, co::EN::SETFOCUS,
		/// [`EN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
		///
		/// Sent when an edit control receives the keyboard focus.
	}

	pub_fn_cmd_ret0! { en_update, co::EN::UPDATE,
		/// [`EN_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
		///
		/// Sent when an edit control is about to redraw itself. This
		/// notification code is sent after the control has formatted the text,
		/// but before it displays the text. This makes it possible to resize
		/// the edit control window, if necessary.
	}

	pub_fn_cmd_ret0! { en_v_scroll, co::EN::VSCROLL,
		/// [`EN_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
		///
		/// Sent when the user clicks an edit control's vertical scroll bar or
		/// when the user scrolls the mouse wheel over the edit control.
		/// Notified before the screen is updated.
	}
}
