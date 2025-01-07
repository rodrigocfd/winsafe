use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of ProgressBar
	base: BaseNativeControl,
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Native
/// [progress bar](https://learn.microsoft.com/en-us/windows/win32/controls/progress-bar-control)
/// control.
#[derive(Clone)]
pub struct ProgressBar(Pin<Arc<Obj>>);

unsafe impl Send for ProgressBar {}

impl AsRef<BaseNativeControl> for ProgressBar {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

impl GuiWindow for ProgressBar {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for ProgressBar {
	fn ctrl_id(&self) -> u16 {
		self.0.base.ctrl_id()
	}
}

impl GuiNativeControl for ProgressBar {}

impl ProgressBar {
	/// Instantiates a new `ProgressBar` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `ProgressBar` in an event closure.
	#[must_use]
	pub fn new(parent: &impl GuiParent, opts: ProgressBarOpts) -> Self {
		let opts = auto_ctrl_id_if_zero(opts);
		let ctrl_id = opts.ctrl_id;

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.create(OptsResz::Wnd(&opts))?;
			Ok(WmRet::NotHandled)
		});

		new_self
	}

	/// Instantiates a new `ProgressBar` object, to be loaded from a dialog
	/// resource with
	/// [`HWND::GetDlgItem`](crate::prelude::user_Hwnd::GetDlgItem).
	///
	/// # Panics
	///
	/// Panics if the parent dialog was already created – that is, you cannot
	/// dynamically create a `ProgressBar` in an event closure.
	#[must_use]
	pub fn new_dlg(
		parent: &impl GuiParent,
		ctrl_id: u16,
		resize_behavior: (Horz, Vert),
	) -> Self
	{
		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_init_dialog(move |_| {
			self2.create(OptsResz::Dlg(resize_behavior))?;
			Ok(false) // this return value is discarded
		});

		new_self
	}

	fn create(&self, opts_resz: OptsResz<&ProgressBarOpts>) -> SysResult<()> {
		match opts_resz {
			OptsResz::Wnd(opts) => {
				let mut pos = POINT::new(opts.position.0, opts.position.1);
				let mut sz = SIZE::new(opts.size.0 as _, opts.size.1 as _);
				multiply_dpi_or_dtu(
					self.0.base.parent(), Some(&mut pos), Some(&mut sz))?;

				self.0.base.create_window(
					"msctls_progress32", None, pos, sz,
					opts.window_ex_style,
					opts.window_style | opts.progress_bar_style.into(),
				)?;
			},
			OptsResz::Dlg(_) => self.0.base.create_dlg()?,
		}

		self.0.base.parent()
			.add_to_layout_arranger(self.hwnd(), opts_resz.resize_behavior())
	}

	/// Retrieves the current position by sending a
	/// [`pbm::GetPos`](crate::msg::pbm::GetPos) message.
	#[must_use]
	pub fn position(&self) -> u32 {
		unsafe { self.hwnd().SendMessage(pbm::GetPos {}) }
	}

	/// Retrieves the current minimum and maximum values by sending a
	/// [`pbm::GetRange`](crate::msg::pbm::GetRange) message. Default values are
	/// 0 and 100.
	#[must_use]
	pub fn range(&self) -> (u32, u32) {
		// For some reason, pbm::GetRange is returning all zeros when passing a
		// PBRANGE pointer.
		unsafe {
			let low = self.hwnd().SendMessage(pbm::GetRange {
				return_low: true,
				ranges: None,
			});
			let high = self.hwnd().SendMessage(pbm::GetRange {
				return_low: false,
				ranges: None,
			});
			(low as _, high as _)
		}
	}

	/// Sets or unsets the marquee mode by sending a
	/// [`pbm::SetMarquee`](crate::msg::pbm::SetMarquee) message combined with a
	/// [`SetWindowLongPtr`](crate::prelude::user_Hwnd::SetWindowLongPtr) call
	/// for a style change.
	pub fn set_marquee(&self, marquee: bool) {
		if marquee {
			// We must also adjust the window style before/after sending
			// PBM_SETMARQUEE message.
			let cur_style: co::PBS = self.hwnd().style().into();
			self.hwnd().set_style(cur_style | co::PBS::MARQUEE);
		}

		unsafe {
			self.hwnd().SendMessage(pbm::SetMarquee {
				turn_on: marquee,
				time_ms: None,
			});
		}

		if !marquee {
			let cur_style: co::PBS = self.hwnd().style().into();
			self.hwnd().set_style(cur_style & !co::PBS::MARQUEE);
		}
	}

	/// Sets the current position by sending a
	/// [`pbm::SetPos`](crate::msg::pbm::SetPos) message, returning the previous
	/// position.
	pub fn set_position(&self, position: u32) -> u32 {
		let cur_style: co::PBS = self.hwnd().style().into();
		if cur_style.has(co::PBS::MARQUEE) {
			self.set_marquee(false); // avoid crash
		}

		unsafe {
			self.hwnd()
				.SendMessage(pbm::SetPos { position })
		}
	}

	/// Sets the minimum and maximum values by sending a
	/// [`pbm::SetRange32`](crate::msg::pbm::SetRange32) message. Default values
	/// are 0 and 100.
	pub fn set_range(&self, min: u32, max: u32) {
		unsafe {
			self.hwnd()
				.SendMessage(pbm::SetRange32 { min, max })
		}
	}

	/// Sets the current state by sending a
	/// [`pbm::SetState`](crate::msg::pbm::SetState) message, retuning the
	/// previous state.
	pub fn set_state(&self, state: co::PBST) -> co::PBST {
		unsafe {
			self.hwnd()
				.SendMessage(pbm::SetState { state })
		}
	}

	/// Retrieves the current state by sending a
	/// [`pbm::GetState`](crate::msg::pbm::GetState) message.
	#[must_use]
	pub fn state(&self) -> co::PBST {
		unsafe { self.hwnd().SendMessage(pbm::GetState {}) }
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ProgressBar`](crate::gui::ProgressBar)
/// programmatically with [`ProgressBar::new`](crate::gui::ProgressBar::new).
pub struct ProgressBarOpts {
	/// Left and top position coordinates of control within parent's client
	/// area, to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(0, 0)`.
	pub position: (i32, i32),
	/// Width and height of control to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// If the parent window is a dialog, the values are in Dialog Template
	/// Units; otherwise in pixels, which will be multiplied to match current
	/// system DPI.
	///
	/// Defaults to `(120, 23)`.
	pub size: (u32, u32),
	/// Progress bar styles to be
	/// [created](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `PBS::SMOOTH`.
	pub progress_bar_style: co::PBS,
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
	/// Horizontal and vertical behavior of the control when the parent window
	/// is resized.
	///
	/// Defaults to `(gui::Horz::None, gui::Vert::None)`.
	pub resize_behavior: (Horz, Vert),
}

impl Default for ProgressBarOpts {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (120, 23),
			progress_bar_style: co::PBS::SMOOTH,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			resize_behavior: (Horz::None, Vert::None),
		}
	}
}

impl ResizeBehavior for &ProgressBarOpts {
	fn resize_behavior(&self) -> (Horz, Vert) {
		self.resize_behavior
	}
}

impl AutoCtrlId for ProgressBarOpts {
	fn ctrl_id_mut(&mut self) -> &mut u16 {
		&mut self.ctrl_id
	}
}
