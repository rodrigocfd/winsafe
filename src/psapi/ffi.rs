use crate::kernel::ffi_types::*;

extern_sys! { "psapi";
	EmptyWorkingSet(HANDLE) -> BOOL
	GetMappedFileNameW(HANDLE, PVOID, PSTR, u32) -> BOOL
	GetModuleBaseNameW(HANDLE, HANDLE, PSTR, u32) -> BOOL
	GetModuleFileNameExW(HANDLE, HANDLE, PSTR, u32) -> BOOL
	GetModuleInformation(HANDLE, HANDLE, PVOID, u32) -> BOOL
	GetPerformanceInfo(PVOID, u32) -> BOOL
	GetProcessImageFileNameW(HANDLE, PSTR, u32) -> BOOL
	GetProcessMemoryInfo(HANDLE, PVOID, u32) -> BOOL
}
