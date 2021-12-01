//! Raw bindings to ole32.lib functions.

use crate::ffi::{HRES, PCVOID, PVOID};

extern_sys! { "ole32";
	CoCreateInstance(PCVOID, PVOID, u32, PCVOID, *mut PVOID) -> HRES
	CoInitializeEx(PVOID, u32) -> HRES
	CoTaskMemFree(PVOID)
	CoUninitialize()
}
