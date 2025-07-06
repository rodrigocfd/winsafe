use crate::kernel::ffi_types::*;

extern_sys! { "dwmapi";
	DwmEnableMMCSS(BOOL) -> HRES
	DwmExtendFrameIntoClientArea(HANDLE, PCVOID) -> HRES
	DwmFlush() -> HRES
	DwmGetColorizationColor(*mut u32, *mut BOOL) -> HRES
	DwmGetWindowAttribute(HANDLE, u32, PVOID, u32) -> HRES
	DwmInvalidateIconicBitmaps(HANDLE) -> HRES
	DwmIsCompositionEnabled(*mut BOOL) -> HRES
	DwmModifyPreviousDxFrameDuration(HANDLE, i32, BOOL) -> HRES
	DwmSetDxFrameDuration(HANDLE, i32) -> HRES
	DwmSetIconicLivePreviewBitmap(HANDLE, HANDLE, PCVOID, u32) -> HRES
	DwmSetIconicThumbnail(HANDLE, HANDLE, u32) -> HRES
	DwmSetWindowAttribute(HANDLE, u32, PCVOID, u32) -> HRES
	DwmShowContact(u32, u32) -> HRES
}
