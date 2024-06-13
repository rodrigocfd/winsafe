use crate::kernel::ffi_types::*;

extern_sys! { "shell32";
	ShellExecuteExW(PVOID) -> BOOL
	SHGetKnownFolderPath(PCVOID, u32, HANDLE, *mut PSTR) -> HRES
}
