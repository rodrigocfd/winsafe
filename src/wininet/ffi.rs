use crate::kernel::ffi_types::*;

extern_sys! { "wininet";
	HttpAddRequestHeadersW(HANDLE, PCSTR, u32, u32) -> BOOL
	HttpOpenRequestW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, *const PCSTR, u32, isize) -> HANDLE
	HttpQueryInfoW(HANDLE, u32, PVOID, *mut u32, *mut u32) -> BOOL
	HttpSendRequestW(HANDLE, PCSTR, u32, PCVOID, u32) -> BOOL
	InternetCanonicalizeUrlW(PCSTR, PSTR, *mut u32, u32) -> BOOL
	InternetCloseHandle(HANDLE) -> BOOL
	InternetCombineUrlW(PCSTR, PCSTR, PSTR, *mut u32, u32) -> BOOL
	InternetConnectW(HANDLE, PCSTR, u16, PCSTR, PCSTR, u32, u32, isize) -> HANDLE
	InternetCrackUrlW(PCSTR, u32, u32, PVOID) -> BOOL
	InternetOpenUrlW(HANDLE, PCSTR, PCSTR, u32, u32, isize) -> HANDLE
	InternetOpenW(PCSTR, u32, PCSTR, PCSTR, u32) -> HANDLE
	InternetQueryDataAvailable(HANDLE, *mut u32, u32, isize) -> BOOL
	InternetReadFile(HANDLE, PVOID, u32, *mut u32) -> BOOL
	InternetTimeToSystemTimeW(PCSTR, PVOID, u32) -> BOOL
}
