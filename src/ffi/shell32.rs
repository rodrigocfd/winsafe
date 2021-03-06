//! Raw bindings to shell32.lib functions.

use crate::ffi::{BOOL, HANDLE, PSTR, PVOID};

#[link(name = "shell32")]
extern "system" {
	pub fn DragFinish(hDrop: HANDLE);
	pub fn DragQueryFileW(hDrop: HANDLE, hFile: u32, lpszFile: PSTR, cch: u32) -> u32;
	pub fn DragQueryPoint(hDrop: HANDLE, ppt: PVOID) -> BOOL;
}
