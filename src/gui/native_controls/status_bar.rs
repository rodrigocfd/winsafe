use std::any::Any;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::events::{StatusBarEvents, WindowEvents};
use crate::gui::native_controls::base_native_control::BaseNativeControl;
use crate::gui::native_controls::status_bar_parts::StatusBarParts;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi_or_dtu};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::msg::{sb, wm};
use crate::prelude::{
	GuiChild, GuiEvents, GuiNativeControl, GuiNativeControlEvents, GuiParent,
	GuiWindow, Handle, MsgSend, NativeBitflag, user_Hwnd,
};
use crate::user::decl::{HWND, POINT, SIZE};

struct Obj { // actual fields of StatusBar
	base: BaseNativeControl,
	ctrl_id: u16,
	events: StatusBarEvents,
	parts_info: VeryUnsafeCell<Vec<StatusBarPart>>,
	right_edges: VeryUnsafeCell<Vec<i32>>, // buffer to speed up resize calls
	_pin: PhantomPinned,
}

/// Used when adding the parts in
/// [`StatusBar::new`](crate::gui::StatusBar::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone, Copy)]
pub enum StatusBarPart {
	/// A part that has a fixed size, in pixels.
	///
	/// Will be adjusted to match current system DPI.
	Fixed(u32),
	/// A part that will resize when the parent window resizes, filling the
	/// space left by the fixed-size parts. Has the resizing proportion.
	///
	/// How proportion works:
	///
	/// 1. Suppose you have 3 parts, respectively with proportions of 1, 1 and 2.
	/// 2. If available client area width is 400px, respective part widths will be 100, 100 and 200px.
	/// 3. If parent is resized to have a client area of 600px, parts will then have 200, 200 and 400px.
	///
	/// If you're uncertain, just give all resizable parts the proportion 1.
	Proportional(u8),
}

//------------------------------------------------------------------------------

/// Native
/// [status bar](https://learn.microsoft.com/en-us/windows/win32/controls/status-bars)
/// control, which has one or more parts.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
#[derive(Clone)]
pub struct StatusBar(Pin<Arc<Obj>>);

unsafe impl Send for StatusBar {}

impl GuiWindow for StatusBar {
	fn hwnd(&self) -> &HWND {
		self.0.base.hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiChild for StatusBar {
	fn ctrl_id(&self) -> u16 {
		self.0.ctrl_id
	}
}

impl GuiNativeControl for StatusBar {
	fn on_subclass(&self) -> &WindowEvents {
		self.0.base.on_subclass()
	}
}

impl GuiNativeControlEvents<StatusBarEvents> for StatusBar {
	fn on(&self) -> &StatusBarEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add events after the control creation.");
		} else if *self.0.base.parent().hwnd() != HWND::NULL {
			panic!("Cannot add events after the parent window creation.");
		}
		&self.0.events
	}
}

impl StatusBar {
	/// Instantiates a new `StatusBar` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let status_bar = gui::StatusBar::new(
	///     &wnd,
	///     &[
	///         gui::StatusBarPart::Fixed(200),      // 200 pixels, never resizes
	///         gui::StatusBarPart::Proportional(1), // these two will fill the remaning space
	///         gui::StatusBarPart::Proportional(1),
	///     ],
	/// );
	/// ```
	#[must_use]
	pub fn new(
		parent: &impl GuiParent,
		parts: &[StatusBarPart]) -> StatusBar
	{
		let parent_ref = unsafe { Base::from_guiparent(parent) };
		let ctrl_id = auto_ctrl_id();

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent_ref),
					ctrl_id,
					events: StatusBarEvents::new(parent_ref, ctrl_id),
					parts_info: VeryUnsafeCell::new(parts.to_vec()),
					right_edges: VeryUnsafeCell::new(vec![0; parts.len()]),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm(parent_ref.creation_msg(), move |_| {
			self2.create();
			Ok(None) // not meaningful
		});

		let self2 = new_self.clone();
		parent_ref.privileged_on().wm_size(move |p| {
			let mut p = p;
			self2.resize(&mut p);
			Ok(())
		});

		new_self
	}

	fn create(&self) {
		for part in self.0.parts_info.as_mut().iter_mut() {
			if let StatusBarPart::Fixed(width) = part { // adjust fixed-width parts to DPI
				let mut col_cx = SIZE::new(*width as _, 0);
				multiply_dpi_or_dtu(self.0.base.parent(), None, Some(&mut col_cx));
				*width = col_cx.cx as _;
			}
		}

		let hparent = self.0.base.parent().hwnd();
		let parent_style = co::WS(
			hparent.GetWindowLongPtr(co::GWLP::STYLE) as _,
		);
		let is_parent_resizable = parent_style.has(co::WS::MAXIMIZEBOX)
			|| parent_style.has(co::WS::SIZEBOX);

		self.0.base.create_window( // may panic
			"msctls_statusbar32", None,
			POINT::default(), SIZE::default(),
			self.0.ctrl_id,
			co::WS_EX::LEFT,
			co::WS::CHILD | co::WS::VISIBLE | co::SBARS::TOOLTIPS.into() |
				if is_parent_resizable {
					co::SBARS::SIZEGRIP
				} else {
					co::SBARS::NoValue
				}.into(),
		);

		// Force first resizing, so the panels are created.
		let parent_rc = hparent.GetClientRect().unwrap();
		self.resize(&mut wm::Size {
			client_area: SIZE::new(parent_rc.right, parent_rc.bottom),
			request: co::SIZE_R::RESTORED,
		});
	}

	fn resize(&self, p: &mut wm::Size) {
		if p.request == co::SIZE_R::MINIMIZED || *self.hwnd() == HWND::NULL {
			return; // nothing to do
		}

		self.hwnd().SendMessage(p.as_generic_wm()); // send WM_SIZE to status bar, so it resizes itself to fit parent

		let mut total_proportions: u8 = 0;
		let mut cx_available = p.client_area.cx as u32;

		for part_info in self.0.parts_info.iter() {
			match part_info {
				StatusBarPart::Fixed(pixels) => cx_available -= pixels,
				StatusBarPart::Proportional(prop) => total_proportions += prop,
			}
		}

		let right_edges = &mut self.0.right_edges.as_mut();
		let mut total_cx = p.client_area.cx as u32;

		for (idx, part_info) in self.0.parts_info.iter().rev().enumerate() {
			right_edges[self.0.parts_info.len() - idx - 1] = total_cx as _;
			total_cx -= match part_info {
				StatusBarPart::Fixed(pixels) => *pixels,
				StatusBarPart::Proportional(pp) => (cx_available / total_proportions as u32) * (*pp as u32),
			};
		}
		*right_edges.last_mut().unwrap() = -1;

		self.hwnd()
			.SendMessage(sb::SetParts { right_edges: &right_edges })
			.unwrap();
	}

	/// Exposes the part methods.
	#[must_use]
	pub const fn parts(&self) -> StatusBarParts {
		StatusBarParts::new(self)
	}
}
