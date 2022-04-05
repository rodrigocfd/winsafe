use std::any::Any;
use std::sync::Arc;

use crate::co;
use crate::gui::events::{TrackbarEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl, OptsId,
};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::gui::resizer::{Horz, Vert};
use crate::kernel::decl::WinResult;
use crate::msg::trbm;
use crate::prelude::{
	AsAny, GuiChild, GuiEventsView, GuiFocusControl, GuiNativeControl,
	GuiNativeControlEvents, GuiParent, GuiWindow, Handle, UserHwnd,
};
use crate::user::decl::{HWND, POINT, SIZE};

/// Native
/// [trackbar](https://docs.microsoft.com/en-us/windows/win32/controls/trackbar-controls)
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct Trackbar(Arc<Obj>);

struct Obj { // actual fields of Trackbar
	base: BaseNativeControl,
	opts_id: OptsId<TrackbarOpts>,
	events: TrackbarEvents,
}

impl AsAny for Trackbar {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiWindow for Trackbar {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl GuiChild for Trackbar {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl GuiNativeControl for Trackbar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<TrackbarEvents> for Trackbar {
	fn on(&self) -> &TrackbarEvents {
		if !self.hwnd().is_null() {
			panic!("Cannot add events after the control creation.");
		} else if !self.0.base.parent_base().hwnd().is_null() {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl GuiFocusControl for Trackbar {}

impl Trackbar {
	/// Instantiates a new `Trackbar` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::prelude::UserHwnd::CreateWindowEx).
	pub fn new(parent: &impl GuiParent, opts: TrackbarOpts) -> Trackbar {
		let opts = TrackbarOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
					events: TrackbarEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm(parent.as_base().wmcreate_or_wminitdialog(), {
			let self2 = new_self.clone();
			move |_| self2.create(horz, vert)
				.map_err(|e| e.into())
				.map(|_| 0)
		});
		new_self
	}

	/// Instantiates a new `Trackbar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::UserHwnd::GetDlgItem).
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> Trackbar
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TrackbarEvents::new(parent.as_base(), ctrl_id),
				},
			),
		);

		parent.as_base().privileged_on().wm_init_dialog({
			let self2 = new_self.clone();
			move |_| self2.create(resize_behavior.0, resize_behavior.1)
				.map_err(|e| e.into())
				.map(|_| true)
		});
		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = opts.size;
				multiply_dpi_or_dtu(
					self.0.base.parent_base(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window( // may panic
					"msctls_trackbar32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.trackbar_style.into(),
				)?;

				if opts.range != (0, 100) {
					self.set_range(opts.range.0, opts.range.1);
				}
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?, // may panic
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Retrieves the current position by sending a
	/// [`trbm::GetPos`](crate::msg::trbm::GetPos) message.
	pub fn pos(&self) -> u32 {
		self.hwnd().SendMessage(trbm::GetPos {})
	}

	/// Retrieves the minimum and maximum position values by sending
	/// [`trbm::GetRangeMin`](crate::msg::trbm::GetRangeMin) and
	/// [`trbm::GetRangeMax`](crate::msg::trbm::GetRangeMax) messages.
	pub fn range(&self) -> (u32, u32) {
		(
			self.hwnd().SendMessage(trbm::GetRangeMin {}),
			self.hwnd().SendMessage(trbm::GetRangeMax {}),
		)
	}

	/// Sets the current position by sending a
	/// [`trbm::SetPos`](crate::msg::trbm::SetPos) message.
	pub fn set_pos(&self, pos: u32) {
		self.hwnd().SendMessage(trbm::SetPos { redraw: true, pos });
	}

	/// Sets the minimum and maximum position values by sending
	/// [`trbm::SetRangeMin`](crate::msg::trbm::SetRangeMin) and
	/// [`trbm::SetRangeMax`](crate::msg::trbm::SetRangeMax) messages.
	pub fn set_range(&self, min: u32, max: u32) {
		self.hwnd().SendMessage(trbm::SetRangeMin { redraw: false, min });
		self.hwnd().SendMessage(trbm::SetRangeMax { redraw: true, max });
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Trackbar`](crate::gui::Trackbar) programmatically with
/// [`Trackbar::new`](crate::gui::Trackbar::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct TrackbarOpts {
	/// Control position within parent client area, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to 120 x 23.
	pub size: SIZE,
	/// Trackbar styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `TBS::HORZ | TBS::AUTOTICKS`.
	pub trackbar_style: co::TBS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,

	/// The minimum and maximum position values.
	///
	/// Defaults to 0 and 100.
	pub range: (u32, u32),
}

impl Default for TrackbarOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(120, 23),
			trackbar_style: co::TBS::HORZ | co::TBS::AUTOTICKS,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			range: (0, 100),
		}
	}
}

impl TrackbarOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
