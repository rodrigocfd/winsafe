//! Raw bindings to version.lib functions.

use crate::ffi::{BOOL, PCSTR, PCVOID, PVOID};

extern_sys! { "version";
	GetFileVersionInfoSizeW(PCSTR, *mut u32) -> u32
	GetFileVersionInfoW(PCSTR, u32, u32, PVOID) -> BOOL
	VerQueryValueW(PCVOID, PCSTR, PVOID, *mut u32) -> BOOL
}

