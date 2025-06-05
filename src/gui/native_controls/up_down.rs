use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

struct UpDownObj {
	base: BaseCtrl,
	events: UpDownEvents,
	_pin: PhantomPinned,
}

native_ctrl! { UpDown: UpDownObj => UpDownEvents;
	/// Native
	/// [up-down](https://learn.microsoft.com/en-us/windows/win32/controls/up-down-controls)
	/// control.
	///
	/// Note that if the `UpDown` is created with
	/// [`UDS::AUTOBUDDY`](crate::co::UDS::AUTOBUDDY) style, it takes the control
	/// created immediately before the `UpDown` as the buddy one, attaching the
	/// `UpDown` to it. This control should be an [`Edit`](crate::gui::Edit) with
	/// [`ES::NUMBER`](crate::co::ES::NUMBER) style.
}

impl UpDown {
	/// Instantiates a new `UpDown` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create an `UpDown` in an event closure.
	///
	/// # Examples
	///
	/// In the example below, the `UpDown` has
	/// [`UDS::AUTOBUDDY`](crate::co::UDS::AUTOBUDDY) style by default, so it
	/// will take the [`Edit`](crate::gui::Edit), which was created immediately
	/// prior, as its buddy control. The `UpDown` will automatically attach
	/// itself to the `Edit`.
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let txt = gui::Edit::new(
	///     &wnd,
	///     gui::EditOpts {
	///         control_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL | co::ES::NUMBER,
	///         ..Default::default()
	///     },
	/// );
	///
	/// let updn = gui::UpDown::new(
	///     &wnd,
	///     gui::UpDownOpts {
	///         range: (0, 50),
	///         ..Default::default()
	///     },
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), opts: UpDownOpts) -> Self {
		let ctrl_id = auto_id::set_if_zero(opts.ctrl_id);
		let new_self = Self(Arc::pin(UpDownObj {
			base: BaseCtrl::new(ctrl_id),
			events: UpDownEvents::new(parent, ctrl_id),
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
					"msctls_updown32",
					None,
					opts.window_style | opts.control_style.into(),
					opts.position.into(),
					SIZE::new(0, opts.height),
					&parent2,
				)?;
				if opts.range != (0, 100) {
					self2.set_range(opts.range.0, opts.range.1);
					if opts.control_style.has(co::UDS::AUTOBUDDY) {
						let prev_ctrl = self2.hwnd().GetWindow(co::GW::HWNDPREV)?;
						prev_ctrl.SetWindowText(&opts.range.0.to_string())?;
					}
				}
				if opts.value != 0 {
					self2.set_pos(opts.value);
				}
				Ok(0) // ignored
			});

		new_self
	}

	/// Instantiates a new `UpDown` object, to be loaded from a dialog resource
	/// with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create an `UpDown` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &(impl GuiParent + 'static),
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self {
		let new_self = Self(Arc::pin(UpDownObj {
			base: BaseCtrl::new(ctrl_id),
			events: UpDownEvents::new(parent, ctrl_id),
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

		new_self
	}

	/// Retrieves the current position by sending an
	/// [`udm::GetPos32`](crate::msg::udm::GetPos32) message.
	#[must_use]
	pub fn pos(&self) -> i32 {
		unsafe {
			self.hwnd()
				.SendMessage(udm::GetPos32 { success_flag: None })
		}
	}

	/// Retrieves the minimum and maximum position values by sending an
	/// [`udm::GetRange32`](crate::msg::udm::GetRange32) message.
	#[must_use]
	pub fn range(&self) -> (i32, i32) {
		let (mut min, mut max) = (i32::default(), i32::default());
		unsafe {
			self.hwnd()
				.SendMessage(udm::GetRange32 { min: &mut min, max: &mut max });
		}
		(min, max)
	}

	/// Sets the current position by sending an
	/// [`udm::SetPos32`](crate::msg::udm::SetPos32) message.
	pub fn set_pos(&self, pos: i32) {
		unsafe {
			self.hwnd().SendMessage(udm::SetPos32 { pos });
		}
	}

	/// Set the control range by sending an
	/// [`udm::SetRange32`](crate::msg::udm::SetRange32) message.
	pub fn set_range(&self, min: i32, max: i32) {
		unsafe {
			self.hwnd().SendMessage(udm::SetRange32 { min, max });
		}
	}
}

/// Options to create an [`UpDown`](crate::gui::UpDown) programmatically with
/// [`UpDown::new`](crate::gui::UpDown::new).
pub struct UpDownOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Note that the `UDS::AUTOBUDDY` style automatically positions the
	/// `UpDown`; thus, with this style, `position` is meaningless.
	///
	/// Defaults to `gui::dpi(0, 0)`.
	pub position: (i32, i32),
	/// Control height to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Note that the `UDS::AUTOBUDDY` style automatically resizes the `UpDown`;
	/// thus, with this style, `height` is meaningless.
	///
	/// Defaults to `gui::dpi(40)`.
	pub height: i32,
	/// Up-down styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Note that the `UDS::AUTOBUDDY` style will take the control created
	/// immediately before the `UpDown` as the buddy one, attaching the `UpDown`
	/// to it. This control should be an [`Edit`](crate::gui::Edit) with
	/// [`ES::NUMBER`](crate::co::ES::NUMBER) style.
	///
	/// Defaults to `UDS::AUTOBUDDY | UDS::SETBUDDYINT | UDS::ALIGNRIGHT | UDS::ARROWKEYS | UDS::HOTTRACK`.
	pub control_style: co::UDS,
	/// Window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,

	/// The minimum and maximum position values.
	///
	/// Defaults to `(0, 100)`.
	pub range: (i32, i32),
	/// Initial position value.
	///
	/// Defaults to `0`.
	pub value: i32,
}

impl Default for UpDownOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			height: 40,
			control_style: co::UDS::AUTOBUDDY
				| co::UDS::SETBUDDYINT
				| co::UDS::ALIGNRIGHT
				| co::UDS::ARROWKEYS
				| co::UDS::HOTTRACK,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			range: (0, 100),
			value: 0,
		}
	}
}
