use crate::co::ERROR;
use crate::gui::events::MsgEvents;
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	fn hwnd_ref(&self) -> &HWND;
	fn events_ref(&self) -> &MsgEvents;
}

/// Trait to any child control.
pub trait Child {
	fn create(&self) -> Result<(), ERROR>;
}

/// Physically creates the controls within the parent window:
///
/// * if parent is an ordinary window, calls
/// [`CreateWindowEx`](crate::HWND::CreateWindowEx);
/// * if a dialog resource, calls [`GetDlgItem`](crate::HWND::GetDlgItem).
///
/// This function should be called within parent's `WM_CREATE` or
/// `WM_INITDIALOG` events.
///
/// Note that tab order follows creation order.
///
/// # Panics
///
/// Panics if the control is already created, or if the parent window was not
/// created yet.
pub fn create_children(ctrls: &[&dyn Child]) -> Result<(), ERROR> {
	for ctrl in ctrls.iter() {
		ctrl.create()?;
	}
	Ok(())
}
