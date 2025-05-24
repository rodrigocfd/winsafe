use crate::kernel::ffi_types::*;

extern_sys! { "psapi";
	GetProcessMemoryInfo(HANDLE, PVOID, u32) -> BOOL
	EnumProcessModules(HANDLE, *mut *mut HANDLE, u32, *const u32) -> BOOL
	GetModuleBaseNameA(HANDLE, HANDLE, PSTR, u32) -> u32
	GetModuleInformation(HANDLE, HANDLE, *mut crate::MODULEINFO, u32) -> BOOL
}
