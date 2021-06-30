//! Raw bindings to comctl32.lib functions.

use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PFUNC};

#[link(name = "comctl32")]
extern "system" {
	pub fn DefSubclassProc(_: HANDLE, _: u32, _: usize, _: isize) -> isize;
	pub fn ImageList_Add(_: HANDLE, _: HANDLE, _: HANDLE) -> i32;
	pub fn ImageList_AddMasked(_: HANDLE, _: HANDLE, _: u32) -> i32;
	pub fn ImageList_BeginDrag(_: HANDLE, _: i32, _: i32, _: i32) -> BOOL;
	pub fn ImageList_Create(_: i32, _: i32, _: u32, _: i32, _: i32) -> HANDLE;
	pub fn ImageList_Destroy(_: HANDLE) -> BOOL;
	pub fn ImageList_DragMove(_: HANDLE, _: i32, _: i32) -> BOOL;
	pub fn ImageList_DragShowNolock(_: BOOL) -> BOOL;
	pub fn ImageList_EndDrag();
	pub fn ImageList_GetIconSize(_: HANDLE, _: *mut i32, _: *mut i32) -> BOOL;
	pub fn ImageList_GetImageCount(_: HANDLE) -> i32;
	pub fn ImageList_Remove(_: HANDLE, _: i32) -> BOOL;
	pub fn ImageList_ReplaceIcon(_: HANDLE, _: i32, _: HANDLE) -> i32;
	pub fn ImageList_SetImageCount(_: HANDLE, _: u32) -> BOOL;
	pub fn InitCommonControls();
	pub fn RemoveWindowSubclass(_: HANDLE, _: PFUNC, _: usize) -> BOOL;
	pub fn SetWindowSubclass(_: HANDLE, _: PFUNC, _: usize, _: usize) -> BOOL;
	pub fn TaskDialog(_: HANDLE, _: HANDLE, _: PCSTR, _: PCSTR, _: PCSTR, _: i32, _: PCSTR, _: *mut i32) -> HRESULT;
	pub fn TaskDialogIndirect(_: PCVOID, _: *mut i32, _: *mut i32, _: *mut BOOL) -> HRESULT;
}
