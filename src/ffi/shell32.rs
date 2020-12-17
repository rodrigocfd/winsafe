//! Raw bindings to shell32.lib functions.

use std::ffi::c_void;

use crate::ffi::HANDLE;

#[link(name = "shell32")]
extern "system" {
	pub fn DragFinish(hDrop: HANDLE);
	pub fn DragQueryFileW(hDrop: HANDLE, hFile: u32, lpszFile: *mut u16, cch: u32) -> u32;
	pub fn DragQueryPoint(hDrop: HANDLE, ppt: *mut c_void) -> u32;
}