use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	GetDefaultPrinterW(PSTR, *mut u32) -> BOOL
}
