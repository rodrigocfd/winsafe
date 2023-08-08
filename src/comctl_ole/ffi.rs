use crate::kernel::ffi_types::*;

extern_sys! { "comctl32";
	InitializeFlatSB(HANDLE) -> HRES
	TaskDialog(HANDLE, HANDLE, PCSTR, PCSTR, PCSTR, i32, PCSTR, *mut i32) -> HRES
	TaskDialogIndirect(PCVOID, *mut i32, *mut i32, *mut BOOL) -> HRES
	UninitializeFlatSB(HANDLE) -> HRES
}
