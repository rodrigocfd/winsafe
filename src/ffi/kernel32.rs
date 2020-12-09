//! Raw bindings to kernel32.lib functions.

use std::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
	pub fn FormatMessageW(dwFlags: u32, lpSource: *const c_void, dwMessageId: u32, dwLanguageId: u32, lpBuffer: *const u16, nSize: u32, Arguments: *const c_void) -> u32;
	pub fn GetLastError() -> u32;
	pub fn GetModuleHandleW(lpModuleName: *const u16) -> *const c_void;
	pub fn LocalFree(hMem: *const c_void) -> *const c_void;
	pub fn lstrlenW(lpString: *const u16) -> i32;
	pub fn SetLastError(dwErrorCode: u32);
}