use crate::kernel::ffi_types::{BOOL, HANDLE, HRES, PCVOID};

extern_sys! { "dwmapi";
	DwmExtendFrameIntoClientArea(HANDLE, PCVOID) -> HRES
	DwmIsCompositionEnabled(*mut BOOL) -> HRES
	DwmFlush() -> HRES
}
