use std::cell::UnsafeCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::{NMBCHOTITEM, NMCUSTOMDRAW, co};
use crate::gui::events::Events;
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::parent::Parent;
use crate::handles::HWND;
use crate::structs::NMBCDROPDOWN;

struct Obj {
	base: NativeControlBase,
	parent_events: EventsButton,
	subclass_events: Events,
}

//------------------------------------------------------------------------------

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	obj: Arc<UnsafeCell<Obj>>,
}

unsafe impl Send for Button {}
unsafe impl Sync for Button {}

impl Button {
	/// Creates a new Button object.
	pub fn new<T: Parent>(parent: T) -> Button {
		Self::new_with_id(parent, NativeControlBase::auto_ctrl_id())
	}

	/// Creates a new Button object with a specific control ID.
	pub fn new_with_id<T: Parent>(parent: T, ctrl_id: u16) -> Button {
		Self {
			obj: Arc::new(UnsafeCell::new(
				Obj {
					base: NativeControlBase::new_with_id(ctrl_id),
					parent_events: EventsButton::new(parent.on(), ctrl_id),
					subclass_events: Events::new(),
				}
			)),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		let self2 = unsafe { &*self.obj.get() };
		self2.base.ctrl_id()
	}

	pub fn on(&self) -> EventsButton {
		let self2 = unsafe { &*self.obj.get() };
		self2.parent_events.clone()
	}

	pub fn on_subclass(&self) -> Events {
		let self2 = unsafe { &*self.obj.get() };
		self2.subclass_events.clone()
	}
}

//------------------------------------------------------------------------------

struct EventsObj {
	parent_events: Events,
	ctrl_id: u16,
}

/// Allows adding closures to handle button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
#[derive(Clone)]
pub struct EventsButton {
	obj: Rc<UnsafeCell<EventsObj>>,
}

impl EventsButton {
	pub(crate) fn new(parent_events: Events, ctrl_id: u16) -> EventsButton {
		Self {
			obj: Rc::new(UnsafeCell::new(
				EventsObj {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.add_nfy(self2.ctrl_id, co::NM::BCN_DROPDOWN, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.add_nfy(self2.ctrl_id, co::NM::BCN_HOTITEMCHANGE, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.wm_command(co::CMD::BN_CLICKED, self2.ctrl_id, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.wm_command(co::CMD::BN_DBLCLK, self2.ctrl_id, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.wm_command(co::CMD::BN_KILLFOCUS, self2.ctrl_id, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.wm_command(co::CMD::BN_SETFOCUS, self2.ctrl_id, {
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
		let self2 = unsafe { &mut *self.obj.get() };
		self2.parent_events.add_nfy(self2.ctrl_id, co::NM::CUSTOMDRAW, {
			let mut func = func;
			move |p| Some(u32::from(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })) as isize)
		});
	}
}