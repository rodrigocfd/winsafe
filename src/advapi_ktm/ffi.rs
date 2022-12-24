use crate::kernel::ffi_types::{HANDLE, PCSTR, PCVOID, PVOID};

extern_sys! { "advapi32";
	RegCreateKeyTransactedW(HANDLE, PCSTR, u32, PCSTR, u32, u32, PCVOID, *mut HANDLE, *mut u32, HANDLE, PVOID) -> i32
	RegOpenKeyTransactedW(HANDLE, PCSTR, u32, u32, *mut HANDLE, HANDLE, PVOID) -> i32
}
