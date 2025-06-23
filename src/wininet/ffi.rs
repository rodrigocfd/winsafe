use crate::kernel::ffi_types::*;

extern_sys! { "wininet";
	HttpOpenRequestW(HANDLE, PCSTR, PCSTR, PCSTR, PCSTR, PCSTR, u32, usize) -> HANDLE
	HttpSendRequestW(HANDLE, PCSTR, u32, PCVOID, u32) -> BOOL
	InternetCloseHandle(HANDLE) -> BOOL
	InternetConnectW(HANDLE, PCSTR, u16, PCSTR, PCSTR, u32, u32, usize) -> HANDLE
	InternetOpenW(PCSTR, u32, PCSTR, PCSTR, u32) -> HANDLE
	InternetReadFile(HANDLE, PVOID, u32, *mut u32) -> BOOL
}
