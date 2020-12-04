use crate::ffi::Void;

#[link(name = "kernel32")]
extern "system" {
	pub fn FormatMessageW(dwFlags: u32, lpSource: *const Void, dwMessageId: u32,
		dwLanguageId: u32, lpBuffer: *mut u16, nSize: u32,
		Arguments: *const Void) -> u32;
	pub fn GetLastError() -> u32;
	pub fn GetModuleHandleW(lpModuleName: *const u16) -> *mut Void;
	pub fn LocalFree(hMem: *mut Void) -> *mut Void;
	pub fn lstrlenW(lpString: *const u16) -> i32;
	pub fn SetLastError(dwErrorCode: u32);
}