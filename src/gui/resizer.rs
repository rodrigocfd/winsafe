use std::ptr::NonNull;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::gui::immut::Immut;
use crate::gui::traits::{Child, Parent};
use crate::handles::{HDWP, HWND};
use crate::msg::WmSize;
use crate::structs::{RECT, SIZE};

/// In [`Resizer::add`](crate::gui::Resizer::add), determines how the child
/// controls will be adjusted automatically when the parent window is resized.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Resz {
	/// Nothing will be done.
	Nothing,
	/// Size is fixed, control will move anchored at right/bottom.
	Repos,
	/// Position is fixed, control will be resized.
	Resize,
}

struct Ctrl {
	hctrl: NonNull<HWND>,
	rc_orig: RECT, // original coordinates relative to parent
	horz: Resz,
	vert: Resz,
}

//------------------------------------------------------------------------------

/// When the parent window is resized, automatically adjusts position and size
/// of child controls.
#[derive(Clone)]
pub struct Resizer(Arc<Immut<Obj>>);

struct Obj { // actual fields of Resizer
	ctrls: Vec<Ctrl>,
	sz_parent_orig: SIZE, // original parent client area
	resize_called: bool,
}

impl Resizer {
	/// Instantiates a new `Resizer`.
	pub fn new(parent: &dyn Parent) -> Resizer {
		let resz = Self(
			Arc::new(Immut::new(
				Obj {
					ctrls: Vec::with_capacity(16), // arbitrary, prealloc for speed
					sz_parent_orig: SIZE::default(),
					resize_called: false,
				}
			)),
		);
		parent.privileged_events_ref().wm_size({
			let resz = resz.clone();
			move |p| { resz.resize(&p).unwrap(); }
		});
		resz
	}

	/// Registers one or more child controls. They will be resized in every
	/// [`resize`](crate::gui::Resizer::resize) call.
	///
	/// # Panics
	///
	/// Panics if the slice is empty, or if
	/// [`resize`](crate::gui::Resizer::resize) has already been called.
	pub fn add(&self,
		horz: Resz, vert: Resz, children: &[&dyn Child]) -> &Resizer
	{
		if children.is_empty() {
			panic!("No children being added to Resizer.");
		} else if self.0.resize_called {
			panic!("Cannot add children after Resizer started working.");
		}

		let ctrls = &mut self.0.as_mut().ctrls;
		ctrls.reserve(children.len() + children.len());

		let hparent = children[0].hctrl_ref().GetParent().unwrap();
		if ctrls.is_empty() { // first call to add()
			let rc_parent = hparent.GetClientRect().unwrap();
			self.0.as_mut().sz_parent_orig = SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size
		}

		for child in children.iter() {
			let mut rc_orig = child.hctrl_ref().GetWindowRect().unwrap();
			hparent.ScreenToClientRc(&mut rc_orig).unwrap(); // client coordinates relative to parent

			ctrls.push(Ctrl {
				hctrl: NonNull::from(child.hctrl_ref()), // ref implicitly converted to pointer
				rc_orig,
				horz,
				vert,
			});
		}
		self
	}

	fn resize(&self, size_parm: &WmSize) -> WinResult<()> {
		if self.0.ctrls.is_empty() || size_parm.request == co::SIZE_R::MINIMIZED {
			return Ok(()); // if no controls, or if minimized, no need to process
		}

		let hdwp = HDWP::BeginDeferWindowPos(self.0.ctrls.len() as u32)?;

		let parent_cx = size_parm.client_area.cx;
		let parent_cy = size_parm.client_area.cy;

		for ctrl in self.0.ctrls.iter() {
			let mut uflags = co::SWP::NOZORDER;
			if ctrl.horz == Resz::Repos && ctrl.vert == Resz::Repos { // reposition both vert & horz
				uflags |= co::SWP::NOSIZE;
			} else if ctrl.horz == Resz::Resize && ctrl.vert == Resz::Resize { // resize both vert & horz
				uflags |= co::SWP::NOMOVE;
			}

			hdwp.DeferWindowPos(
				unsafe { *ctrl.hctrl.as_ref() },
				HwndPlace::None,
				match ctrl.horz {
					Resz::Repos => parent_cx - self.0.sz_parent_orig.cx + ctrl.rc_orig.left,
					_ => ctrl.rc_orig.left // keep original x pos
				},
				match ctrl.vert {
					Resz::Repos => parent_cy - self.0.sz_parent_orig.cy + ctrl.rc_orig.top,
					_ => ctrl.rc_orig.top // keep original y pos
				},
				match ctrl.horz {
					Resz::Resize => parent_cx - self.0.sz_parent_orig.cx + ctrl.rc_orig.right - ctrl.rc_orig.left,
					_ => ctrl.rc_orig.right - ctrl.rc_orig.left // keep original width
				} as u32,
				match ctrl.vert {
					Resz::Resize => parent_cy - self.0.sz_parent_orig.cy + ctrl.rc_orig.bottom - ctrl.rc_orig.top,
					_ =>ctrl.rc_orig.bottom - ctrl.rc_orig.top // keep original height
				} as u32,
				uflags,
			)?;
		}

		hdwp.EndDeferWindowPos()
	}
}
