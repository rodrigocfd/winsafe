use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct EditObj {
	base: BaseCtrl,
	events: EditEvents,
	_pin: PhantomPinned,
}

native_ctrl! { Edit: EditObj => EditEvents;
	/// Native
	/// [edit](https://learn.microsoft.com/en-us/windows/win32/controls/about-edit-controls)
	/// (text box) control.
}

impl Edit {
	/// Instantiates a new `Edit` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create an `Edit` in an event closure.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let txt = gui::Edit::new(
	///     &wnd,
	///     gui::EditOpts {
	///         position: gui::dpi(10, 10),
	///         width: gui::dpi_x(120),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: EditOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(EditObj {
			base: BaseCtrl::new(ctrl_id),
			events: EditEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				self2.0.base.create_window(
					opts.window_ex_style,
					"EDIT",
					Some(&opts.text),
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::new(opts.width, opts.height),
					&parent2,
				)?;
				ui_font::set(self2.hwnd())?;
				parent2
					.as_ref()
					.add_to_layout(self2.hwnd(), opts.resize_behavior)?;
				Ok(0) // ignored
			});

		new_self.default_message_handlers(parent);
		new_self
	}

	/// Instantiates a new `Edit` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create an `Edit` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(EditObj {
			base: BaseCtrl::new(ctrl_id),
			events: EditEvents::new(parent, ctrl_id),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent.as_ref().before_on().wm_init_dialog(move |_| {
			self2.0.base.assign_dlg(&parent2)?;
			parent2
				.as_ref()
				.add_to_layout(self2.hwnd(), resize_behavior)?;
			Ok(true) // ignored
		});

		new_self.default_message_handlers(parent);
		new_self
	}

	fn default_message_handlers(&self, parent: &(impl GuiParent + 'static)) {
		let self2 = self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm_command(self.ctrl_id(), co::EN::CHANGE, move || {
				// EN_CHANGE is first sent to the control before CreateWindowEx()
				// returns, so if the user handles EN_CHANGE, the Edit HWND won't be
				// set yet. So we set the HWND here.
				if *self2.hwnd() == HWND::NULL {
					let hctrl = parent2.as_ref().hwnd().GetDlgItem(self2.ctrl_id())?;
					self2.0.base.set_hwnd(hctrl);
				}
				Ok(())
			});
	}

	/// Hides any balloon tip by sending an
	/// [`em::HideBalloonTip`](crate::msg::em::HideBalloonTip) message.
	pub fn hide_balloon_tip(&self) -> SysResult<()> {
		unsafe { self.hwnd().SendMessage(em::HideBalloonTip {}) }
	}

	/// Limits the number of characters that can be type by sending an
	/// [`em::SetLimitText`](crate::msg::em::SetLimitText) message.
	pub fn limit_text(&self, max_chars: Option<u32>) {
		unsafe {
			self.hwnd().SendMessage(em::SetLimitText { max_chars });
		}
	}

	/// Sets the selection range of the text by sending an
	/// [`em::SetSel`](crate::msg::em::SetSel) message.
	///
	/// # Examples
	///
	/// Selecting all text in the control:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_edit = gui::Edit::new(&wnd, gui::EditOpts::default());
	///
	/// my_edit.set_selection(0, -1);
	/// ```
	///
	/// Clearing the selection:
	///
	/// ```no_run
	/// use winsafe::gui;
	///
	/// let my_edit: gui::Edit; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	/// # let my_edit = gui::Edit::new(&wnd, gui::EditOpts::default());
	///
	/// my_edit.set_selection(-1, -1);
	/// ```
	pub fn set_selection(&self, start: i32, end: i32) {
		unsafe {
			self.hwnd().SendMessage(em::SetSel { start, end });
		}
	}

	/// Sets the text by calling
	/// [`HWND::SetWindowText`](crate::HWND::SetWindowText).
	pub fn set_text(&self, text: &str) -> SysResult<()> {
		self.hwnd().SetWindowText(text)?;
		Ok(())
	}

	/// Displays a balloon tip by sending an
	/// [`em::ShowBalloonTip`](crate::msg::em::ShowBalloonTip) message.
	pub fn show_ballon_tip(&self, title: &str, text: &str, icon: co::TTI) -> SysResult<()> {
		let mut title16 = WString::from_str(title);
		let mut text16 = WString::from_str(text);

		let mut info = EDITBALLOONTIP::default();
		info.set_pszTitle(Some(&mut title16));
		info.set_pszText(Some(&mut text16));
		info.ttiIcon = icon;

		unsafe { self.hwnd().SendMessage(em::ShowBalloonTip { info: &info }) }
	}

	/// Retrieves the text by calling
	/// [`HWND::GetWindowText`](crate::HWND::GetWindowText).
	#[must_use]
	pub fn text(&self) -> SysResult<String> {
		self.hwnd().GetWindowText()
	}
}

/// Options to create an [`Edit`](crate::gui::Edit) programmatically with
/// [`Edit::new`](crate::gui::Edit::new).
pub struct EditOpts {
	/// Text of the control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control width to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_x(100)`.
	pub width: i32,
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `gui::dpi_y(23)`.
	///
	/// **Note:** You should change the default height only in a multi-line
	/// edit, otherwise it will look off.
	pub height: i32,
	/// Edit styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `ES::AUTOHSCROLL | ES::NOHIDESEL`.
	///
	/// Suggestions:
	/// * add `ES::PASSWORD` for a password input;
	/// * add `ES::NUMBER` to accept only numbers;
	/// * replace with `ES::MULTILINE | ES::WANTRETURN | ES::AUTOVSCROLL | ES::NOHIDESEL` for a multi-line edit.
	pub control_style: co::ES,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::GROUP | WS::TABSTOP | WS::VISIBLE`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::CLIENTEDGE`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// **Note:** You should use `Vert::Resize` only in a multi-line edit.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for EditOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: dpi(0, 0),
			width: dpi_x(100),
			height: dpi_y(23),
			control_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
			window_style: co::WS::CHILD | co::WS::GROUP | co::WS::TABSTOP | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT | co::WS_EX::CLIENTEDGE,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}
