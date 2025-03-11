use std::ptr::NonNull;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;

/// Base to all native control events. This is actually a proxy to the events of
/// the parent window; events added to a native control are actually added as
/// `WM_COMMAND` or `WM_NOTIFY` messages under the parent window.
pub(in crate::gui) struct BaseCtrlEvents {
	parent_ptr: NonNull<BaseWnd>, // used only to add the events to parent, before the first message is processed
	is_dlg: IsDlg,
	ctrl_id: u16,
}

impl BaseCtrlEvents {
	#[must_use]
	pub(in crate::gui) fn new(parent: &impl AsRef<BaseWnd>, ctrl_id: u16) -> Self {
		Self {
			parent_ptr: NonNull::from(parent.as_ref()),
			ctrl_id,
			is_dlg: parent.as_ref().is_dlg(),
		}
	}

	#[must_use]
	pub(in crate::gui) const fn is_dlg(&self) -> IsDlg {
		self.is_dlg
	}

	/// Adds a `WM` event to the parent window.
	pub(in crate::gui) fn wm<F>(&self, msg: co::WM, func: F)
		where F: Fn(WndMsg) -> AnyResult<isize> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm(msg, func);
	}

	/// Adds a `WM_COMMAND` event to the parent window.
	pub(in crate::gui) fn wm_command<F>(&self, code: impl Into<co::CMD>, func: F)
		where F: Fn() -> AnyResult<()> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm_command(self.ctrl_id, code, func);
	}

	/// Adds a `WM_NOTIFY` event to the parent window.
	pub(in crate::gui) fn wm_notify<F>(&self,
		code: impl Into<NmhdrCode>,
		func: F,
	)
		where F: Fn(wm::Notify) -> AnyResult<isize> + 'static,
	{
		let parent_base_ref = unsafe { self.parent_ptr.as_ref() };
		parent_base_ref.on().wm_notify(self.ctrl_id, code, func);
	}
}
