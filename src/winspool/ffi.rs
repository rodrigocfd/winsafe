use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	EnumPrintersW(u32, PSTR, u32, *mut u8, u32, *mut u32, *mut u32) -> BOOL
	GetDefaultPrinterW(PSTR, *mut u32) -> BOOL
}
