use crate::kernel::ffi_types::*;
use crate::macros::*;

extern_sys! { "winusb";
	WinUsb_AbortPipe(HANDLE, u8) -> BOOL
	WinUsb_FlushPipe(HANDLE, u8) -> BOOL
	WinUsb_Free(HANDLE) -> BOOL
	WinUsb_GetCurrentAlternateSetting(HANDLE, *mut u8) -> BOOL
	WinUsb_GetCurrentFrameNumber(HANDLE, *mut u32, *mut i64) -> BOOL
	WinUsb_GetDescriptor(HANDLE, u8, u8, u16, *mut u8, u32, *mut u32) -> BOOL
	WinUsb_Initialize(HANDLE, *mut HANDLE) -> BOOL
	WinUsb_ResetPipe(HANDLE, u8) -> BOOL
	WinUsb_SetCurrentAlternateSetting(HANDLE, u8) -> BOOL
}
