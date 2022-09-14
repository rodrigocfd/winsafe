#![allow(non_camel_case_types, non_snake_case)]

use crate::ole;
use crate::ole::decl::{HrResult, IDropTarget};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, ole_IUnknown};
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
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub trait ole_Hwnd: Handle {
	/// [`RegisterDragDrop`](https://docs.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-registerdragdrop)
	/// method.
	fn RegisterDragDrop(self, drop_target: &IDropTarget) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				ole::ffi::RegisterDragDrop(self.as_ptr(), drop_target.ptr().0 as _)
			},
		)
	}

	/// [`RevokeDragDrop`](https://docs.microsoft.com/en-us/windows/win32/api/ole2/nf-ole2-revokedragdrop)
	/// method.
	fn RevokeDragDrop(self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ole::ffi::RevokeDragDrop(self.as_ptr()) })
	}
}
