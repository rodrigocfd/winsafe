use crate::ffi::Void;

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(rclsid: *const Void, pUnkOuter: *mut Void,
		dwClsContext: u32, riid: *const Void, ppv: *mut *mut *mut Void) -> u32;
	pub fn CoInitializeEx(lpReserved: *mut Void, dwCoInit: u32) -> u32;
	pub fn CoUninitialize();
}