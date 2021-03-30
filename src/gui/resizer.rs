use std::ptr::NonNull;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::HwndPlace;
use crate::funcs::PostQuitMessage;
use crate::gui::traits::{baseref_from_parent, Child, hwndref_from_child, Parent};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::handles::{HDWP, HWND};
use crate::msg::wm;
use crate::structs::{RECT, SIZE};

/// In [`Resizer::add`](crate::gui::Resizer::add), determines how the child
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

//------------------------------------------------------------------------------

/// When the parent window is resized, automatically adjusts position and size
/// of child controls.
#[derive(Clone)]
pub struct Resizer(Arc<VeryUnsafeCell<Obj>>);

struct Obj { // actual fields of Resizer
	ctrls: Vec<Ctrl>,
	sz_parent_orig: SIZE, // original parent client area
	resize_called: bool,
}

impl Resizer {
	/// Instantiates a new `Resizer`.
	pub fn new(parent: &dyn Parent) -> Resizer {
		let parent_ref = baseref_from_parent(parent);

		let resz = Self(
			Arc::new(VeryUnsafeCell::new(
				Obj {
					ctrls: Vec::with_capacity(16), // arbitrary, prealloc for speed
					sz_parent_orig: SIZE::default(),
					resize_called: false,
				}
			)),
		);

		parent_ref.privileged_events_ref().wm_size({
			let resz = resz.clone();
			move |p| resz.resize(&p).unwrap_or_else(|err| PostQuitMessage(err))
		});

		resz
	}

	/// Registers one or more child controls. Their positions and sizes will be
	/// updated on parent's resizing.
	///
	/// Usually you'll want to call this method on parent's
	/// [`WM_CREATE`](crate::msg::wm::Create) or
	/// [`WM_INITDIALOG`](crate::msg::wm::InitDialog) events, because the
	/// controls will already be created.
	///
	/// # Examples
	///
	/// In the example below, when the parent window is resized, `txtName` and
	/// `btnClick` will move anchored at right and bottom; `btnAnother` will be
	/// only resized vertically.
	///
	/// ```rust,ignore
	/// use winsafe::gui::{Button, Edit, Resizer, Resz};
	///
	/// let resizer: Resizer; // initialize them somewhere...
	/// let txt_name: Edit;
	/// let btn_click: Button;
	/// let txt_another: Edit;
	///
	/// resizer.add(
	///     Resz::Repos, // horizontal
	///     Resz::Repos, // vertical
	///     &[&txt_name, &btn_click],
	/// ).add(
	///     Resz::Nothing,
	///     Resz::Resize,
	///     &[&txt_another],
	/// );
	/// ```
	///
	/// # Panics
	///
	/// Panics if the slice is empty, if the first resizing already happened, or
	/// if a passed control has not been created yet.
	pub fn add(&self,
		horz: Resz, vert: Resz, children: &[&dyn Child]) -> &Resizer
	{
		if children.is_empty() {
			panic!("No children being added to Resizer.");
		} else if self.0.resize_called {
			panic!("Cannot add children after Resizer started working.");
		}

		let first_child_hwnd_ref = hwndref_from_child(children[0]);
		if first_child_hwnd_ref.is_null() {
			panic!("Cannot add a child control to Resizer before it's created.");
		}

		|horz, vert, children: &[&dyn Child]| -> WinResult<&Resizer> {
			let ctrls = &mut self.0.as_mut().ctrls;
			ctrls.reserve(children.len() + children.len());

			let hparent = first_child_hwnd_ref.GetParent()?;
			if ctrls.is_empty() { // first call to add()
				let rc_parent = hparent.GetClientRect()?;
				self.0.as_mut().sz_parent_orig = SIZE::new(rc_parent.right, rc_parent.bottom); // save original parent size
			}

			for child in children.iter() {
				let child_hwnd_ref = hwndref_from_child(*child);
				let mut rc_orig = child_hwnd_ref.GetWindowRect()?;
				hparent.ScreenToClientRc(&mut rc_orig)?; // client coordinates relative to parent

				ctrls.push(Ctrl {
					hwnd_ptr: NonNull::from(child_hwnd_ref), // ref implicitly converted to pointer
					rc_orig,
					horz,
					vert,
				});
			}
			Ok(self)
		}
		(horz, vert, children)
			.unwrap_or_else(|err| { PostQuitMessage(err); self })
	}

	/// Resizes all registered children according to the defined rules.
	fn resize(&self, size_parm: &wm::Size) -> WinResult<()> {
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

		self.0.as_mut().resize_called = true;
		hdwp.EndDeferWindowPos()
	}
}
