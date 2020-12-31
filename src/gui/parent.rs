use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Internal trait to any window which can host child controls.
pub trait Parent {
	fn hwnd_ref(&self) -> &HWND;
	fn events_ref(&self) -> &MsgEvents;
}
