#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::dwm::ffi;
use crate::kernel::privs::*;
use crate::ole::privs::*;

impl HWND {
	/// [`DwmExtendFrameIntoClientArea`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmextendframeintoclientarea)
	/// function.
	pub fn DwmExtendFrameIntoClientArea(&self, margins_inset: &MARGINS) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			ffi::DwmExtendFrameIntoClientArea(self.ptr(), pcvoid(margins_inset))
		})
	}

	/// [`DwmInvalidateIconicBitmaps`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwminvalidateiconicbitmaps)
	/// function.
	pub fn DwmInvalidateIconicBitmaps(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ffi::DwmInvalidateIconicBitmaps(self.ptr()) })
	}

	/// [`DwmSetIconicLivePreviewBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmseticoniclivepreviewbitmap)
	/// function.
	pub fn DwmSetIconicLivePreviewBitmap(
		&self,
		hbmp: HBITMAP,
		pt_client: Option<POINT>,
		sit_flags: Option<co::DWM_SIT>,
	) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			ffi::DwmSetIconicLivePreviewBitmap(
				self.ptr(),
				hbmp.ptr(),
				pcvoid_or_null(pt_client.as_ref()),
				sit_flags.unwrap_or_default().raw(),
			)
		})
	}

	/// [`DwmSetIconicThumbnail`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmseticonicthumbnail)
	/// function.
	pub fn DwmSetIconicThumbnail(
		&self,
		hbmp: HBITMAP,
		sit_flags: Option<co::DWM_SIT>,
	) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			ffi::DwmSetIconicThumbnail(self.ptr(), hbmp.ptr(), sit_flags.unwrap_or_default().raw())
		})
	}
}
