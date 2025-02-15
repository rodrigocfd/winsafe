use std::ptr::NonNull;

use crate::co;
use crate::decl::*;
use crate::gui::{*, privs::*};
use crate::msg::*;

/// Base to all native control events. This is actually a proxy to the events of
/// the parent window; events added to a native control are actually added as
/// `WM_COMMAND` or `WM_NOTIFY` messages under the parent window.
pub(in crate::gui) struct BaseCtrlEventsProxy {
	parent_ptr: NonNull<Base>, // used only to add the events to parent, before the first message is processed
	ctrl_id: u16,
}

impl BaseCtrlEventsProxy {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<Base>, ctrl_id: u16) -> Self {
		Self {
			parent_ptr: NonNull::from(parent.as_ref()),
			ctrl_id,
		}
	}

	/// Adds a `WM_COMMAND` event to the parent window.
	pub(in crate::gui) fn wm_command<F>(&self, code: impl Into<co::CMD>, func: F)
		where F: Fn() -> AnyResult<WmRet> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm_command(self.ctrl_id, code, func);
	}

	/// Adds a `WM_NOTIFY` event to the parent window.
	pub(in crate::gui) fn wm_notify<F>(&self,
		code: impl Into<NmhdrCode>,
		func: F,
	)
		where F: Fn(wm::Notify) -> AnyResult<WmRet> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm_notify(self.ctrl_id, code, func);
	}
}
