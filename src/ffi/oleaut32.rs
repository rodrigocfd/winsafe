//! Raw bindings to oleaut32.lib functions.

use crate::ffi::{BOOL, HRES, PCSTR, PCVOID, PVOID};

extern_sys! { "oleaut32";
	OleLoadPicture(PVOID, i32, BOOL, PCVOID, PVOID) -> HRES
	OleLoadPicturePath(PCSTR, *mut PVOID, u32, u32, PCVOID, *mut PVOID) -> HRES
}
