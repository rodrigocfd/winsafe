#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl_gdi::ffi;
use crate::decl::*;
use crate::kernel::privs::*;

impl HIMAGELIST {
	/// [`ImageList_DrawIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_drawindirect)
	/// function.
	pub fn DrawIndirect(&self, imldp: &IMAGELISTDRAWPARAMS) -> HrResult<()> {
		match unsafe { ffi::ImageList_DrawIndirect(self.ptr(), pcvoid(imldp)) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}
}
