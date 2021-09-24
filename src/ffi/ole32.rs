//! Raw bindings to ole32.lib functions.

use crate::ffi::{HRESULT, PCVOID, PVOID};

extern_sys! { "ole32",
	CoCreateInstance, PCVOID, PVOID, u32, PCVOID, *mut PVOID, => HRESULT
	CoInitializeEx, PVOID, u32, => HRESULT
	CoTaskMemFree, PVOID, => ()
	CoUninitialize, => ()
}
