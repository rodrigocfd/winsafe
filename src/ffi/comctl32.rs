//! Raw bindings to comctl32.lib functions.

use std::ffi::c_void;

use crate::ffi::HANDLE;

#[link(name = "comctl32")]
extern "system" {
	pub fn DefSubclassProc(hWnd: HANDLE, uMsg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn InitCommonControls();
	pub fn RemoveWindowSubclass(hWnd: HANDLE, pfnSubclass: *const c_void, uIdSubclass: usize) -> u32;
	pub fn SetWindowSubclass(hWnd: HANDLE, pfnSubclass: *const c_void, uIdSubclass: usize, dwRefData: usize) -> u32;
}