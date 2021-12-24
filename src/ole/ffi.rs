use crate::ffi_types::{HRES, PCVOID, PVOID};

extern_sys! { "ole32";
	CoCreateInstance(PCVOID, PVOID, u32, PCVOID, *mut PVOID) -> HRES
	CoInitializeEx(PVOID, u32) -> HRES
	CoTaskMemFree(PVOID)
	CoUninitialize()
}
