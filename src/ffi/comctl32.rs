//! Raw bindings to comctl32.lib functions.

use crate::ffi::{BOOL, HANDLE, PFUNC};

#[link(name = "comctl32")]
extern "system" {
	pub fn DefSubclassProc(hWnd: HANDLE, uMsg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn ImageList_Add(himl: HANDLE, hbmImage: HANDLE, hbmMask: HANDLE) -> i32;
	pub fn ImageList_AddMasked(himl: HANDLE, hbmImage: HANDLE, crMask: u32) -> i32;
	pub fn ImageList_BeginDrag(himlTrack: HANDLE, iTrack: i32, dxHotspot: i32, dyHotspot: i32) -> BOOL;
	pub fn ImageList_Create(cx: i32, cy: i32, flags: u32, cInitial: i32, cGrow: i32) -> HANDLE;
	pub fn ImageList_Destroy(himl: HANDLE) -> BOOL;
	pub fn ImageList_DragMove(himl: HANDLE, x: i32, y: i32) -> BOOL;
	pub fn ImageList_DragShowNolock(fShow: BOOL) -> BOOL;
	pub fn ImageList_EndDrag();
	pub fn ImageList_GetImageCount(himl: HANDLE) -> i32;
	pub fn ImageList_Remove(himl: HANDLE, i: i32) -> BOOL;
	pub fn ImageList_ReplaceIcon(himl: HANDLE, i: i32, hicon: HANDLE) -> i32;
	pub fn ImageList_SetImageCount(himl: HANDLE, uNewCount: u32) -> BOOL;
	pub fn InitCommonControls();
	pub fn RemoveWindowSubclass(hWnd: HANDLE, pfnSubclass: PFUNC, uIdSubclass: usize) -> BOOL;
	pub fn SetWindowSubclass(hWnd: HANDLE, pfnSubclass: PFUNC, uIdSubclass: usize, dwRefData: usize) -> BOOL;
}
