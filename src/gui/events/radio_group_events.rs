use std::ptr::NonNull;
use std::rc::Rc;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::immut::Immut;
use crate::gui::traits::Parent;

/// Exposes button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
/// for a [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioGroupEvents {
	parent_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_ids: Vec<u16>,
}

impl RadioGroupEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_ids: Vec<u16>) -> RadioGroupEvents {
		Self {
			parent_events: NonNull::from(parent.events_ref()), // convert reference to pointer
			ctrl_ids,
		}
	}

	fn parent_events(&self) -> &MsgEvents {
		unsafe { self.parent_events.as_ref() }
	}

	/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification for all radio buttons in the group.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::{RadioGroup, WindowMain};
	///
	/// let wnd = WindowMain; // initialize them somewhere...
	/// let radios = RadioGroup;
	///
	/// radios.on().bn_clicked({
	///   let radios = radios.clone();
	///   move || {
	///     println!("Selected {}",
	///       rads.checked().unwrap().hwnd().GetWindowTextStr().unwrap());
	///   }
	/// });
	/// ```
	pub fn bn_clicked<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_events().wm_command(co::CMD::BN_CLICKED, *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification for all radio buttons in the group.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_events().wm_command(co::CMD::BN_DBLCLK, *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification for all radio buttons in the group.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_events().wm_command(co::CMD::BN_KILLFOCUS, *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}

	/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification for all radio buttons in the group.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		let shared_func = Rc::new(Immut::new(func));

		for ctrl_id in self.ctrl_ids.iter() {
			self.parent_events().wm_command(co::CMD::BN_SETFOCUS, *ctrl_id, {
				let shared_func = shared_func.clone();
				move || shared_func.as_mut()()
			});
		}
	}
}
