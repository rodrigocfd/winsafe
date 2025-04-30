use std::any::Any;
use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{collections::*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Used when adding the parts in
/// [`StatusBar::new`](crate::gui::StatusBar::new).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SbPart {
	/// A part that has a fixed size, in pixels.
	Fixed(i32),
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

struct StatusBarObj {
	base: BaseCtrl,
	events: StatusBarEvents,
	parts_info: UnsafeCell<Vec<SbPart>>,
	right_edges: UnsafeCell<Vec<i32>>, // buffer to speed up resize calls
	_pin: PhantomPinned,
}

native_ctrl! { StatusBar: StatusBarObj => StatusBarEvents;
	/// Native
	/// [status bar](https://learn.microsoft.com/en-us/windows/win32/controls/status-bars)
	/// control, which has one or more parts.
}

impl StatusBar {
	/// Instantiates a new `StatusBar` object, to be created on the parent
	/// window with
	/// [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
	///
	/// # Panics
	///
	/// Panics if the parent window was already created – that is, you cannot
	/// dynamically create a `StatusBar` in an event closure.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, gui};
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	/// # let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
	///
	/// let status_bar = gui::StatusBar::new(
	///     &wnd,
	///     &[
	///         gui::SbPart::Fixed(200),      // 200 pixels, never resizes
	///         gui::SbPart::Proportional(1), // these two will fill the remaning space
	///         gui::SbPart::Proportional(1),
	///     ],
	/// );
	/// ```
	#[must_use]
	pub fn new(parent: &(impl GuiParent + 'static), parts: &[SbPart]) -> Self {
		let ctrl_id = auto_id::next();
		let new_self = Self(Arc::pin(StatusBarObj {
			base: BaseCtrl::new(ctrl_id),
			events: StatusBarEvents::new(parent, ctrl_id),
			parts_info: UnsafeCell::new(parts.to_vec()),
			right_edges: UnsafeCell::new(vec![0; parts.len()]),
			_pin: PhantomPinned,
		}));

		let self2 = new_self.clone();
		let parent2 = parent.clone();
		parent
			.as_ref()
			.before_on()
			.wm(parent.as_ref().wnd_ty().creation_msg(), move |_| {
				let parent_style = parent2.hwnd().style();
				let is_parent_resizable =
					parent_style.has(co::WS::MAXIMIZEBOX) || parent_style.has(co::WS::SIZEBOX);

				self2.0.base.create_window(
					co::WS_EX::LEFT,
					"msctls_statusbar32",
					None,
					co::WS::CHILD
						| co::WS::VISIBLE | co::SBARS::TOOLTIPS.into()
						| if is_parent_resizable {
							co::SBARS::SIZEGRIP
						} else {
							co::SBARS::NoValue
						}
						.into(),
					POINT::default(),
					SIZE::default(),
					&parent2,
				)?;

				// Force first resizing, so the panels are created.
				let parent_rc = parent2.hwnd().GetClientRect()?;
				self2.resize(&mut wm::Size {
					client_area: SIZE::new(parent_rc.right, parent_rc.bottom),
					request: co::SIZE_R::RESTORED,
				})?;

				Ok(0) // ignored
			});

		let self2 = new_self.clone();
		parent.as_ref().before_on().wm_size(move |mut p| {
			self2.resize(&mut p)?;
			Ok(())
		});

		new_self
	}

	fn resize(&self, p: &mut wm::Size) -> SysResult<()> {
		if p.request == co::SIZE_R::MINIMIZED || *self.hwnd() == HWND::NULL {
			return Ok(()); // nothing to do
		}

		unsafe {
			self.hwnd().SendMessage(p.as_generic_wm()); // send WM_SIZE to status bar, so it resizes itself to fit parent
		}

		let mut total_proportions = 0u8;
		let mut cx_available = p.client_area.cx as i32;

		let parts_info = unsafe { &mut *self.0.parts_info.get() };
		for part_info in parts_info.iter() {
			match part_info {
				SbPart::Fixed(pixels) => {
					cx_available -= if *pixels > cx_available { 0 } else { *pixels }; // prevent subtract overflow
				},
				SbPart::Proportional(prop) => total_proportions += prop,
			}
		}

		let right_edges = unsafe { &mut *self.0.right_edges.get() };
		let mut total_cx = p.client_area.cx;

		for (idx, part_info) in parts_info.iter().rev().enumerate() {
			right_edges[parts_info.len() - idx - 1] = total_cx as _;
			let minus = match part_info {
				SbPart::Fixed(pixels) => *pixels,
				SbPart::Proportional(pp) => {
					(cx_available / total_proportions as i32) * (*pp as i32)
				},
			};
			total_cx -= if minus > total_cx { 0 } else { minus }; // prevent subtract overflow
		}
		*right_edges.last_mut().unwrap() = -1;

		unsafe {
			self.hwnd()
				.SendMessage(sb::SetParts { right_edges: &right_edges })
		}
	}

	/// Part methods.
	#[must_use]
	pub const fn parts(&self) -> StatusBarParts<'_> {
		StatusBarParts::new(self)
	}
}
