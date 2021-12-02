use std::any::Any;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co::{self, prelude::NativeBitflag};
use crate::gui::events::{prelude::EventsView, WindowEvents};
use crate::gui::native_controls::base_native_control::{
	BaseNativeControl,
	OptsId,
};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{AsAny, Child, NativeControl, Parent, Window};
use crate::handles::HWND;
use crate::msg::pbm;
use crate::structs::{PBRANGE, POINT, SIZE};

/// Native
/// [progress bar](https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control)
/// control.
#[derive(Clone)]
pub struct ProgressBar(Arc<Obj>);

struct Obj { // actual fields of ProgressBar
	base: BaseNativeControl,
	opts_id: OptsId<ProgressBarOpts>,
}

unsafe impl Send for ProgressBar {}

impl AsAny for ProgressBar {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl Window for ProgressBar {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl Child for ProgressBar {
	fn ctrl_id(&self) -> u16 {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => opts.ctrl_id,
			OptsId::Dlg(ctrl_id) => *ctrl_id,
		}
	}
}

impl NativeControl for ProgressBar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl ProgressBar {
	/// Instantiates a new `ProgressBar` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: ProgressBarOpts) -> ProgressBar {
		let opts = ProgressBarOpts::define_ctrl_id(opts);
		let (horz, vert) = (opts.horz_resize, opts.vert_resize);
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Wnd(opts),
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

	/// Instantiates a new `ProgressBar` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert)) -> ProgressBar
	{
		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent.as_base()),
					opts_id: OptsId::Dlg(ctrl_id),
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

				self.0.base.create_window(
					"msctls_progress32", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.progress_bar_style.into(),
				)?;
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?,
		}

		self.0.base.parent_base().add_to_resizer(self.hwnd(), horz, vert)
	}

	/// Retrieves the current position by sending a
	/// [`pbm::GetPos`](crate::msg::pbm::GetPos) message.
	pub fn position(&self) -> u32 {
		self.hwnd().SendMessage(pbm::GetPos {})
	}

	/// Retrieves the current minimum and maximum values by sending a
	/// [`pbm::GetRange`](crate::msg::pbm::GetRange) message. Default values are
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
	/// [`pbm::SetMarquee`](crate::msg::pbm::SetMarquee) message combined with a
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
	/// [`pbm::SetPos`](crate::msg::pbm::SetPos) message, returning the previous
	/// position.
	pub fn set_position(&self, position: u32) -> u32 {
		if self.cur_style().has(co::PBS::MARQUEE) {
			self.set_marquee(false); // avoid crash
		}

		self.hwnd().SendMessage(pbm::SetPos { position })
	}

	/// Sets the minimum and maximum values by sending a
	/// [`pbm::SetRange32`](crate::msg::pbm::SetRange32) message. Default values
	/// are 0 and 100.
	pub fn set_range(&self, min: u32, max: u32) {
		self.hwnd().SendMessage(pbm::SetRange32 { min, max })
	}

	/// Sets the current state by sending a
	/// [`pbm::SetState`](crate::msg::pbm::SetState) message, retuning the
	/// previous state.
	pub fn set_state(&self, state: co::PBST) -> co::PBST {
		self.hwnd().SendMessage(pbm::SetState { state })
	}

	/// Retrieves the current state by sending a
	/// [`pbm::GetState`](crate::msg::pbm::GetState) message.
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
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,
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
			horz_resize: Horz::None,
			vert_resize: Vert::None,
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
