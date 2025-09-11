use std::ptr::NonNull;
use std::rc::Rc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

enum CtrlIds {
	One(u16),       // common case
	Many(Vec<u16>), // RadioGroup, which has many radio buttons
}

/// Implements all control event traits.
///
/// Supports one or multiple controls because of RadioGroup.
pub(in crate::gui) struct BaseCtrlEvents {
	parent_ptr: NonNull<BaseWnd>, // used only to add the events to parent, before the first message is processed
	wnd_ty: WndTy,
	ctrl_ids: CtrlIds,
}

impl BaseCtrlEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<BaseWnd>, ctrl_id: u16) -> Self {
		Self {
			parent_ptr: NonNull::from_ref(parent.as_ref()),
			wnd_ty: parent.as_ref().wnd_ty(),
			ctrl_ids: CtrlIds::One(ctrl_id),
		}
	}

	#[must_use]
	pub(in crate::gui) fn new_many(parent: &impl AsRef<BaseWnd>, ctrl_ids: Vec<u16>) -> Self {
		Self {
			parent_ptr: NonNull::from_ref(parent.as_ref()),
			ctrl_ids: CtrlIds::Many(ctrl_ids),
			wnd_ty: parent.as_ref().wnd_ty(),
		}
	}
}

pub(in crate::gui) mod priv_ctrl_events {
	use crate::co;
	use crate::decl::*;
	use crate::gui::privs::*;
	use crate::msg::*;

	pub trait GuiEvents {
		#[allow(private_interfaces)]
		#[must_use]
		fn wnd_ty(&self) -> WndTy;

		fn wm<F>(&self, msg: co::WM, func: F)
		where
			F: Fn(WndMsg) -> AnyResult<isize> + 'static;

		fn wm_command<F>(&self, code: impl Into<co::CMD>, func: F)
		where
			F: Fn() -> AnyResult<()> + 'static;

		fn wm_notify<F>(&self, code: impl Into<NmhdrCode>, func: F)
		where
			F: Fn(wm::Notify) -> AnyResult<isize> + 'static;
	}
}

impl priv_ctrl_events::GuiEvents for BaseCtrlEvents {
	fn wnd_ty(&self) -> WndTy {
		self.wnd_ty
	}

	fn wm<F>(&self, msg: co::WM, func: F)
	where
		F: Fn(WndMsg) -> AnyResult<isize> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		match &self.ctrl_ids {
			CtrlIds::One(_ctrl_id) => {
				parent_base_ref.on().wm(msg, func);
			},
			CtrlIds::Many(_ctrl_ids) => {}, // doesn't happen
		}
	}

	fn wm_command<F>(&self, code: impl Into<co::CMD>, func: F)
	where
		F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		match &self.ctrl_ids {
			CtrlIds::One(ctrl_id) => {
				// Common case.
				parent_base_ref.on().wm_command(*ctrl_id, code, func);
			},
			CtrlIds::Many(ctrl_ids) => {
				// For the RadioGroup, we add the event for multiple control IDs.
				let shared_func = Rc::new(func);
				for ctrl_id in ctrl_ids.iter() {
					parent_base_ref.on().wm_command(*ctrl_id, co::BN::CLICKED, {
						let shared_func = shared_func.clone();
						move || shared_func()
					});
				}
			},
		}
	}

	fn wm_notify<F>(&self, code: impl Into<NmhdrCode>, func: F)
	where
		F: Fn(wm::Notify) -> AnyResult<isize> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		match &self.ctrl_ids {
			CtrlIds::One(ctrl_id) => {
				parent_base_ref.on().wm_notify(*ctrl_id, code, func);
			},
			CtrlIds::Many(_ctrl_ids) => {}, // doesn't happen
		}
	}
}
