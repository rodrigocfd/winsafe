use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;

/// Exposes edit
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications).
pub struct EditEvents {
	parent_events: *const MsgEvents, // used only before parent creation
	ctrl_id: u16,
}

impl EditEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> EditEvents {
		Self {
			parent_events: parent.events_ref(), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_events(&self) -> &MsgEvents {
		unsafe { &*self.parent_events }
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

	cmd_event! { en_kill_focus, co::CMD::EN_KILLFOCUS,
		/// [`EN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/en-killfocus)
		/// command notification.
	}

	cmd_event! { en_max_text, co::CMD::EN_MAXTEXT,
		/// [`EN_MAXTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/en-maxtext)
		/// command notification.
	}
}
