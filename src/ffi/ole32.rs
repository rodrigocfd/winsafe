//! Raw bindings to ole32.lib functions.

use crate::ffi::Void;

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(rclsid: *const Void, pUnkOuter: *mut Void,
		dwClsContext: u32, riid: *const Void,
		ppv: *mut *const *const Void) -> u32;
	pub fn CoInitializeEx(lpReserved: *const Void, dwCoInit: u32) -> u32;
	pub fn CoUninitialize();
}