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

	/// [`DwmGetWindowAttribute`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetwindowattribute)
	/// function.
	pub fn DwmGetWindowAttribute(&self, attribute: co::DWMWA) -> HrResult<DwmAttr> {
		let mut buf_u32 = 0u32;
		let mut buf_rc = RECT::new();
		let (ptr, sz) = DwmAttr::ptr_sz_of_flag(attribute, &mut buf_u32, &mut buf_rc);

		ok_to_hrresult(unsafe { ffi::DwmGetWindowAttribute(self.ptr(), attribute.raw(), ptr, sz) })
			.map(|_| DwmAttr::from_raw(attribute, buf_u32, buf_rc))
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

	/// [`DwmSetWindowAttribute`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmsetwindowattribute)
	/// function.
	pub fn DwmSetWindowAttribute(&self, attribute: DwmAttr) -> HrResult<()> {
		let mut buf_u32 = 0u32;
		let mut buf_rc = RECT::new();

		ok_to_hrresult(unsafe {
			ffi::DwmSetWindowAttribute(
				self.ptr(),
				attribute.flag().raw(),
				attribute.ptr(&mut buf_u32, &mut buf_rc),
				attribute.sz(),
			)
		})
	}
}
