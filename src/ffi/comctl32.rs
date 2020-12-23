//! Raw bindings to comctl32.lib functions.

use crate::ffi::{BOOL, HANDLE, PFUNC};

#[link(name = "comctl32")]
extern "system" {
	pub fn DefSubclassProc(hWnd: HANDLE, uMsg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn InitCommonControls();
	pub fn RemoveWindowSubclass(hWnd: HANDLE, pfnSubclass: PFUNC, uIdSubclass: usize) -> BOOL;
	pub fn SetWindowSubclass(hWnd: HANDLE, pfnSubclass: PFUNC, uIdSubclass: usize, dwRefData: usize) -> BOOL;
}