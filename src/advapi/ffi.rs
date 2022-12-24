use crate::kernel::ffi_types::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "advapi32";
	DecryptFileW(PCSTR, u32) -> BOOL
	EncryptFileW(PCSTR) -> BOOL
	EncryptionDisable(PCSTR, BOOL) -> BOOL
	GetUserNameW(PSTR, *mut u32) -> BOOL
	InitializeSecurityDescriptor(PVOID, u32) -> BOOL
	IsValidSecurityDescriptor(PCVOID) -> BOOL
	RegCloseKey(HANDLE) -> i32
	RegConnectRegistryW(PCSTR, HANDLE, *mut HANDLE) -> i32
	RegCopyTreeW(HANDLE, PCSTR, HANDLE) -> i32
	RegCreateKeyExW(HANDLE, PCSTR, u32, PCSTR, u32, u32, PCVOID, *mut HANDLE, *mut u32) -> i32
	RegDeleteKeyExW(HANDLE, PCSTR, u32, u32) -> i32
	RegDeleteKeyW(HANDLE, PCSTR) -> i32
	RegDeleteTreeW(HANDLE, PCSTR) -> i32
	RegDeleteValueW(HANDLE, PCSTR) -> i32
	RegEnumKeyExW(HANDLE, u32, PSTR, *mut u32, *mut u32, PSTR, *mut u32, PVOID) -> i32
	RegEnumValueW(HANDLE, u32, PSTR, *mut u32, *mut u32, *mut u32, *mut u8, *mut u32) -> i32
	RegFlushKey(HANDLE) -> i32
	RegGetValueW(HANDLE, PCSTR, PCSTR, u32, *mut u32, PVOID, *mut u32) -> i32
	RegOpenKeyExW(HANDLE, PCSTR, u32, u32, *mut HANDLE) -> i32
	RegQueryInfoKeyW(HANDLE, PSTR, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, PVOID) -> i32
	RegQueryMultipleValuesW(HANDLE, PVOID, u32, PSTR, *mut u32) -> i32
	RegQueryValueExW(HANDLE, PCSTR, *mut u32, *mut u32, *mut u8, *mut u32) -> i32
	RegSetKeyValueW(HANDLE, PCSTR, PCSTR, u32, PCVOID, u32) -> i32
	RegSetValueExW(HANDLE, PCSTR, u32, u32, *const u8, u32) -> i32
}
