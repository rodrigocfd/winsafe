#![allow(unused_imports)]

use crate::kernel::ffi_types::*;
use crate::macros::*;

#[cfg(target_pointer_width = "64")]
extern_sys! { "comctl32";
	ImageList_DrawIndirect(HANDLE, PCVOID) -> BOOL
}
