use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::msg::wm;
use crate::prelude::{Handle, user_Hdwp, user_Hwnd};
use crate::user::decl::{HDWP, HWND, HwndPlace, POINT, RECT, SIZE};

/// Specifies the horizontal behavior of the control when the parent window is
/// resized.
///
/// The values are analog to [`gui::Vert`](crate::gui::Vert).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
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
/// The values are analog to [`gui::Horz`](crate::gui::Horz).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
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

struct Obj { // actual fields of LayoutArranger
	ctrls: VeryUnsafeCell<Vec<ChildInfo>>,
	sz_parent_orig: VeryUnsafeCell<SIZE>, // original parent client area
	_pin: PhantomPinned,
}

//------------------------------------------------------------------------------

/// Rearranges the controls according to predefined rules.
#[derive(Clone)]
pub(in crate::gui) struct LayoutArranger(Pin<Arc<Obj>>);

impl LayoutArranger {
	pub(in crate::gui) fn new() -> Self {
		Self(
			Arc::pin(
				Obj {
					ctrls: VeryUnsafeCell::new(Vec::with_capacity(10)), // arbitrary
					sz_parent_orig: VeryUnsafeCell::new(SIZE::default()),
					_pin: PhantomPinned,
				},
			),
		)
	}

	/// Adds a new child control to the internal list, so this control will have
	/// its position and size rearranged when requested.
	pub(in crate::gui) fn add(&self,
		hparent: &HWND, hchild: &HWND, horz: Horz, vert: Vert)
	{
		if *hparent == HWND::NULL || *hchild == HWND::NULL {
			panic!("Cannot add resizer entries before window/control creation.");
		}

		if horz == Horz::None && vert == Vert::None {
			return; // nothing to do, don't even add it
		}

		if self.0.ctrls.is_empty() { // first control being added?
			let rc_parent = hparent.GetClientRect().unwrap();
			*self.0.sz_parent_orig.as_mut() =
				SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size
		}

		let mut rc_orig = hchild.GetWindowRect().unwrap();
		hparent.ScreenToClientRc(&mut rc_orig).unwrap(); // control client coordinates relative to parent

		self.0.ctrls.as_mut().push(
			ChildInfo {
				hchild: unsafe { hchild.raw_copy() },
				rc_orig,
				horz,
				vert,
			},
		);
	}

	/// Rearranges all child controls to fit the new width/height of parent
	/// window.
	pub(in crate::gui) fn rearrange(&self, p: &wm::Size) {
		if self.0.ctrls.is_empty() // no controls
			|| p.request == co::SIZE_R::MINIMIZED { // we're minimized
			return;
		}

		let hdwp = HDWP::BeginDeferWindowPos(self.0.ctrls.len() as _).unwrap();

		for ctrl in self.0.ctrls.iter() {
			let mut uflags = co::SWP::NOZORDER;
			if ctrl.horz == Horz::Repos && ctrl.vert == Vert::Repos { // reposition both vert & horz
				uflags |= co::SWP::NOSIZE;
			} else if ctrl.horz == Horz::Resize && ctrl.vert == Vert::Resize { // resize both vert & horz
				uflags |= co::SWP::NOMOVE;
			}

			let sz_parent_orig = *self.0.sz_parent_orig;

			hdwp.DeferWindowPos(
				&ctrl.hchild,
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
			).unwrap();
		}

		hdwp.EndDeferWindowPos().unwrap();
	}
}
