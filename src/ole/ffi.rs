use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};

extern_sys! { "ole32";
	CoCreateInstance(PCVOID, PVOID, u32, PCVOID, *mut PVOID) -> HRES
	CoInitializeEx(PVOID, u32) -> HRES
	CoTaskMemAlloc(usize) -> PVOID
	CoTaskMemFree(PVOID)
	CoTaskMemRealloc(PVOID, usize) -> PVOID
	CoUninitialize()
}
