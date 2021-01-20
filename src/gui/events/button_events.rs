use std::ptr::NonNull;

use crate::co;
use crate::gui::events::MsgEvents;
use crate::gui::traits::Parent;
use crate::structs::{NMBCDROPDOWN, NMBCHOTITEM, NMCUSTOMDRAW};

/// Exposes button
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
pub struct ButtonEvents {
	parent_user_events: NonNull<MsgEvents>, // used only before parent creation
	ctrl_id: u16,
}

impl ButtonEvents {
	pub(crate) fn new(parent: &dyn Parent, ctrl_id: u16) -> ButtonEvents {
		Self {
			parent_user_events: NonNull::from(parent.user_events_ref()), // convert reference to pointer
			ctrl_id,
		}
	}

	fn parent_user_events(&self) -> &MsgEvents {
		unsafe { self.parent_user_events.as_ref() }
	}

	nfy_event! { bcn_drop_down, co::NM::BCN_DROPDOWN, NMBCDROPDOWN,
		/// [`BCN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-dropdown)
		/// notification.
	}

	nfy_event! { bcn_hot_item_change, co::NM::BCN_HOTITEMCHANGE, NMBCHOTITEM,
		/// [`BCN_HOTITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-hotitemchange)
		/// notification.
	}

	cmd_event! { bn_clicked, co::CMD::BN_CLICKED,
		/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
		/// command notification.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::gui::Button;
		///
		/// let btn: Button; // initialize it somewhere
		///
		/// btn.on().bn_clicked({
		///   let btn = btn.clone(); // pass into the closure
		///   move || {
		///     println!("HWND: {}", btn.hwnd());
		///   }
		/// });
		/// ```
	}

	cmd_event! { bn_dbl_clk, co::CMD::BN_DBLCLK,
		/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
		/// command notification.
	}

	cmd_event! { bn_kill_focus, co::CMD::BN_KILLFOCUS,
		/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
		/// command notification.
	}

	cmd_event! { bn_set_focus, co::CMD::BN_SETFOCUS,
		/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
		/// command notification.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-button)
	/// notification.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: FnMut(&NMCUSTOMDRAW) -> co::CDRF + 'static,
	{
		self.parent_user_events().add_nfy(self.ctrl_id, co::NM::CUSTOMDRAW, {
			let mut func = func;
			move |p| Some(u32::from(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })) as isize)
		});
	}
}
