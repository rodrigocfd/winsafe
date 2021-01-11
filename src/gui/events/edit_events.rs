use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes edit
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
pub struct EditEvents {
	parent_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl EditEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> EditEvents {
		Self {
			parent_events: NonNull::from(parent.events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_events(&self) -> &MsgEvents {
		unsafe { self.parent_events.as_ref() }
	}

	cmd_event! { en_align_ltr_ec, co::CMD::EN_ALIGN_LTR_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-ltr-ec)
		/// command notification.
	}

	cmd_event! { en_align_rtl_ec, co::CMD::EN_ALIGN_RTL_EC,
		/// [`EN_ALIGN_LTR_EC`](https://docs.microsoft.com/en-us/windows/win32/controls/en-align-rtl-ec)
		/// command notification.
	}

	cmd_event! { en_change, co::CMD::EN_CHANGE,
		/// [`EN_CHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-change)
		/// command notification.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::gui::Edit;
		///
		/// let txt: Edit; // initialize it somewhere...
		///
		/// txt.on().en_change({
		///   let txt = txt.clone(); // pass into the closure
		///   move || {
		///     println!("Text: {}",
		///       btn.hwnd().GetWindowTextStr().unwrap());
		///   }
		/// });
		/// ```
	}

	cmd_event! { en_err_space, co::CMD::EN_ERRSPACE,
		/// [`EN_ERRSPACE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-errspace)
		/// command notification.
	}

	cmd_event! { en_h_scoll, co::CMD::EN_HSCROLL,
		/// [`EN_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-hscroll)
		/// command notification.
	}

	cmd_event! { en_kill_focus, co::CMD::EN_KILLFOCUS,
		/// [`EN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
	}

	cmd_event! { en_max_text, co::CMD::EN_MAXTEXT,
		/// [`EN_MAXTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
	}

	cmd_event! { en_set_focus, co::CMD::EN_SETFOCUS,
		/// [`EN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-setfocus)
		/// command notification.
	}

	cmd_event! { en_update, co::CMD::EN_UPDATE,
		/// [`EN_UPDATE`](https://docs.microsoft.com/en-us/windows/win32/controls/en-update)
		/// command notification.
	}

	cmd_event! { en_v_scoll, co::CMD::EN_VSCROLL,
		/// [`EN_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/en-vscroll)
		/// command notification.
	}
}
