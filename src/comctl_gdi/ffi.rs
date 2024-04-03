use crate::kernel::ffi_types::*;

extern_sys! { "comctl32";
	ImageList_DrawIndirect(HANDLE, PCVOID) -> BOOL
}
