use crate::kernel::ffi_types::*;

extern_sys! { "psapi";
	GetProcessMemoryInfo(HANDLE, PVOID, u32) -> BOOL
}
