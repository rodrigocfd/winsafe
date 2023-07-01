#![allow(non_camel_case_types, non_snake_case)]

use crate::ole;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ole_IDropTarget, user_Hwnd};
use crate::user::decl::HWND;

impl ole_Hwnd for HWND {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_Hwnd: user_Hwnd {
	/// [`RegisterDragDrop`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-registerdragdrop)
	/// function.
	fn RegisterDragDrop(&self,
		drop_target: &impl ole_IDropTarget) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				ole::ffi::RegisterDragDrop(self.ptr(), drop_target.ptr() as _)
			},
		)
	}

	/// [`RevokeDragDrop`](https://learn.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-revokedragdrop)
	/// function.
	fn RevokeDragDrop(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ole::ffi::RevokeDragDrop(self.ptr()) })
	}
}
