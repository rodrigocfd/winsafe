use std::any::Any;
use std::cell::UnsafeCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*, spec::*};
use crate::msg::*;
use crate::prelude::*;

struct Obj { // actual fields of StatusBar
	base: BaseNativeControl,
	events: StatusBarEvents,
	parts_info: UnsafeCell<Vec<SbPart>>,
	right_edges: UnsafeCell<Vec<i32>>, // buffer to speed up resize calls
	_pin: PhantomPinned,
}

/// Native
/// [status bar](https://learn.microsoft.com/en-us/windows/win32/controls/status-bars)
/// control, which has one or more parts.
#[derive(Clone)]
pub struct StatusBar(Pin<Arc<Obj>>);

unsafe impl Send for StatusBar {}

impl AsRef<BaseNativeControl> for StatusBar {
	fn as_ref(&self) -> &BaseNativeControl {
		&self.0.base
	}
}

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
		self.0.base.ctrl_id()
	}
}

impl GuiNativeControl for StatusBar {}

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
	pub fn new(parent: &impl GuiParent, parts: &[SbPart]) -> Self {
		let ctrl_id = next_auto_ctrl_id();

		let new_self = Self(
			Arc::pin(
				Obj {
					base: BaseNativeControl::new(parent, ctrl_id),
					events: StatusBarEvents::new(parent, ctrl_id),
					parts_info: UnsafeCell::new(parts.to_vec()),
					right_edges: UnsafeCell::new(vec![0; parts.len()]),
					_pin: PhantomPinned,
				},
			),
		);

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_create_or_initdialog(move |_, _| {
			self2.create()?;
			Ok(WmRet::NotHandled)
		});

		let self2 = new_self.clone();
		parent.as_ref().before_user_on().wm_size(move |mut p| {
			self2.resize(&mut p);
			Ok(())
		});

		new_self
	}

	fn create(&self) -> SysResult<()> {
		let parts_info = unsafe { &mut *self.0.parts_info.get() };
		for part in parts_info.iter_mut() {
			if let SbPart::Fixed(width) = part { // adjust fixed-width parts to DPI
				let mut col_cx = SIZE::new(*width as _, 0);
				multiply_dpi_or_dtu(self.0.base.parent(), None, Some(&mut col_cx))?;
				*width = col_cx.cx as _;
			}
		}

		let hparent = self.0.base.parent().hwnd();
		let parent_style = hparent.style();
		let is_parent_resizable = parent_style.has(co::WS::MAXIMIZEBOX)
			|| parent_style.has(co::WS::SIZEBOX);

		self.0.base.create_window( // may panic
			"msctls_statusbar32", None,
			POINT::default(), SIZE::default(),
			co::WS_EX::LEFT,
			co::WS::CHILD | co::WS::VISIBLE | co::SBARS::TOOLTIPS.into() |
				if is_parent_resizable {
					co::SBARS::SIZEGRIP
				} else {
					co::SBARS::NoValue
				}.into(),
		)?;

		// Force first resizing, so the panels are created.
		let parent_rc = hparent.GetClientRect()?;
		self.resize(&mut wm::Size {
			client_area: SIZE::new(parent_rc.right, parent_rc.bottom),
			request: co::SIZE_R::RESTORED,
		});

		Ok(())
	}

	fn resize(&self, p: &mut wm::Size) {
		if p.request == co::SIZE_R::MINIMIZED || *self.hwnd() == HWND::NULL {
			return; // nothing to do
		}

		unsafe { self.hwnd().SendMessage(p.as_generic_wm()); } // send WM_SIZE to status bar, so it resizes itself to fit parent

		let mut total_proportions: u8 = 0;
		let mut cx_available = p.client_area.cx as u32;

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
		let mut total_cx = p.client_area.cx as u32;

		for (idx, part_info) in parts_info.iter().rev().enumerate() {
			right_edges[parts_info.len() - idx - 1] = total_cx as _;
			let minus = match part_info {
				SbPart::Fixed(pixels) => *pixels,
				SbPart::Proportional(pp) =>
					(cx_available / total_proportions as u32) * (*pp as u32),
			};
			total_cx -= if minus > total_cx { 0 } else { minus }; // prevent subtract overflow
		}
		*right_edges.last_mut().unwrap() = -1;

		unsafe {
			self.hwnd()
				.SendMessage(sb::SetParts { right_edges: &right_edges })
		}.unwrap();
	}

	/// Exposes the part methods.
	#[must_use]
	pub const fn parts(&self) -> StatusBarParts<'_> {
		StatusBarParts::new(self)
	}
}
