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
}
