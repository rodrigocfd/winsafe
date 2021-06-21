use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::traits::{baseref_from_parent, Child, hwndref_from_child, Parent};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::{HDWP, HWND};
use crate::msg::wm;
use crate::structs::{RECT, SIZE};

/// In [`Resizer::new`](crate::gui::Resizer::new), determines how the child
/// controls will be adjusted automatically when the parent window is resized.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Resz {
	/// Nothing will be done when parent window is resized.
	Nothing,
	/// When parent window resizes, the control will move anchored at
	/// right/bottom. Size of the control will remain fixed.
	Repos,
	/// When parent window resizes, the control will be resized. Position will
	/// remain fixed.
	Resize,
}

struct Ctrl {
	hwnd_ptr: NonNull<HWND>,
	rc_orig: RECT, // original coordinates relative to parent
	horz: Resz,
	vert: Resz,
}

struct ChildEntry { // parameters passed to constructor
	horz: Resz,
	vert: Resz,
	children: Vec<NonNull<HWND>>,
}

//------------------------------------------------------------------------------

/// When the parent window is resized, automatically adjusts position and size
/// of child controls.
#[derive(Clone)]
pub struct Resizer(Arc<VeryUnsafeCell<Obj>>);

struct Obj { // actual fields of Resizer
	ctrls: Vec<Ctrl>,
	sz_parent_orig: SIZE, // original parent client area
}

impl Resizer {
	/// Instantiates a new `Resizer`, receiving the controls along with their
	/// horizontal and vertical behaviors when the owner window is resized.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::gui::{Button, Edit, Resizer, Resz, WindowMain};
	///
	/// let wnd: WindowMain; // initialized somewhere
	/// let txt_name: Edit;
	/// let txt_surname: Edit;
	/// let txt_another: Edit;
	/// let btn_click: Button;
	///
	/// let layout_resizer = Resizer::new(&wnd, &[
	///
	///     // The first parameter of the tuple is the horizontal behavior,
	///     // the second one is the vertical behavior,
	///     // and the third specifies all controls to which the behaviors apply.
	///     //
	///     // Horizontally: stretch or shrink.
	///     // Vertically: don't move or stretch/shrink; do nothing.
	///     (Resz::Resize, Resz::Nothing, &[&txt_name, &txt_surname]),
	///
	///     // Horizontally: don't move or stretch/shrink; do nothing.
	///     // Vertically: move up or down.
	///     (Resz::Resize, Resz::Repos, &[&txt_another]),
	///
	///     // Horizontally: move left or right.
	///     // Vertically: move up or down.
	///     (Resz::Repos, Resz::Repos, &[&btn_click]),
	/// ]);
	/// ```
	///
	/// # Panics
	///
	/// Panics if no child controls are passed.
	pub fn new(
		parent: &dyn Parent, children: &[(Resz, Resz, &[&dyn Child])]) -> Resizer
	{
		if children.is_empty() {
			panic!("Cannot create a Resizer without child controls.");
		}

		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					ctrls: Vec::with_capacity(16), // arbitrary, prealloc for speed
					sz_parent_orig: SIZE::default(),
				},
			)),
		);

		let ptr_parent = NonNull::from(parent_base_ref);
		let rc_children = Rc::new(
			children.iter().map(|c|
				ChildEntry {
					horz: c.0,
					vert: c.1,
					children: c.2.iter()
						.map(|dyn_child| NonNull::from(hwndref_from_child(*dyn_child)))
						.collect(),
				},
			).collect::<Vec<_>>()
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.creation_wm(), {
			let me = new_self.clone();
			move |_| { me.add_children(ptr_parent, rc_children.clone()); 0 }
		});
		parent_base_ref.privileged_events_ref().wm_size({
			let me = new_self.clone();
			move |p| me.resize(&p).unwrap_or_else(|err| PostQuitMessage(err))
		});

		new_self
	}

	fn add_children(&self,
		ptr_parent: NonNull<Base>, rc_children: Rc<Vec<ChildEntry>>)
	{
		let ctrls = &mut self.0.as_mut().ctrls;
		ctrls.reserve(rc_children.len());

		|ptr_parent: NonNull<Base>, rc_children: Rc<Vec<ChildEntry>>| -> WinResult<()> {
			let parent_base_ref = unsafe { ptr_parent.as_ref() };
			let rc_parent = parent_base_ref.hwnd_ref().GetClientRect()?;
			self.0.as_mut().sz_parent_orig = SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size

			for entry in rc_children.iter() {
				for child_hwnd_ptr in entry.children.iter() {
					let child_hwnd_ref = unsafe { child_hwnd_ptr.as_ref() };
					let mut rc_orig = child_hwnd_ref.GetWindowRect()?;
					parent_base_ref.hwnd_ref().ScreenToClientRc(&mut rc_orig)?; // control client coordinates relative to parent

					ctrls.push(Ctrl {
						hwnd_ptr: *child_hwnd_ptr,
						rc_orig,
						horz: entry.horz,
						vert: entry.vert,
					});
				}
			}

			Ok(())
		}
		(ptr_parent, rc_children)
			.unwrap_or_else(|err| { PostQuitMessage(err); });
	}

	/// Resizes all registered children according to the defined rules.
	fn resize(&self, size_parm: &wm::Size) -> WinResult<()> {
		if self.0.ctrls.is_empty() || size_parm.request == co::SIZE_R::MINIMIZED {
			return Ok(()); // if no controls, or if minimized, no need to process
		}

		let hdwp = HDWP::BeginDeferWindowPos(self.0.ctrls.len() as _)?;

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
				unsafe { *ctrl.hwnd_ptr.as_ref() },
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
				},
				match ctrl.vert {
					Resz::Resize => parent_cy - self.0.sz_parent_orig.cy + ctrl.rc_orig.bottom - ctrl.rc_orig.top,
					_ =>ctrl.rc_orig.bottom - ctrl.rc_orig.top // keep original height
				},
				uflags,
			)?;
		}

		hdwp.EndDeferWindowPos()
	}
}
