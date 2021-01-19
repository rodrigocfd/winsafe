use crate::aliases::WinResult;
use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the window handle.
	fn hwnd_ref(&self) -> &HWND;
	/// Returns a reference to the window events object.
	fn events_ref(&self) -> &MsgEvents;
	/// Receives a closure that will create a control, to be called either in
	/// [`WM_CREATE`](crate::msg::WmCreate) or
	/// [`WM_INITDIALOG`](crate::msg::WmInitDialog).
	fn add_child_to_be_created(&self, func: Box<dyn Fn() -> WinResult<()> + 'static>);
}

/// Trait to any child control.
pub trait Child {
	/// Returns a reference to the control handle.
	fn hctrl_ref(&self) -> &HWND;
}
