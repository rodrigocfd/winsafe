use std::ptr::NonNull;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;

/// Exposes button control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
/// for a [`RadioGroup`](crate::gui::RadioGroup).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct RadioGroupEvents {
	parent_ptr: NonNull<BaseWnd>, // used only to add the events to parent, before the first message is processed
	ctrl_ids: Vec<u16>,
}

impl RadioGroupEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<BaseWnd>, ctrl_ids: Vec<u16>) -> Self {
		Self {
			parent_ptr: NonNull::from(parent.as_ref()),
			ctrl_ids,
		}
	}

	/// [`BN_CLICKED`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-clicked)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user clicks a button.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// let radios: gui::RadioGroup;
	/// # let radios = gui::RadioGroup::new(&wnd, &[]);
	///
	/// radios.on().bn_clicked({
	///     let radios = radios.clone();
	///     move || -> w::AnyResult<()> {
	///         println!("Selected {}",
	///             radios.selected().unwrap()
	///                 .hwnd().GetWindowText()?,
	///         );
	///         Ok(())
	///     }
	/// });
	/// # w::SysResult::Ok(())
	/// ```
	pub fn bn_clicked<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			parent_base_ref.on().wm_command(*ctrl_id, co::BN::CLICKED, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}

	/// [`BN_DBLCLK`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when the user double-clicks a button. This notification code is
	/// sent automatically for [`BS::USERBUTTON`](crate::co::BS::USERBUTTON),
	/// [`BS::RADIOBUTTON`](crate::co::BS::RADIOBUTTON), and
	/// [`BS::OWNERDRAW`](crate::co::BS::OWNERDRAW) buttons. Other button types
	/// send only if they have the [`BS::NOTIFY`](crate::co::BS::NOTIFY) style.
	pub fn bn_dbl_clk<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			parent_base_ref.on().wm_command(*ctrl_id, co::BN::DBLCLK, {
				let shared_func = shared_func.clone();
				move || shared_func()
			});
		}
	}

	/// [`BN_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button loses the keyboard focus. The button must have the
	/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
	/// code.
	pub fn bn_kill_focus<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			parent_base_ref
				.on()
				.wm_command(*ctrl_id, co::BN::KILLFOCUS, {
					let shared_func = shared_func.clone();
					move || shared_func()
				});
		}
	}

	/// [`BN_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
	/// command notification for all radio buttons in the group.
	///
	/// Sent when a button receives the keyboard focus. The button must have the
	/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this notification
	/// code.
	pub fn bn_set_focus<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		let shared_func = Rc::new(func);

		for ctrl_id in self.ctrl_ids.iter() {
			parent_base_ref
				.on()
				.wm_command(*ctrl_id, co::BN::SETFOCUS, {
					let shared_func = shared_func.clone();
					move || shared_func()
				});
		}
	}
}
