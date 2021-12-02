//! Raw bindings to oleaut32.lib functions.

use crate::ffi::{HRES, PCSTR, PCVOID, PVOID};

extern_sys! { "oleaut32";
	OleLoadPicturePath(PCSTR, *mut PVOID, u32, u32, PCVOID, *mut PVOID) -> HRES
}
