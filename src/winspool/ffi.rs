use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	AbortPrinter(HANDLE) -> BOOL
	AddPrinterConnection(PSTR) -> BOOL
	ClosePrinter(HANDLE) -> BOOL
	DeleteForm(HANDLE, PSTR) -> BOOL
	DeletePrinter(HANDLE) -> BOOL
	DeletePrinterConnection(PSTR) -> BOOL
	DeletePrinterData(HANDLE, PSTR) -> BOOL
	DeletePrinterDataEx(HANDLE, PCSTR, PCSTR) -> BOOL
	DeletePrinterKey(HANDLE, PCSTR) -> BOOL
	EnumPrintersW(u32, PSTR, u32, *mut u8, u32, *mut u32, *mut u32) -> BOOL
	GetDefaultPrinterW(PSTR, *mut u32) -> BOOL
	OpenPrinterW(PSTR, *mut HANDLE, PVOID) -> BOOL
	ResetPrinter(HANDLE, PVOID) -> BOOL
	SetDefaultPrinter(PCSTR) -> BOOL
}
