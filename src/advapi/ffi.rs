use crate::kernel::ffi_types::*;

extern_sys! { "advapi32";
	AdjustTokenPrivileges(HANDLE, BOOL, PCVOID, u32, PVOID, *mut u32) -> BOOL
	AllocateAndInitializeSid(PCVOID, u8, u32, u32, u32, u32, u32, u32, u32, u32, PVOID) -> BOOL
	CheckTokenCapability(HANDLE, PCVOID, *mut BOOL) -> BOOL
	CheckTokenMembership(HANDLE, PCVOID, *mut BOOL) -> BOOL
	CloseServiceHandle(HANDLE) -> BOOL
	ConvertSidToStringSidW(PCVOID, *mut PSTR) -> BOOL
	ConvertStringSidToSidW(PCSTR, *mut *mut u8) -> BOOL
	CopySid(u32, PVOID, PCVOID) -> BOOL
	CreateServiceW(HANDLE, PCSTR, PCSTR, u32, u32, u32, u32, PCSTR, PCSTR, *mut u32, PCSTR, PCSTR, PCSTR) -> HANDLE
	CreateWellKnownSid(u32, PCVOID, PVOID, *mut u32) -> BOOL
	DecryptFileW(PCSTR, u32) -> BOOL
	DeleteService(HANDLE) -> BOOL
	DeregisterEventSource(HANDLE) -> BOOL
	DuplicateToken(HANDLE, u32, *mut HANDLE) -> BOOL
	EncryptFileW(PCSTR) -> BOOL
	EncryptionDisable(PCSTR, BOOL) -> BOOL
	EqualDomainSid(PCVOID, PCVOID, *mut BOOL) -> BOOL
	EqualPrefixSid(PCVOID, PCVOID) -> BOOL
	EqualSid(PCVOID, PCVOID) -> BOOL
	FreeSid(PVOID)
	GetLengthSid(PCVOID) -> u32
	GetSidLengthRequired(u8) -> u32
	GetTokenInformation(HANDLE, u32, PCVOID, u32, *mut u32) -> BOOL
	GetUserNameW(PSTR, *mut u32) -> BOOL
	GetWindowsAccountDomainSid(PCVOID, PVOID, *mut u32) -> BOOL
	ImpersonateLoggedOnUser(HANDLE) -> BOOL
	InitializeSecurityDescriptor(PVOID, u32) -> BOOL
	InitiateSystemShutdownExW(PCSTR, PCSTR, u32, BOOL, BOOL, u32) -> BOOL
	InitiateSystemShutdownW(PCSTR, PCSTR, u32, BOOL, BOOL) -> BOOL
	IsTokenRestricted(HANDLE) -> BOOL
	IsValidSecurityDescriptor(PCVOID) -> BOOL
	IsValidSid(PCVOID) -> BOOL
	IsWellKnownSid(PCVOID, u32) -> BOOL
	LookupAccountNameW(PCSTR, PCSTR, PVOID, *mut u32, PSTR, *mut u32, *mut u32) -> BOOL
	LookupAccountSidW(PCSTR, PCVOID, PSTR, *mut u32, PSTR, *mut u32, *mut u32) -> BOOL
	LookupPrivilegeNameW(PCSTR, PCVOID, PSTR, *mut u32) -> BOOL
	LookupPrivilegeValueW(PCSTR, PCSTR, PVOID) -> BOOL
	OpenProcessToken(HANDLE, u32, *mut HANDLE) -> BOOL
	OpenSCManagerW(PCSTR, PCSTR, u32) -> HANDLE
	OpenServiceW(HANDLE, PCSTR, u32) -> HANDLE
	OpenThreadToken(HANDLE, u32, BOOL, *mut HANDLE) -> BOOL
	RegCloseKey(HANDLE) -> i32
	RegConnectRegistryW(PCSTR, HANDLE, *mut HANDLE) -> i32
	RegCopyTreeW(HANDLE, PCSTR, HANDLE) -> i32
	RegCreateKeyExW(HANDLE, PCSTR, u32, PCSTR, u32, u32, PCVOID, *mut HANDLE, *mut u32) -> i32
	RegCreateKeyTransactedW(HANDLE, PCSTR, u32, PCSTR, u32, u32, PCVOID, *mut HANDLE, *mut u32, HANDLE, PVOID) -> i32
	RegDeleteKeyExW(HANDLE, PCSTR, u32, u32) -> i32
	RegDeleteKeyTransactedW(HANDLE, PCSTR, u32, u32, HANDLE, PVOID) -> i32
	RegDeleteKeyW(HANDLE, PCSTR) -> i32
	RegDeleteTreeW(HANDLE, PCSTR) -> i32
	RegDeleteValueW(HANDLE, PCSTR) -> i32
	RegDisablePredefinedCache() -> i32
	RegDisablePredefinedCacheEx() -> i32
	RegDisableReflectionKey(HANDLE) -> i32
	RegEnableReflectionKey(HANDLE) -> i32
	RegEnumKeyExW(HANDLE, u32, PSTR, *mut u32, *mut u32, PSTR, *mut u32, PVOID) -> i32
	RegEnumValueW(HANDLE, u32, PSTR, *mut u32, *mut u32, *mut u32, *mut u8, *mut u32) -> i32
	RegFlushKey(HANDLE) -> i32
	RegGetValueW(HANDLE, PCSTR, PCSTR, u32, *mut u32, PVOID, *mut u32) -> i32
	RegisterEventSourceW(PCSTR, PCSTR) -> HANDLE
	RegisterServiceCtrlHandlerExW(PCSTR, PFUNC, PCVOID) -> HANDLE
	RegLoadKeyW(HANDLE, PCSTR, PCSTR) -> i32
	RegOpenCurrentUser(u32, *mut HANDLE) -> i32
	RegOpenKeyExW(HANDLE, PCSTR, u32, u32, *mut HANDLE) -> i32
	RegOpenKeyTransactedW(HANDLE, PCSTR, u32, u32, *mut HANDLE, HANDLE, PVOID) -> i32
	RegQueryInfoKeyW(HANDLE, PSTR, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, PVOID) -> i32
	RegQueryMultipleValuesW(HANDLE, PVOID, u32, PSTR, *mut u32) -> i32
	RegQueryReflectionKey(HANDLE, *mut BOOL) -> i32
	RegQueryValueExW(HANDLE, PCSTR, *mut u32, *mut u32, *mut u8, *mut u32) -> i32
	RegRenameKey(HANDLE, PCSTR, PCSTR) -> i32
	RegReplaceKeyW(HANDLE, PCSTR, PCSTR, PCSTR) -> i32
	RegRestoreKeyW(HANDLE, PCSTR, u32) -> i32
	RegSaveKeyExW(HANDLE, PCSTR, PCVOID, u32) -> i32
	RegSaveKeyW(HANDLE, PCSTR, PCVOID) -> i32
	RegSetKeyValueW(HANDLE, PCSTR, PCSTR, u32, PCVOID, u32) -> i32
	RegSetValueExW(HANDLE, PCSTR, u32, u32, *const u8, u32) -> i32
	RegUnLoadKeyW(HANDLE, PCSTR) -> i32
	ReportEventW(HANDLE, u16, u16, u32, PCVOID, u16, u32, *const PCSTR, PCVOID) -> BOOL
	SetServiceStatus(HANDLE, PCVOID) -> BOOL
}

extern_sys! { "ktmw32";
	CommitTransaction(HANDLE) -> BOOL
	CreateTransaction(PCVOID, PVOID, u32, u32, u32, u32, PSTR) -> HANDLE
	GetTransactionId(HANDLE, PVOID) -> BOOL
	OpenTransaction(u32, PCVOID) -> HANDLE
	RollbackTransaction(HANDLE) -> BOOL
}
