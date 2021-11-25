use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::{Handle, HDWP, HWND};
use crate::msg::wm;
use crate::structs::{POINT, RECT, SIZE};

/// Specifies the horizontal behavior of the control when the parent window is
/// resized.
///
/// These values are analog to [`gui::Vert`](crate::gui::Vert).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horz {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at right.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control width will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

/// Specifies the vertical behavior of the control when the parent window is
/// resized.
///
/// These values are analog to [`gui::Horz`](crate::gui::Horz).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vert {
	/// Nothing will be done when parent window is resized.
	None,
	/// When parent window resizes, the control will move anchored at bottom.
	/// Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control height will stretch/shrink
	/// accordingly. Position will remain fixed.
	Resize,
}

struct ChildInfo {
	hchild: HWND,
	rc_orig: RECT, // original coordinates relative to parent
	horz: Horz,
	vert: Vert,
}

//------------------------------------------------------------------------------

/// Resizes all registered child controls at once, according to predefined
/// behaviors.
#[derive(Clone)]
pub(in crate::gui) struct Resizer(Arc<VeryUnsafeCell<Obj>>);

struct Obj { // actual fields of Resizer
	ctrls: Vec<ChildInfo>,
	sz_parent_orig: SIZE, // original parent client area
}

impl Resizer {
	pub(in crate::gui) fn new() -> Self {
		Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					ctrls: Vec::with_capacity(10), // arbitrary
					sz_parent_orig: SIZE::default(),
				},
			)),
		)
	}

	pub(in crate::gui) fn add(&self,
		hparent: HWND,
		hchild: HWND,
		horz: Horz, vert: Vert) -> WinResult<()>
	{
		if hparent.is_null() || hchild.is_null() {
			panic!("Cannot add resizer entries before window/control creation.");
		}

		if horz == Horz::None && vert == Vert::None {
			return Ok(()); // nothing to do, don't even add it
		}

		if self.0.ctrls.is_empty() { // first control being added?
			let rc_parent = hparent.GetClientRect()?;
			self.0.as_mut().sz_parent_orig =
				SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size
		}

		let mut rc_orig = hchild.GetWindowRect()?;
		hparent.ScreenToClientRc(&mut rc_orig)?; // control client coordinates relative to parent

		self.0.as_mut().ctrls.push(ChildInfo { hchild, rc_orig, horz, vert });
		Ok(())
	}

	pub(in crate::gui) fn resize(&self, p: &wm::Size) -> WinResult<()> {
		if self.0.ctrls.is_empty() // no controls
			|| p.request == co::SIZE_R::MINIMIZED { // we're minimized
			return Ok(());
		}

		let hdwp = HDWP::BeginDeferWindowPos(self.0.ctrls.len() as _)?;

		for ctrl in self.0.ctrls.iter() {
			let mut uflags = co::SWP::NOZORDER;
			if ctrl.horz == Horz::Repos && ctrl.vert == Vert::Repos { // reposition both vert & horz
				uflags |= co::SWP::NOSIZE;
			} else if ctrl.horz == Horz::Resize && ctrl.vert == Vert::Resize { // resize both vert & horz
				uflags |= co::SWP::NOMOVE;
			}

			let sz_parent_orig = self.0.sz_parent_orig;

			hdwp.DeferWindowPos(
				ctrl.hchild,
				HwndPlace::None,
				POINT::new(
					match ctrl.horz {
						Horz::Repos => p.client_area.cx - sz_parent_orig.cx + ctrl.rc_orig.left,
						_ => ctrl.rc_orig.left // keep original x pos
					},
					match ctrl.vert {
						Vert::Repos => p.client_area.cy - sz_parent_orig.cy + ctrl.rc_orig.top,
						_ => ctrl.rc_orig.top // keep original y pos
					},
				),
				SIZE::new(
					match ctrl.horz {
						Horz::Resize => p.client_area.cx - sz_parent_orig.cx + ctrl.rc_orig.right - ctrl.rc_orig.left,
						_ => ctrl.rc_orig.right - ctrl.rc_orig.left // keep original width
					},
					match ctrl.vert {
						Vert::Resize => p.client_area.cy - sz_parent_orig.cy + ctrl.rc_orig.bottom - ctrl.rc_orig.top,
						_ =>ctrl.rc_orig.bottom - ctrl.rc_orig.top // keep original height
					},
				),
				uflags,
			)?;
		}

		hdwp.EndDeferWindowPos()
	}
}
