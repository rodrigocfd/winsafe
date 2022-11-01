#![allow(non_camel_case_types, non_snake_case)]

use crate::comctl;
use crate::comctl::decl::SUBCLASSPROC;
use crate::kernel::decl::SysResult;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::{Handle, MsgSend};
use crate::user::decl::HWND;

impl comctl_Hwnd for HWND {}

/// This trait is enabled with the `comctl` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub trait comctl_Hwnd: Handle {
	/// [`DefSubclassProc`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
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

	/// [`RemoveWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	fn RemoveWindowSubclass(self,
		subclass_func: SUBCLASSPROC, subclass_id: usize) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				comctl::ffi::RemoveWindowSubclass(
					self.as_ptr(),
					subclass_func as _,
					subclass_id,
				)
			},
		)
	}

	/// [`SetWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	fn SetWindowSubclass(self,
		subclass_proc: SUBCLASSPROC,
		subclass_id: usize, ref_data: usize) -> SysResult<()>
	{
		bool_to_sysresult(
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
