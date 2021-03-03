use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::funcs::PostQuitMessage;
use crate::gui::events::{StatusBarEvents, WindowEvents};
use crate::gui::immut::Immut;
use crate::gui::native_controls::native_control_base::NativeControlBase;
use crate::gui::privs::{auto_ctrl_id, multiply_dpi};
use crate::gui::traits::{Child, Parent};
use crate::handles::HWND;
use crate::msg::{Message, sb, wm};
use crate::structs::{POINT, SIZE};
use crate::WString;

/// Native
/// [status bar](https://docs.microsoft.com/en-us/windows/win32/controls/status-bars)
/// control, which has one or more parts.
#[derive(Clone)]
pub struct StatusBar(Arc<Immut<Obj>>);

struct Obj { // actual fields of StatusBar
	base: NativeControlBase<StatusBarEvents>,
	ctrl_id: u16,
	parts: Vec<StatusBarPart>,
	right_edges: Vec<i32>, // buffer to speed up resize calls
}

unsafe impl Send for StatusBar {}
unsafe impl Sync for StatusBar {}

impl Child for StatusBar {
	fn hctrl_ref(&self) -> &HWND {
		self.0.base.hctrl_ref()
	}
}

impl StatusBar {
	/// Instantiates a new `StatusBar` object, to be created on the parent window
	/// with [`CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &dyn Parent, parts: &[StatusBarPart]) -> StatusBar {
		let ctrl_id = auto_ctrl_id();
		let new_self = Self(
			Arc::new(Immut::new(
				Obj {
					base: NativeControlBase::new(
						parent,
						StatusBarEvents::new(parent, ctrl_id),
					),
					ctrl_id,
					parts: parts.to_vec(),
					right_edges: vec![0; parts.len()],
				},
			)),
		);
		parent.privileged_events_ref().wm_create({
			let me = new_self.clone();
			move |_| { me.create(); 0 }
		});
		parent.privileged_events_ref().wm_size({
			let me = new_self.clone();
			move |p| me.resize(&p)
		});
		new_self
	}

	fn create(&self) {
		|| -> WinResult<()> {
			for part in self.0.as_mut().parts.iter_mut() {
				if let StatusBarPart::Fixed(width) = part {
					let mut col_cx = SIZE::new(*width as i32, 0);
					multiply_dpi(None, Some(&mut col_cx))?;
					*width = col_cx.cx as u32;
				}
			}

			let parent_style = co::WS(
				self.0.base.parent_hwnd().GetWindowLongPtr(co::GWLP::STYLE) as u32,
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
						co::SBARS::NONE
					}.into(),
			)?;

			// Force first resizing, so the panels are created.
			let parent_rc = self.0.base.parent_hwnd().GetClientRect()?;
			self.resize(&wm::Size {
				client_area: SIZE::new(parent_rc.right, parent_rc.bottom),
				request: co::SIZE_R::RESTORED,
			});

			Ok(())
		}
		().unwrap_or_else(|err| PostQuitMessage(err))
	}

	fn resize(&self, p: &wm::Size) {
		|p: &wm::Size| -> WinResult<()> {
			if p.request == co::SIZE_R::MINIMIZED || self.hwnd().is_null() {
				return Ok(()); // nothing to do
			}

			self.hwnd().SendMessage(p.as_generic_wm()); // tell status bar to fit parent

			let mut total_proportions: u8 = 0;
			let mut cx_available = p.client_area.cx as u32;

			for part in self.0.parts.iter() {
				match part {
					StatusBarPart::Fixed(pixels) => cx_available -= pixels,
					StatusBarPart::Proportional(prop) => total_proportions += prop,
				}
			}

			let right_edges = &mut self.0.as_mut().right_edges;
			let mut total_cx = p.client_area.cx as u32;

			for (idx, part) in self.0.parts.iter().rev().enumerate() {
				right_edges[self.0.parts.len() - idx - 1] = total_cx as i32;
				total_cx -= match part {
					StatusBarPart::Fixed(pixels) => *pixels,
					StatusBarPart::Proportional(pp) => (cx_available / total_proportions as u32) * (*pp as u32),
				};
			}
			*right_edges.last_mut().unwrap() = -1;

			self.hwnd().SendMessage(sb::SetParts { right_edges: &right_edges })
		}
		(p).unwrap_or_else(|err| PostQuitMessage(err))
	}

	hwnd_on_onsubclass!(StatusBarEvents);

	/// Retrieves the number of parts.
	pub fn part_count(&self) -> u8 {
		self.hwnd().SendMessage(sb::GetParts { right_edges: None })
	}

	/// Retrieves the text or a part.
	pub fn part_text(&self, part_index: u8) -> String {
		let (len, _) = self.hwnd().SendMessage(sb::GetTextLength { part_index });
		let mut buf = WString::new_alloc_buffer(len as usize + 1);

		self.hwnd().SendMessage(sb::GetText {
			part_index,
			text: &mut buf,
		});
		buf.to_string()
	}

	/// Sets the text of a part.
	pub fn set_part_text(&self, part_index: u8, text: &str) -> WinResult<()> {
		self.hwnd().SendMessage(sb::SetText {
			part_index,
			drawing_operation: co::SBT::NONE,
			text,
		})
	}
}

//------------------------------------------------------------------------------

/// Used when adding the parts in [`StatusBar::new`](crate::gui::StatusBar::new).
#[derive(Clone, Copy)]
pub enum StatusBarPart {
	/// A part that has a fixed size, in pixels.
	//
	/// Will be adjusted to match current system DPI.
	Fixed(u32),
	/// A part that will resize when the parent window resizes, filling the space
	/// left by the fixed-size parts. Has the resizing proportion.
	///
	/// How proportion works:
	///
	/// 1. Suppose you have 3 parts, respectively with proportions of 1, 1 and 2.
	/// 2. If available client area width is 400px, respective part widths will
	/// be 100, 100 and 200px.
	/// 3. If parent is resized to have a client area of 600px, parts will then
	/// have 200, 200 and 400px.
	///
	/// If you're uncertain, just give all resizable parts the proportion 1.
	Proportional(u8),
}
