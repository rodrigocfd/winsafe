//! Raw bindings to ole32.lib functions.

use crate::ffi::{PCVOID, PVOID};

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(rclsid: PCVOID, pUnkOuter: PVOID, dwClsContext: u32, riid: PCVOID, ppv: *mut PVOID) -> u32;
	pub fn CoInitializeEx(lpReserved: PVOID, dwCoInit: u32) -> u32;
	pub fn CoUninitialize();
}
