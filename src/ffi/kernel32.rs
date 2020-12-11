//! Raw bindings to kernel32.lib functions.

use std::ffi::c_void;

use crate::ffi::HANDLE;

#[link(name = "kernel32")]
extern "system" {
	pub fn FormatMessageW(dwFlags: u32, lpSource: *const c_void, dwMessageId: u32, dwLanguageId: u32, lpBuffer: *mut u16, nSize: u32, Arguments: *const c_void) -> u32;
	pub fn GetComputerNameW(lpBuffer: *mut u16, nSize: *mut u32) -> u32;
	pub fn GetLastError() -> u32;
	pub fn GetModuleHandleW(lpModuleName: *const u16) -> HANDLE;
	pub fn LocalFree(hMem: HANDLE) -> HANDLE;
	pub fn lstrlenW(lpString: *const u16) -> i32;
	pub fn SetLastError(dwErrorCode: u32);
}