//! Raw bindings to ole32.lib functions.

use crate::ffi::{HRESULT, PCVOID, PVOID};

#[link(name = "ole32")]
extern "system" {
	pub fn CoCreateInstance(_: PCVOID, _: PVOID, _: u32, _: PCVOID, _: *mut PVOID) -> HRESULT;
	pub fn CoInitializeEx(_: PVOID, _: u32) -> HRESULT;
	pub fn CoTaskMemFree(_: PVOID);
	pub fn CoUninitialize();
}
