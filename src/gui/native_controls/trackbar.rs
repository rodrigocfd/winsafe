use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::TrackbarEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::trbm;
use crate::structs::{POINT, SIZE};

/// Native
/// [trackbar](https://docs.microsoft.com/en-us/windows/win32/controls/trackbar-controls)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct Trackbar(Arc<Obj>);

struct Obj { // actual fields of Trackbar
	base: BaseNativeControl,
	opts_id: OptsId<TrackbarOpts>,
	events: TrackbarEvents,
}

impl_send_sync_child!(Trackbar);

impl Trackbar {
	/// Instantiates a new `Trackbar` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: TrackbarOpts) -> Trackbar {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = TrackbarOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: TrackbarEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `Trackbar` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> Trackbar {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: TrackbarEvents::new(parent_base_ref, ctrl_id),
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
					let mut sz = opts.size;
					multiply_dpi(Some(&mut pos), Some(&mut sz))?;

					self.0.base.create_window( // may panic
						"msctls_trackbar32", None, pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.trackbar_style.into(),
					)?;

					if opts.range != (0, 100) {
						self.set_range(opts.range.0, opts.range.1);
					}

					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_ctrlid_hwnd_on_onsubclass!(TrackbarEvents);

	/// Retrieves the current position by sending a
	/// [`TBM_GETPOS`](crate::msg::trbm::GetPos) message.
	pub fn pos(&self) -> u32 {
		self.hwnd().SendMessage(trbm::GetPos {})
	}

	/// Retrieves the minimum and maximum position values by sending
	/// [`TBM_GETRANGEMIN`](crate::msg::trbm::GetRangeMin) and
	/// [`TBM_GETRANGEMAX`](crate::msg::trbm::GetRangeMax) messages.
	pub fn range(&self) -> (u32, u32) {
		(
			self.hwnd().SendMessage(trbm::GetRangeMin {}),
			self.hwnd().SendMessage(trbm::GetRangeMax {}),
		)
	}

	/// Sets the current position by sending a
	/// [`TBM_SETPOS`](crate::msg::trbm::SetPos) message.
	pub fn set_pos(&self, pos: u32) {
		self.hwnd().SendMessage(trbm::SetPos { redraw: true, pos });
	}

	/// Sets the minimum and maximum position values by sending
	/// [`TBM_SETRANGEMIN`](crate::msg::trbm::SetRangeMin) and
	/// [`TBM_SETRANGEMAX`](crate::msg::trbm::SetRangeMax) messages.
	pub fn set_range(&self, min: u32, max: u32) {
		self.hwnd().SendMessage(trbm::SetRangeMin { redraw: false, min });
		self.hwnd().SendMessage(trbm::SetRangeMax { redraw: true, max });
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`Trackbar`](crate::gui::Trackbar) programmatically with
/// [`Trackbar::new`](crate::gui::Trackbar::new).
pub struct TrackbarOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
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