//! Raw bindings to shlwapi.lib functions.

use crate::ffi::PVOID;

extern_sys! { "shlwapi";
	SHCreateMemStream(*const u8, u32) -> PVOID
}
