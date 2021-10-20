use crate::aliases::ErrResult;
use crate::co;
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::structs::{NMBCDROPDOWN, NMBCHOTITEM, NMCUSTOMDRAW};

/// Exposes button control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
pub struct ButtonEvents(BaseEventsProxy);

impl ButtonEvents {
	pub(in crate::gui) fn new(parent_base_ref: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base_ref, ctrl_id))
	}

	pub_fn_nfy_ret0_param! { bcn_drop_down, co::BCN::DROPDOWN.into(), NMBCDROPDOWN,
		/// [`BCN_DROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-dropdown)
		/// notification.
		///
		/// Sent when the user clicks a drop down arrow on a button.
	}

	pub_fn_nfy_ret0_param! { bcn_hot_item_change, co::BCN::HOTITEMCHANGE.into(), NMBCHOTITEM,
		/// [`BCN_HOTITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/bcn-hotitemchange)
		/// notification.
		///
		/// Notifies the button control owner that the mouse is entering or
		/// leaving the client area of the button control.
	}

	pub_fn_cmd_ret0! { bn_clicked, co::BN::CLICKED.into(),
		/// [`BN_CLICKED`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-clicked)
		/// command notification.
		///
		/// Sent when the user clicks a button.
		///
		/// # Examples
		///
		/// ```rust,ignore
		/// use winsafe::prelude::*;
		/// use winsafe::{gui, ErrResult};
		///
		/// let btn: gui::Button; // initialized somewhere
		///
		/// btn.on().bn_clicked({
		///     let btn = btn.clone(); // pass into the closure
		///     move || -> ErrResult<()> {
		///         println!("HWND: {}", btn.hwnd());
		///         Ok(())
		///     }
		/// });
		/// ```
	}

	pub_fn_cmd_ret0! { bn_dbl_clk, co::BN::DBLCLK.into(),
		/// [`BN_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-dblclk)
		/// command notification.
		///
		/// Sent when the user double-clicks a button. This notification code is
		/// sent automatically for
		/// [`BS::USERBUTTON`](crate::co::BS::USERBUTTON),
		/// [`BS::RADIOBUTTON`](crate::co::BS::RADIOBUTTON), and
		/// [`BS::OWNERDRAW`](crate::co::BS::OWNERDRAW) buttons. Other button
		/// types send only if they have the
		/// [`BS::NOTIFY`](crate::co::BS::NOTIFY) style.
	}

	pub_fn_cmd_ret0! { bn_kill_focus, co::BN::KILLFOCUS.into(),
		/// [`BN_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-killfocus)
		/// command notification.
		///
		/// Sent when a button loses the keyboard focus. The button must have
		/// the [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this
		/// notification code.
	}

	pub_fn_cmd_ret0! { bn_set_focus, co::BN::SETFOCUS.into(),
		/// [`BN_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/bn-setfocus)
		/// command notification.
		///
		/// Sent when a button receives the keyboard focus. The button must have
		/// the [`BS::NOTIFY`](crate::co::BS::NOTIFY) style to send this
		/// notification code.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-button)
	/// notification.
	///
	/// Notifies about custom draw operations on the button.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMCUSTOMDRAW) -> ErrResult<co::CDRF> + 'static,
	{
		self.0.add_nfy(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMCUSTOMDRAW>() })?.0 as _)));
	}
}
