#![allow(non_snake_case)]

use crate::comctl;
use crate::comctl::decl::SUBCLASSPROC;
use crate::kernel::decl::WinResult;
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, MsgSend};
use crate::user::decl::HWND;

impl ComctlHwnd for HWND {}

/// [`HWND`](crate::HWND) methods from `comctl` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub trait ComctlHwnd: Handle {
	/// [`DefSubclassProc`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	fn DefSubclassProc<M>(self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				comctl::ffi::DefSubclassProc(
					self.as_ptr(), wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`RemoveWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	fn RemoveWindowSubclass(self,
		subclass_func: SUBCLASSPROC, subclass_id: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl::ffi::RemoveWindowSubclass(
					self.as_ptr(),
					subclass_func as _,
					subclass_id,
				)
			},
		)
	}

	/// [`SetWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	fn SetWindowSubclass(self,
		subclass_proc: SUBCLASSPROC,
		subclass_id: usize, ref_data: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl::ffi::SetWindowSubclass(
					self.as_ptr(),
					subclass_proc as _,
					subclass_id,
					ref_data,
				)
			},
		)
	}
}
