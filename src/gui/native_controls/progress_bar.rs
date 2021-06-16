use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::pbm;
use crate::structs::{PBRANGE, POINT, SIZE};

/// Native
/// [progress bar](https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct ProgressBar(Arc<Obj>);

struct Obj { // actual fields of ProgressBar
	base: BaseNativeControl,
	opts_id: OptsId<ProgressBarOpts>,
}

impl_send_sync_child!(ProgressBar);

impl ProgressBar {
	/// Instantiates a new `ProgressBar` object, to be created on the parent
	/// window with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, opts: ProgressBarOpts) -> ProgressBar {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = ProgressBarOpts::define_ctrl_id(opts);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});

		new_self
	}

	/// Instantiates a new `ProgressBar` object, to be loaded from a dialog
	/// resource with [`GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(parent: &dyn Parent, ctrl_id: u16) -> ProgressBar {
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
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
						"msctls_progress32", None, pos, sz,
						opts.ctrl_id,
						opts.window_ex_style,
						opts.window_style | opts.progress_bar_style.into(),
					)?;

					Ok(())
				},
				OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ()), // may panic
			}
		}().unwrap_or_else(|err| PostQuitMessage(err))
	}

	pub_fn_hwnd_onsubclass!();

	/// Retrieves the current position by sending a
	/// [`PBM_GETPOS`](crate::msg::pbm::GetPos) message.
	pub fn position(&self) -> u32 {
		self.hwnd().SendMessage(pbm::GetPos {})
	}

	/// Retrieves the current minimum and maximum values by sending a
	/// [`PBM_GETRANGE`](crate::msg::pbm::GetRange) message. Default values are
	/// 0 and 100.
	pub fn range(&self) -> (u32, u32) {
		let mut ranges = PBRANGE::default();
		self.hwnd().SendMessage(pbm::GetRange {
			return_low: false, // indifferent, return value not used
			ranges: Some(&mut ranges),
		});
		(ranges.iLow as _, ranges.iHigh as _)
	}

	/// Sets or unsets the marquee mode by sending a
	/// [`PBM_SETMARQUEE`](crate::msg::pbm::SetMarquee) message combined with a
	/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) call for a style
	/// change.
	pub fn set_marquee(&self, marquee: bool) {
		if marquee {
			self.hwnd().SetWindowLongPtr(
				co::GWLP::STYLE,
				u32::from(self.cur_style() | co::PBS::MARQUEE) as _,
			);
		}

		self.hwnd().SendMessage(pbm::SetMarquee {
			turn_on: marquee,
			time_ms: None,
		});

		if !marquee {
			self.hwnd().SetWindowLongPtr(
				co::GWLP::STYLE,
				u32::from(self.cur_style() & !co::PBS::MARQUEE) as _,
			);
		}
	}

	/// Sets the current position by sending a
	/// [`PBM_SETPOS`](crate::msg::pbm::SetPos) message, returning the previous
	/// position.
	pub fn set_position(&self, position: u32) -> u32 {
		if self.cur_style().has(co::PBS::MARQUEE) {
			self.set_marquee(false); // avoid crash
		}

		self.hwnd().SendMessage(pbm::SetPos { position })
	}

	/// Sets the minimum and maximum values by sending a
	/// [`PBM_SETRANGE32`](crate::msg::pbm::SetRange32) message. Default values
	/// are 0 and 100.
	pub fn set_range(&self, min: u32, max: u32) {
		self.hwnd().SendMessage(pbm::SetRange32 { min, max })
	}

	/// Sets the current state by sending a
	/// [`PBM_SETSTATE`](crate::msg::pbm::SetState) message, retuning the
	/// previous state.
	pub fn set_state(&self, state: co::PBST) -> co::PBST {
		self.hwnd().SendMessage(pbm::SetState { state })
	}

	/// Retrieves the current state by sending a
	/// [`PBM_GETSTATE`](crate::msg::pbm::GetState) message.
	pub fn state(&self) -> co::PBST {
		self.hwnd().SendMessage(pbm::GetState {})
	}

	fn cur_style(&self) -> co::PBS {
		co::PBS(self.hwnd().GetWindowLongPtr(co::GWLP::STYLE) as _)
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ProgressBar`](crate::gui::ProgressBar)
/// programmatically with [`ProgressBar::new`](crate::gui::ProgressBar::new).
pub struct ProgressBarOpts {
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
	/// Progress bar styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `PBS::SMOOTH`.
	pub progress_bar_style: co::PBS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
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
}

impl Default for ProgressBarOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			size: SIZE::new(120, 23),
			progress_bar_style: co::PBS::SMOOTH,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
		}
	}
}

impl ProgressBarOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
