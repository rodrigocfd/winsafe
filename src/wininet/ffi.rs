use crate::kernel::ffi_types::*;

extern_sys! { "wininet";
	HttpOpenRequestW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, *const PCSTR, u32, isize) -> HANDLE
	HttpQueryInfoW(HANDLE, u32, PVOID, *mut u32, *mut u32) -> BOOL
	HttpSendRequestW(HANDLE, PCSTR, u32, PCVOID, u32) -> BOOL
	InternetCanonicalizeUrlW(PCSTR, PSTR, *mut u32, u32) -> BOOL
	InternetCloseHandle(HANDLE) -> BOOL
	InternetConnectW(HANDLE, PCSTR, u16, PCSTR, PCSTR, u32, u32, isize) -> HANDLE
	InternetOpenUrlW(HANDLE, PCSTR, PCSTR, u32, u32, isize) -> HANDLE
	InternetOpenW(PCSTR, u32, PCSTR, PCSTR, u32) -> HANDLE
	InternetReadFile(HANDLE, PVOID, u32, *mut u32) -> BOOL
}
