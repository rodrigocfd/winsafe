use crate::aliases::ErrResult;
use crate::co;

pub_struct_ctrl_events_proxy! {
	/// Exposes edit control
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
	///
	/// These event methods are just proxies to the
	/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window,
	/// who is the real responsible for the child event handling.
	///
	/// You cannot directly instantiate this object, it is created internally by
	/// the control.
	EditEvents
}

impl EditEvents {
	pub_fn_cmd_ret0! { en_align_ltr_ec, co::EN::ALIGN_LTR_EC.into(),
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
		///
		/// Sent when the user has changed the edit control direction to
		/// left-to-right.
	}

	pub_fn_cmd_ret0! { en_align_rtl_ec, co::EN::ALIGN_RTL_EC.into(),
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
		///
		/// Sent when the user has changed the edit control direction to
		/// right-to-left.
	}

	pub_fn_cmd_ret0! { en_change, co::EN::CHANGE.into(),
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
		/// ```rust,ignore
		/// use winsafe::{gui, ErrResult};
		///
		/// let txt: gui::Edit; // initialized somewhere
		///
		/// txt.on().en_change({
		///     let txt = txt.clone(); // pass into the closure
		///     move || -> ErrResult<()> {
		///         println!("Text: {}", btn.hwnd().GetWindowText()?);
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_cmd_ret0! { en_err_space, co::EN::ERRSPACE.into(),
		/// [`EN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
		///
		/// Sent when an edit control cannot allocate enough memory to meet a
		/// specific request.
	}

	pub_fn_cmd_ret0! { en_h_scroll, co::EN::HSCROLL.into(),
		/// [`EN_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
		///
		/// Sent when the user clicks an edit control's horizontal scroll bar.
		/// Notified before the screen is updated.
	}

	pub_fn_cmd_ret0! { en_kill_focus, co::EN::KILLFOCUS.into(),
		/// [`EN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
		///
		/// Sent when an edit control loses the keyboard focus.
	}

	pub_fn_cmd_ret0! { en_max_text, co::EN::MAXTEXT.into(),
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

	pub_fn_cmd_ret0! { en_set_focus, co::EN::SETFOCUS.into(),
		/// [`EN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
		///
		/// Sent when an edit control receives the keyboard focus.
	}

	pub_fn_cmd_ret0! { en_update, co::EN::UPDATE.into(),
		/// [`EN_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
		///
		/// Sent when an edit control is about to redraw itself. This
		/// notification code is sent after the control has formatted the text,
		/// but before it displays the text. This makes it possible to resize
		/// the edit control window, if necessary.
	}

	pub_fn_cmd_ret0! { en_v_scroll, co::EN::VSCROLL.into(),
		/// [`EN_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
		///
		/// Sent when the user clicks an edit control's vertical scroll bar or
		/// when the user scrolls the mouse wheel over the edit control.
		/// Notified before the screen is updated.
	}
}
