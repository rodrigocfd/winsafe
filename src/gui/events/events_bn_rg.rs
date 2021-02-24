use std::ptr::NonNull;
use std::rc::Rc;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::immut::Immut;
use crate::gui::traits::Parent;

/// Exposes button control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
/// for a [`RadioGroup`](crate::gui::RadioGroup).
///
/// These event methods are just proxies to the
/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
/// the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct RadioGroupEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_ids: Vec<u16>,
}

impl RadioGroupEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_ids: Vec<u16>) -> RadioGroupEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_ids,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user clicks a button.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::{RadioGroup, WindowMain};
	///
	/// let wnd: WindowMain; // initialize them somewhere...
	/// let radios: RadioGroup;
	///
	/// radios.on().bn_clicked({
	///     let radios = radios.clone();
	///     move || {
	///         println!("Selected {}",
	///             rads.checked().unwrap().hwnd().GetWindowTextStr().unwrap());
	///     }
	/// });
	/// ```
	pub fn bn_clicked<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::CLICKED.into(), *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user double-clicks a button. This notification code is
	/// sent automatically for [`BS_USERBUTTON`](crate::co::BS::USERBUTTON),
	/// [`BS_RADIOBUTTON`](crate::co::BS::RADIOBUTTON), and
	/// [`BS_OWNERDRAW`](crate::co::BS::OWNERDRAW) buttons. Other button types
	/// send only if they have the [`BS_NOTIFY`](crate::co::BS::NOTIFY) style.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::DBLCLK.into(), *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button loses the keyboard focus. The button must have the
	/// [`BS_NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
	/// code.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::KILLFOCUS.into(), *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button receives the keyboard focus. The button must have
	/// the [`BS_NOTIFY`](crate::co::BS::NOTIFY) style to send this
	/// notification code.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_user_events().wm_command(co::BN::SETFOCUS.into(), *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}
}
