use crate::ffi_types::PVOID;

extern_sys! { "shlwapi";
	SHCreateMemStream(*const u8, u32) -> PVOID
}
