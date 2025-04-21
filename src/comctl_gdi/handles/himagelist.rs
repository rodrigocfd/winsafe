#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl_gdi::ffi;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl comctl_gdi_Himagelist for HIMAGELIST {}

/// This trait is enabled with `comctl` and `gdi` features, and provides methods
/// for [`HIMAGELIST`](crate::HIMAGELIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_gdi_Himagelist: Handle {
	/// [`ImageList_DrawIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_drawindirect)
	/// function.
	fn DrawIndirect(&self, imldp: &IMAGELISTDRAWPARAMS) -> HrResult<()> {
		match unsafe { ffi::ImageList_DrawIndirect(self.ptr(), pcvoid(imldp)) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}
}
