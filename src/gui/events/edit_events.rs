use crate::co;
use crate::gui::privs::*;

/// This trait is enabled with the `gui` feature, and exposes edit control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`GuiEventsParent`](crate::prelude::GuiEventsParent) of the parent window,
/// who is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsEdit: priv_ctrl_events::GuiEvents {
	fn_cmd_noparm_noret! { en_align_ltr_ec, co::EN::ALIGN_LTR_EC;
		/// [`EN_ALIGN_LTR_EC`](https://learn.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_align_rtl_ec, co::EN::ALIGN_RTL_EC;
		/// [`EN_ALIGN_RTL_EC`](https://learn.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_change, co::EN::CHANGE;
		/// [`EN_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-change)
		/// command notification.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, gui};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let txt: gui::Edit;
		/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
		/// # let txt = gui::Edit::new(&wnd, gui::EditOpts::default());
		///
		/// let txt2 = txt.clone(); // to pass into the closure
		///
		/// txt.on().en_change(
		///     move || -> w::AnyResult<()> {
		///         println!("Text: {}", txt2.text()?);
		///         Ok(())
		///     },
		/// );
		/// # w::SysResult::Ok(())
		/// ```
	}

	fn_cmd_noparm_noret! { en_err_space, co::EN::ERRSPACE;
		/// [`EN_ERRSPACE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_h_scroll, co::EN::HSCROLL;
		/// [`EN_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_kill_focus, co::EN::KILLFOCUS;
		/// [`EN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_max_text, co::EN::MAXTEXT;
		/// [`EN_MAXTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_set_focus, co::EN::SETFOCUS;
		/// [`EN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_update, co::EN::UPDATE;
		/// [`EN_UPDATE`](https://learn.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
	}

	fn_cmd_noparm_noret! { en_v_scroll, co::EN::VSCROLL;
		/// [`EN_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
	}
}

impl GuiEventsEdit for BaseCtrlEvents {}
