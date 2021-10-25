use std::marker::PhantomData;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::{EventsView, StatusBarEvents};
use crate::gui::native_controls::base_native_control::BaseNativeControl;
use crate::gui::native_controls::status_bar_parts::StatusBarParts;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{baseref_from_parent, Child, Parent, Window};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::msg::{MsgSend, sb, wm};
use crate::structs::{POINT, SIZE};

struct Obj { // actual fields of StatusBar
	base: BaseNativeControl,
	ctrl_id: u16,
	events: StatusBarEvents,
	parts_info: VeryUnsafeCell<Vec<StatusBarPart>>,
	right_edges: VeryUnsafeCell<Vec<i32>>, // buffer to speed up resize calls
}

impl_obj_window!(Obj);

impl Child for Obj {
	fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}
}

impl_obj_nativecontrol!(Obj);

//------------------------------------------------------------------------------

/// Used when adding the parts in
/// [`StatusBar::new`](crate::gui::StatusBar::new).
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
/// [status bar](https://docs.microsoft.com/en-us/windows/win32/controls/status-bars)
/// control, which has one or more parts.
#[derive(Clone)]
pub struct StatusBar(Arc<Obj>);

impl_send_sync!(StatusBar);
impl_debug!(StatusBar);

impl_window!(StatusBar);
impl_child!(StatusBar);
impl_nativecontrol!(StatusBar);
impl_asnativecontrol!(StatusBar);
impl_nativecontrolevents!(StatusBar, StatusBarEvents);

impl StatusBar {
	/// Instantiates a new `StatusBar` object, to be created on the parent
	/// window with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::gui;
	///
	/// let wnd: gui::WindowMain; // initialized somewhere
	///
	/// let status_bar = gui::StatusBar::new(&[
	///     gui::StatusBarPart::Fixed(200),      // 200 pixels, never resizes
	///     gui::StatusBarPart::Proportional(1), // these two will fill the remaning space
	///     gui::StatusBarPart::Proportional(1),
	/// ]);
	/// ```
	pub fn new(parent: &impl Parent, parts: &[StatusBarPart]) -> StatusBar {
		let parent_base_ref = baseref_from_parent(parent);
		let ctrl_id = auto_ctrl_id();

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					ctrl_id,
					events: StatusBarEvents::new(parent_base_ref, ctrl_id),
					parts_info: VeryUnsafeCell::new(parts.to_vec()),
					right_edges: VeryUnsafeCell::new(vec![0; parts.len()]),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.create_or_initdlg(), {
			let self2 = new_self.clone();
			move |_| { self2.create()?; Ok(0) }
		});
		parent_base_ref.privileged_events_ref().wm_size({
			let self2 = new_self.clone();
			move |p| { self2.resize(&p)?; Ok(()) }
		});

		new_self
	}

	fn create(&self) -> WinResult<()> {
		for part in self.0.parts_info.as_mut().iter_mut() {
			if let StatusBarPart::Fixed(width) = part { // adjust fixed-width parts to DPI
				let mut col_cx = SIZE::new(*width as _, 0);
				multiply_dpi(None, Some(&mut col_cx))?;
				*width = col_cx.cx as _;
			}
		}

		let hparent = *self.0.base.parent_base_ref().hwnd_ref();
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
		)?;

		// Force first resizing, so the panels are created.
		let parent_rc = hparent.GetClientRect()?;
		self.resize(&wm::Size {
			client_area: SIZE::new(parent_rc.right, parent_rc.bottom),
			request: co::SIZE_R::RESTORED,
		})?;

		Ok(())
	}

	fn resize(&self, p: &wm::Size) -> WinResult<()> {
		if p.request == co::SIZE_R::MINIMIZED || self.hwnd().is_null() {
			return Ok(()); // nothing to do
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

		self.hwnd().SendMessage(sb::SetParts { right_edges: &right_edges })
	}

	/// Exposes the part methods.
	pub fn parts<'a>(&'a self) -> StatusBarParts<'a> {
		StatusBarParts {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}
}
