use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::structs::{NMBCDROPDOWN, NMBCHOTITEM, NMCUSTOMDRAW};

struct Obj {
	parent_events: MsgEvents,
	ctrl_id: u16,
}

//------------------------------------------------------------------------------

/// Allows adding closures to handle button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
#[derive(Clone)]
pub struct ButtonEvents {
	obj: Rc<UnsafeCell<Obj>>,
}

cref_mref!(ButtonEvents);

impl ButtonEvents {
	pub(crate) fn new(parent_events: MsgEvents, ctrl_id: u16) -> ButtonEvents {
		Self {
			obj: Rc::new(UnsafeCell::new(
				Obj {
					parent_events,
					ctrl_id,
				},
			)),
		}
	}

	/// Adds a handler to
	/// [`BCN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-dropdown)
	/// notification.
	pub fn bcn_drop_down<F>(&self, func: F)
		where F: FnMut(&NMBCDROPDOWN) + Send + Sync + 'static,
	{
		self.mref().parent_events.add_nfy(self.cref().ctrl_id, co::NM::BCN_DROPDOWN, {
			let mut func = func;
			move |p| { func(unsafe { p.cast_nmhdr::<NMBCDROPDOWN>() }); None }
		});
	}

	/// Adds a handler to
	/// [`BCN_HOTITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-hotitemchange)
	/// notification.
	pub fn bcn_hot_item_change<F>(&self, func: F)
		where F: FnMut(&NMBCHOTITEM) + Send + Sync + 'static,
	{
		self.mref().parent_events.add_nfy(self.cref().ctrl_id, co::NM::BCN_HOTITEMCHANGE, {
			let mut func = func;
			move |p| { func(unsafe { p.cast_nmhdr::<NMBCHOTITEM>() }); None }
		});
	}

	/// Adds a handler to
	/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification.
	pub fn bn_clicked<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.mref().parent_events.wm_command(co::CMD::BN_CLICKED, self.cref().ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.mref().parent_events.wm_command(co::CMD::BN_DBLCLK, self.cref().ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.mref().parent_events.wm_command(co::CMD::BN_KILLFOCUS, self.cref().ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.mref().parent_events.wm_command(co::CMD::BN_SETFOCUS, self.cref().ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-button)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: FnMut(&NMCUSTOMDRAW) -> co::CDRF + Send + Sync + 'static,
	{
		self.mref().parent_events.add_nfy(self.cref().ctrl_id, co::NM::CUSTOMDRAW, {
			let mut func = func;
			move |p| Some(u32::from(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })) as isize)
		});
	}
}
