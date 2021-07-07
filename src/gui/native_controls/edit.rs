use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::EditEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::{POINT, SIZE};
use crate::various::WString;

/// Native
/// [edit](https://docs.microsoft.com/en-us/windows/win32/controls/about-edit-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct Edit(Arc<Obj>);

struct Obj { // actual fields of Edit
	base: BaseNativeControl,
	opts_id: OptsId<EditOpts>,
	events: EditEvents,
}

impl_send_sync_child!(Edit);

impl Edit {
	/// Instantiates a new `Edit` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: EditOpts) -> Edit {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = EditOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: EditEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `Edit` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Edit {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: EditEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(); true }
		});

		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => {
					let mut pos = opts.position;
					let mut sz = SIZE::new(opts.width as _, opts.height as _);
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					let our_hwnd = self.0.base.create_window( // may panic
						"EDIT", Some(&opts.text), pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.edit_style.into(),
					)?;

					our_hwnd.SendMessage(wm::SetFont{ hfont: ui_font(), redraw: true });
					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_ctrlid_hwnd_on_onsubclass!(EditEvents);

	/// Sets the text in the control by calling
	/// [`SetWindowText`](crate::HWND::SetWindowText).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// my_edit.set_text("This my text").unwrap();
	/// ```
	pub fn set_text(&self, text: &str) -> WinResult<()> {
		self.hwnd().SetWindowText(text)
	}

	/// Retrieves the text in the control by calling
	/// [`GetWindowText`](crate::HWND::GetWindowText).
	///
	/// The passed buffer will be automatically allocated.
	///
	/// This method can be more performant than
	/// [`text_str`](crate::gui::Edit::text_str) because the buffer can be
	/// reused, avoiding multiple allocations. However, it has the inconvenient
	/// of the manual conversion from [`WString`](crate::WString) to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{gui, WString};
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// let mut buf = WString::default;
	/// my_edit.text(&mut buf).unwrap();
	///
	/// println!("The text is: {}", buf.to_string());
	/// ```
	pub fn text(&self, buf: &mut WString) -> WinResult<()> {
		self.hwnd().GetWindowText(buf).map(|_| ())
	}

	/// A more convenient [`text`](crate::gui::Edit::text), which directly
	/// returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	///
	/// println!("The text is: {}", my_edit.text().unwrap());
	/// ```
	pub fn text_str(&self) -> WinResult<String> {
		self.hwnd().GetWindowTextStr()
	}
}

//------------------------------------------------------------------------------

/// Options to create an [`Edit`](crate::gui::Edit) programmatically with
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
	pub window_ex_style: co::WS_EX,

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
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
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
