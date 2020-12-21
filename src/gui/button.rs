use crate::co;
use crate::gui::events::Events;
use crate::gui::native_control_base::NativeControlBase;
use crate::gui::Parent;
use crate::handles::HWND;

/// Native
/// [button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#push-buttons)
/// control.
#[derive(Clone)]
pub struct Button {
	base: NativeControlBase,
	parent_events: EventsButton,
	subclass_events: Events,
}

impl Button {
	/// Creates a new Button object.
	pub fn new(parent: &impl Parent) -> Button {
		Self::new_with_id(parent, NativeControlBase::auto_ctrl_id())
	}

	/// Creates a new Button object with a specific control ID.
	pub fn new_with_id(parent: &impl Parent, ctrl_id: u16) -> Button {
		Self {
			base: NativeControlBase::new_with_id(ctrl_id),
			parent_events: EventsButton::new(parent.on(), ctrl_id),
			subclass_events: Events::new(),
		}
	}

	/// Returns the underlying handle for this window.
	pub fn hwnd(&self) -> HWND {
		self.base.hwnd()
	}

	/// Returns the control ID.
	pub fn ctrl_id(&self) -> u16 {
		self.base.ctrl_id()
	}

	pub fn on(&self) -> EventsButton {
		self.parent_events.clone()
	}

	pub fn on_subclass(&self) -> Events {
		self.subclass_events.clone()
	}
}

//------------------------------------------------------------------------------

/// Allows adding closures to handle button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
#[derive(Clone)]
pub struct EventsButton {
	parent_events: Events,
	ctrl_id: u16,
}

impl EventsButton {
	pub(super) fn new(parent_events: Events, ctrl_id: u16) -> EventsButton {
		Self {
			parent_events,
			ctrl_id,
		}
	}

	/// Adds a handler to
	/// [BN_CLICKED](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification.
	pub fn bn_clicked<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.parent_events.wm_command(co::CMD::BN_CLICKED, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [BN_DBLCLK](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification.
	pub fn bn_dbl_clk<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.parent_events.wm_command(co::CMD::BN_DBLCLK, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [BN_KILLFOCUS](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification.
	pub fn bn_kill_focus<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.parent_events.wm_command(co::CMD::BN_KILLFOCUS, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}

	/// Adds a handler to
	/// [BN_SETFOCUS](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification.
	pub fn bn_set_focus<F>(&self, func: F)
		where F: FnMut() + Send + Sync + 'static,
	{
		self.parent_events.wm_command(co::CMD::BN_SETFOCUS, self.ctrl_id, {
			let mut func = func;
			move || func()
		});
	}
}