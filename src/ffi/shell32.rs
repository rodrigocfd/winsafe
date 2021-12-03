//! Raw bindings to shell32.lib functions.

use crate::ffi::{BOOL, HANDLE, HRES, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "shell32";
	CommandLineToArgvW(PCSTR, *mut i32) -> *mut PSTR
	DragFinish(HANDLE)
	DragQueryFileW(HANDLE, u32, PSTR, u32) -> u32
	DragQueryPoint(HANDLE, PVOID) -> BOOL
	SHAddToRecentDocs(u32, PCVOID)
	Shell_NotifyIconW(u32, PVOID) -> BOOL
	ShellExecuteW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, i32) -> HANDLE
	SHFileOperationW(PVOID) -> i32
	SHGetFileInfoW(PCSTR, u32, PVOID, u32, u32) -> usize
	SHCreateItemFromParsingName(PCSTR, PVOID, PCVOID, *mut PVOID) -> HRES
}
