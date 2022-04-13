use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

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

	pub_fn_cmd_noparm_noret! { en_align_ltr_ec, co::EN::ALIGN_LTR_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_align_rtl_ec, co::EN::ALIGN_RTL_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_change, co::EN::CHANGE,
		/// [`EN_CHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-change)
		/// command notification.
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
		///     let txt = txt.clone(); // to pass into the closure
		///     move || -> ErrResult<()> {
		///         println!("Text: {}", txt.hwnd().GetWindowText()?);
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_cmd_noparm_noret! { en_err_space, co::EN::ERRSPACE,
		/// [`EN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_h_scroll, co::EN::HSCROLL,
		/// [`EN_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_kill_focus, co::EN::KILLFOCUS,
		/// [`EN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_max_text, co::EN::MAXTEXT,
		/// [`EN_MAXTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_set_focus, co::EN::SETFOCUS,
		/// [`EN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_update, co::EN::UPDATE,
		/// [`EN_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
	}

	pub_fn_cmd_noparm_noret! { en_v_scroll, co::EN::VSCROLL,
		/// [`EN_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
	}
}
