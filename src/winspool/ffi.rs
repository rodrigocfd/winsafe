use crate::kernel::ffi_types::*;

extern_sys! { "winspool";
	AbortPrinter(HANDLE) -> BOOL
	AddFormW(HANDLE, u32, PCVOID) -> BOOL
	AddJobW(HANDLE, u32, PVOID, u32, *mut u32) -> BOOL
	AddPortW(PSTR, HANDLE, PSTR) -> BOOL
	AddPrinterConnection2W(HANDLE, PCSTR, u32, PCVOID) -> BOOL
	AddPrinterConnectionW(PSTR) -> BOOL
	AddPrinterW(PCSTR, u32, PCVOID) -> HANDLE
	AdvancedDocumentPropertiesW(HANDLE, HANDLE, PCSTR, PVOID, PCVOID) -> i32
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
	GetPrinterW(HANDLE, u32, PVOID, u32, *mut u32) -> BOOL
	OpenPrinter2W(PCSTR, *mut HANDLE, PCVOID, PCVOID) -> BOOL
	OpenPrinterW(PSTR, *mut HANDLE, PCVOID) -> BOOL
	PrinterProperties(HANDLE, HANDLE) -> BOOL
	ResetPrinterW(HANDLE, PCVOID) -> BOOL
	ScheduleJob(HANDLE, u32) -> BOOL
	SetDefaultPrinterW(PCSTR) -> BOOL
}
