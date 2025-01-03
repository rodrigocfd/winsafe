use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	ClosePrinter(HANDLE) -> BOOL
	EnumPrintersW(u32, PSTR, u32, *mut u8, u32, *mut u32, *mut u32) -> BOOL
	GetDefaultPrinterW(PSTR, *mut u32) -> BOOL
	OpenPrinterW(PSTR, *mut HANDLE, PVOID) -> BOOL
}
