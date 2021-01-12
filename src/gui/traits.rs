use crate::co::ERROR;
use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the window handle.
	fn hwnd_ref(&self) -> &HWND;
	/// Returns a reference to the window events object.
	fn events_ref(&self) -> &MsgEvents;
}

/// Trait to any child control.
pub trait Child {
	/// Physically creates the control, either with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx) or
	/// [`GetDlgItem`](crate::HWND::GetDlgItem) (in dialog resources).
	fn create(&self) -> Result<(), ERROR>;
}

/// Physically creates the controls within the parent window:
///
/// * if parent is an ordinary window, calls
/// [`CreateWindowEx`](crate::HWND::CreateWindowEx);
/// * if a dialog resource, calls [`GetDlgItem`](crate::HWND::GetDlgItem).
///
/// This function should be called within parent's
/// [`WM_CREATE`](crate::msg::WmCreate) or
/// [`WM_INITDIALOG`](crate::msg::WmInitDialog) events.
///
/// Note that tab order follows creation order.
///
/// # Panics
///
/// Panics if one of the controls is already created, or if the parent window
/// was not created yet.
pub fn create_children(ctrls: &[&dyn Child]) -> Result<(), ERROR> {
	for ctrl in ctrls.iter() {
		ctrl.create()?;
	}
	Ok(())
}
