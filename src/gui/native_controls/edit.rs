use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{EditEvents, MsgEvents};
use crate::gui::native_controls::native_control_base::{NativeControlBase, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::WmSetFont;
use crate::structs::{POINT, SIZE};

/// Native
/// [edit](https://docs.microsoft.com/en-us/windows/win32/controls/about-edit-controls)
/// control.
#[derive(Clone)]
pub struct Edit {
	base: Arc<
		NativeControlBase<EditEvents, EditOpts>,
	>,
}

unsafe impl Send for Edit {}
unsafe impl Sync for Edit {}

impl Child for Edit {
	fn hctrl_ref(&self) -> &HWND {
		self.base.hctrl_ref()
	}
}

impl Edit {
	/// Instantiates a new `Edit` object, to be created on the parent window with
	/// [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: EditOpts) -> Edit {
		let opts = EditOpts::define_ctrl_id(opts);
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					EditEvents::new(parent, opts.ctrl_id),
					OptsId::Wnd(opts),
				),
			),
		};
		parent.privileged_events_ref().wm_create({
			let me = me.clone();
			move |_| { me.create(); 0 }
		});
		me
	}

	/// Instantiates a new `Edit` object, to be loaded from a dialog resource
	/// with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Edit {
		let me = Self {
			base: Arc::new(
				NativeControlBase::new(
					parent,
					EditEvents::new(parent, ctrl_id),
					OptsId::Dlg(ctrl_id),
				),
			),
		};
		parent.privileged_events_ref().wm_init_dialog({
			let me = me.clone();
			move |_| { me.create(); true }
		});
		me
	}

	fn create(&self) {
		|| -> WinResult<()> {
			match self.base.opts_id() {
				OptsId::Wnd(opts) => {
					let mut pos = opts.position;
					let mut sz = SIZE::new(opts.width as i32, opts.height as i32);
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					let our_hwnd = self.base.create_window( // may panic
						"EDIT", Some(&opts.text), pos, sz,
						opts.ctrl_id,
						opts.ex_window_style,
						opts.window_style | opts.edit_style.into(),
					)?;

					our_hwnd.SendMessage(WmSetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

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

	/// Exposes the edit events.
	///
	/// These event methods are just proxies to the
	/// [`MsgEvents`](crate::gui::events::MsgEvents) of the parent window, who is
	/// the real responsible for the child event handling.
	///
	/// # Panics
	///
	/// Panics if the control or the parent window are already created. Events
	/// must be set before control and parent window creation.
	pub fn on(&self) -> &EditEvents {
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

	/// Sets the text in the control.
	pub fn set_text(&self, text: &str) {
		self.hwnd().SetWindowText(text)
			.unwrap_or_else(|err| PostQuitMessage(err));
	}

	/// Retrieves the text in the control.
	pub fn text(&self) -> String {
		self.hwnd().GetWindowTextStr()
			.unwrap_or_else(|err| { PostQuitMessage(err); String::default() })
	}
}

//------------------------------------------------------------------------------

/// Options to create an [`Edit`](crate::gui::Edit) programatically with
/// [`Edit::new`](crate::gui::Edit::new).
pub struct EditOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	///
	/// **Tip:** To vertically align side-by-side with a button, add 1 to `y`.
	/// That's necessary because default button height is 23, while edit is 21.
	pub position: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 100.
	pub width: u32,
	/// Control height, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 21.
	///
	/// **Note:** You should change the default height only in a multi-line edit.
	pub height: u32,
	/// Edit styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `ES::AUTOHSCROLL | ES::NOHIDESEL`.
	///
	/// Suggestions:
	/// * add `ES::PASSWORD` for a password input;
	/// * add `ES::NUMBER` to accept only numbers;
	/// * replace with `ES::MULTILINE | ES:WANTRETURN | ES:AUTOVSCROLL | ES::NOHIDESEL` for a multi-line edit.
	pub edit_style: co::ES,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub ex_window_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
}

impl Default for EditOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			width: 100,
			height: 21,
			edit_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			ex_window_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
		}
	}
}

impl EditOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
