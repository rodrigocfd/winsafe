//! Raw bindings to shell32.lib functions.

use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "shell32")]
extern "system" {
	pub fn DragFinish(_: HANDLE);
	pub fn DragQueryFileW(_: HANDLE, _: u32, _: PSTR, _: u32) -> u32;
	pub fn DragQueryPoint(_: HANDLE, _: PVOID) -> BOOL;
	pub fn SHCreateItemFromParsingName(_: PCSTR, _: PVOID, _: PCVOID, _: *mut PVOID) -> HRESULT;
	pub fn Shell_NotifyIconW(_: u32, _: PVOID) -> BOOL;
	pub fn SHGetFileInfoW(_: PCSTR, _: u32, _: PVOID, _: u32, _: u32) -> usize;
}
