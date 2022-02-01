use crate::ffi_types::{BOOL, HANDLE, HRES, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "shell32";
	CommandLineToArgvW(PCSTR, *mut i32) -> *mut PSTR
	DragFinish(HANDLE)
	DragQueryFileW(HANDLE, u32, PSTR, u32) -> u32
	DragQueryPoint(HANDLE, PVOID) -> BOOL
	SHAddToRecentDocs(u32, PCVOID)
	SHCreateItemFromParsingName(PCSTR, PVOID, PCVOID, *mut PVOID) -> HRES
	Shell_NotifyIconW(u32, PVOID) -> BOOL
	ShellExecuteW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, i32) -> HANDLE
	SHFileOperationW(PVOID) -> i32
	SHGetFileInfoW(PCSTR, u32, PVOID, u32, u32) -> usize
	SHGetKnownFolderPath(PCVOID, u32, HANDLE, *mut PSTR) -> HRES
	SHGetStockIconInfo(u32, u32, PVOID) -> HRES
}
