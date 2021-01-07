use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::structs::{NMBCDROPDOWN, NMBCHOTITEM, NMCUSTOMDRAW};

/// Exposes button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
pub struct ButtonEvents {
	parent_events: *const MsgEvents, // used only before parent creation
	ctrl_id: u16,
}

impl ButtonEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> ButtonEvents {
		Self {
			parent_events: parent.events_ref(), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_events(&self) -> &MsgEvents {
		unsafe { &*self.parent_events }
	}

	/// [`BCN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-dropdown)
	/// notification.
	pub fn bcn_drop_down<F>(&self, func: F)
		where F: FnMut(&NMBCDROPDOWN) + 'static,
	{
		self.parent_events().add_nfy(self.ctrl_id, co::NM::BCN_DROPDOWN, {
			let mut func = func;
			move |p| { func(unsafe { p.cast_nmhdr::<NMBCDROPDOWN>() }); None }
		});
	}

	/// [`BCN_HOTITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-hotitemchange)
	/// notification.
	pub fn bcn_hot_item_change<F>(&self, func: F)
		where F: FnMut(&NMBCHOTITEM) + 'static,
	{
		self.parent_events().add_nfy(self.ctrl_id, co::NM::BCN_HOTITEMCHANGE, {
			let mut func = func;
			move |p| { func(unsafe { p.cast_nmhdr::<NMBCHOTITEM>() }); None }
		});
	}

	/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::{Button, WindowMain};
	///
	/// let wnd = WindowMain; // initialize them somewhere...
	/// let btn = Button;
	///
	/// btn.on().bn_clicked({
	///   let btn = btn.clone();
	///   move || {
	///     println!("HWND: {}", btn.hwnd());
	///   }
	/// });
	/// ```
	pub fn bn_clicked<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		self.parent_events().wm_command(co::CMD::BN_CLICKED, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		self.parent_events().wm_command(co::CMD::BN_DBLCLK, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		self.parent_events().wm_command(co::CMD::BN_KILLFOCUS, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: FnMut() + 'static,
	{
		self.parent_events().wm_command(co::CMD::BN_SETFOCUS, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-button)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: FnMut(&NMCUSTOMDRAW) -> co::CDRF + 'static,
	{
		self.parent_events().add_nfy(self.ctrl_id, co::NM::CUSTOMDRAW, {
			let mut func = func;
			move |p| Some(u32::from(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })) as isize)
		});
	}
}
