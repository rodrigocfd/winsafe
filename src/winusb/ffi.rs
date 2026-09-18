use crate::kernel::ffi_types::*;
use crate::macros::*;

extern_sys! { "winusb";
	WinUsb_Free(HANDLE) -> BOOL
	WinUsb_Initialize(HANDLE, *mut HANDLE) -> BOOL
}
