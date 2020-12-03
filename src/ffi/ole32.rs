use crate::ffi::Void;

#[link(name = "ole32")]
extern "system" {
	pub fn CoInitializeEx(lpReserved: *mut Void, dwCoInit: u32) -> u32;
	pub fn CoUninitialize();
}