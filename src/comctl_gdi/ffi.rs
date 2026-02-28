use crate::kernel::ffi_types::*;
use crate::macros::*;

extern_sys! { "comctl32";
	ImageList_DrawIndirect(HANDLE, PCVOID) -> BOOL
}
