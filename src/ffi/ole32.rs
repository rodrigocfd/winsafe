//! Raw bindings to ole32.lib functions.

use crate::ffi::{HRESULT, PCVOID, PVOID};

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(rclsid: PCVOID, pUnkOuter: PVOID, dwClsContext: u32, riid: PCVOID, ppv: *mut PVOID) -> HRESULT;
	pub fn CoInitializeEx(lpReserved: PVOID, dwCoInit: u32) -> HRESULT;
	pub fn CoTaskMemFree(pv: PVOID);
	pub fn CoUninitialize();
}
