use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	AbortPrinter(HANDLE) -> BOOL
	AddPortW(PSTR, HANDLE, PSTR) -> BOOL
	AddPrinterConnectionW(PSTR) -> BOOL
	ClosePrinter(HANDLE) -> BOOL
	ConfigurePortW(PSTR, HANDLE, PSTR) -> BOOL
	DeleteFormW(HANDLE, PSTR) -> BOOL
	DeleteMonitorW(PSTR, PSTR, PSTR) -> BOOL
	DeletePrinter(HANDLE) -> BOOL
	DeletePrinterConnectionW(PSTR) -> BOOL
	DeletePrinterDataExW(HANDLE, PCSTR, PCSTR) -> BOOL
	DeletePrinterDataW(HANDLE, PSTR) -> BOOL
	DeletePrinterKeyW(HANDLE, PCSTR) -> BOOL
	EnumPrintersW(u32, PSTR, u32, *mut u8, u32, *mut u32, *mut u32) -> BOOL
	GetDefaultPrinterW(PSTR, *mut u32) -> BOOL
	OpenPrinterW(PSTR, *mut HANDLE, PVOID) -> BOOL
	ResetPrinterW(HANDLE, PVOID) -> BOOL
	SetDefaultPrinterW(PCSTR) -> BOOL
}
