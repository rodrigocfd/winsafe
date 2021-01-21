/// Implements methods common to controls.
macro_rules! hwnd_ctrlid_on_onsubclass {
	($evstruc:ident) => {
		/// Returns the underlying handle for this control.
		///
		/// Note that the handle is initially null, receiving an actual value only
		/// after the control is created.
		pub fn hwnd(&self) -> HWND {
			*self.hctrl_ref()
		}

		/// Returns the control ID.
		pub fn ctrl_id(&self) -> u16 {
			match self.base.opts_id() {
				OptsId::Wnd(opts) => opts.ctrl_id,
				OptsId::Dlg(ctrl_id) => *ctrl_id,
			}
		}

		/// Exposes the control events.
		///
		/// These event methods are just proxies to the
		/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who
		/// is the real responsible for the child event handling.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created. Events
		/// must be set before control and parent window creation.
		pub fn on(&self) -> &$evstruc {
			self.base.on()
		}

		/// Exposes the subclass events. If at least one event exists, the control
		/// will be
		/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
		///
		/// **Note:** Subclassing may impact performance, use with care.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created. Events
		/// must be set before control and parent window creation.
		pub fn on_subclass(&self) -> &MsgEvents {
			self.base.on_subclass()
		}
	};
}
