#![allow(non_camel_case_types, non_snake_case)]

use crate::dwm;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::Handle;
use crate::uxtheme::decl::MARGINS;

/// This trait is enabled with the `dwm` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait dwm_Hwnd: Handle {
	/// [`DwmExtendFrameIntoClientArea`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmextendframeintoclientarea)
	/// function.
	fn DwmExtendFrameIntoClientArea(&self,
		margins_inset: &MARGINS) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				dwm::ffi::DwmExtendFrameIntoClientArea(
					self.ptr(),
					margins_inset as *const _ as _,
				)
			},
		)
	}
}
