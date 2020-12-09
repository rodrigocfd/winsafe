//! Raw bindings to ole32.lib functions.

use std::ffi::c_void;

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(rclsid: *const c_void, pUnkOuter: *mut c_void, dwClsContext: u32, riid: *const c_void, ppv: *mut *mut *mut c_void) -> u32;
	pub fn CoInitializeEx(lpReserved: *const c_void, dwCoInit: u32) -> u32;
	pub fn CoUninitialize();
}