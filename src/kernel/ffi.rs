use crate::kernel::ffi_types::{BOOL, HANDLE, PCSTR, PCVOID, PFUNC, PSTR, PVOID};

extern_sys! { "advapi32";
	AllocateAndInitializeSid(PCVOID, u8, u32, u32, u32, u32, u32, u32, u32, u32, *mut u8) -> BOOL
	ConvertSidToStringSidW(PCVOID, *mut PSTR) -> BOOL
	ConvertStringSidToSidW(PCSTR, *mut *mut u8) -> BOOL
	CopySid(u32, *mut u8, PCVOID) -> BOOL
	CreateWellKnownSid(u32, PCVOID, *mut u8, *mut u32) -> BOOL
	DecryptFileW(PCSTR, u32) -> BOOL
	EncryptFileW(PCSTR) -> BOOL
	EncryptionDisable(PCSTR, BOOL) -> BOOL
	EqualDomainSid(PVOID, PVOID, *mut BOOL) -> BOOL
	EqualPrefixSid(PVOID, PVOID) -> BOOL
	EqualSid(PVOID, PVOID) -> BOOL
	FreeSid(PVOID)
	GetLengthSid(PVOID) -> u32
	GetSidLengthRequired(u8) -> u32
	GetUserNameW(PSTR, *mut u32) -> BOOL
	GetWindowsAccountDomainSid(PCVOID, *mut u8, *mut u32) -> BOOL
	InitializeSecurityDescriptor(PVOID, u32) -> BOOL
	IsValidSecurityDescriptor(PCVOID) -> BOOL
	IsValidSid(PVOID) -> BOOL
	IsWellKnownSid(PVOID, u32) -> BOOL
	LookupAccountNameW(PCSTR, PCSTR, *mut u8, *mut u32, PSTR, *mut u32, *mut u32) -> BOOL
	LookupAccountSidW(PCSTR, PCVOID, PSTR, *mut u32, PSTR, *mut u32, *mut u32) -> BOOL
	LookupPrivilegeValueW(PCSTR, PCSTR, PVOID) -> BOOL
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
	RegSaveKeyExW(HANDLE, PCSTR, PVOID, u32) -> i32
	RegSaveKeyW(HANDLE, PCSTR, PVOID) -> i32
	RegSetKeyValueW(HANDLE, PCSTR, PCSTR, u32, PCVOID, u32) -> i32
	RegSetValueExW(HANDLE, PCSTR, u32, u32, *const u8, u32) -> i32
	RegUnLoadKeyW(HANDLE, PCSTR) -> i32
}

extern_sys! { "kernel32";
	BeginUpdateResourceW(PCSTR, BOOL) -> HANDLE
	CheckRemoteDebuggerPresent(HANDLE, *mut BOOL) -> BOOL
	CloseHandle(HANDLE) -> BOOL
	CopyFileW(PCSTR, PCSTR, BOOL) -> BOOL
	CreateFileMappingFromApp(HANDLE, PVOID, u32, u64, PCSTR) -> HANDLE
	CreateFileW(PCSTR, u32, u32, PVOID, u32, u32, HANDLE) -> HANDLE
	CreatePipe(*mut HANDLE, *mut HANDLE, PVOID, u32) -> BOOL
	CreateProcessW(PCSTR, PSTR, PVOID, PVOID, BOOL, u32, PVOID, PCSTR, PVOID, PVOID) -> BOOL
	CreateThread(PVOID, usize, PVOID, PVOID, u32, *mut u32) -> HANDLE
	CreateToolhelp32Snapshot(u32, u32) -> HANDLE
	DeleteFileW(PCSTR) -> BOOL
	DuplicateToken(HANDLE, u32, *mut HANDLE) -> BOOL
	EndUpdateResourceW(HANDLE, BOOL) -> BOOL
	EnumResourceLanguagesW(HANDLE, PCSTR, PCSTR, PFUNC, isize) -> BOOL
	EnumResourceNamesW(HANDLE, PCSTR, PFUNC, isize) -> BOOL
	EnumResourceTypesW(HANDLE, PFUNC, isize) -> BOOL
	ExitProcess(u32)
	ExitThread(u32)
	ExpandEnvironmentStringsW(PCSTR, PSTR, u32) -> u32
	FileTimeToSystemTime(PCVOID, PVOID) -> BOOL
	FindClose(HANDLE) -> BOOL
	FindFirstFileW(PCSTR, PVOID) -> HANDLE
	FindNextFileW(HANDLE, PVOID) -> BOOL
	FindResourceExW(HANDLE, PCSTR, PCSTR, u16) -> HANDLE
	FindResourceW(HANDLE, PCSTR, PCSTR) -> HANDLE
	FlushConsoleInputBuffer(HANDLE) -> BOOL
	FlushInstructionCache(HANDLE, PCVOID, usize) -> BOOL
	FlushProcessWriteBuffers()
	FormatMessageW(u32, PCVOID, u32, u32, PSTR, u32, PVOID) -> u32
	FreeEnvironmentStringsW(HANDLE) -> BOOL
	FreeLibrary(HANDLE) -> BOOL
	GetBinaryTypeW(PCSTR, *mut u32) -> BOOL
	GetCommandLineW() -> PCSTR
	GetComputerNameW(PSTR, *mut u32) -> BOOL
	GetConsoleMode(HANDLE, *mut u32) -> BOOL
	GetCurrentDirectoryW(u32, PSTR) -> u32
	GetCurrentProcess() -> HANDLE
	GetCurrentProcessId() -> u32
	GetCurrentProcessToken() -> HANDLE
	GetCurrentThread() -> HANDLE
	GetCurrentThreadEffectiveToken() -> HANDLE
	GetCurrentThreadId() -> u32
	GetDiskSpaceInformationW(PCSTR, PVOID) -> u32
	GetDriveTypeW(PCSTR) -> u32
	GetEnvironmentStringsW() -> *mut u16
	GetExitCodeProcess(HANDLE, *mut u32) -> BOOL
	GetExitCodeThread(HANDLE, *mut u32) -> BOOL
	GetFileAttributesW(PCSTR) -> u32
	GetFileInformationByHandle(HANDLE, PVOID) -> BOOL
	GetFileSizeEx(HANDLE, *mut i64) -> BOOL
	GetFileType(HANDLE) -> u32
	GetFirmwareType(*mut u32) -> BOOL
	GetGuiResources(HANDLE, u32) -> u32
	GetLargePageMinimum() -> usize
	GetLastError() -> u32
	GetLocalTime(PVOID)
	GetLogicalDrives() -> u32
	GetLogicalDriveStringsW(u32, PSTR) -> u32
	GetModuleFileNameW(HANDLE, PSTR, u32) -> u32
	GetModuleHandleW(PCSTR) -> HANDLE
	GetNativeSystemInfo(PVOID)
	GetPriorityClass(HANDLE) -> u32
	GetProcAddress(HANDLE, *const u8) -> PCVOID
	GetProcessHandleCount(HANDLE, &mut u32) -> BOOL
	GetProcessHeap() -> HANDLE
	GetProcessHeaps(u32, *mut HANDLE) -> u32
	GetProcessId(HANDLE) -> u32
	GetProcessIdOfThread(HANDLE) -> u32
	GetProcessTimes(HANDLE, PVOID, PVOID, PVOID, PVOID) -> BOOL
	GetStartupInfoW(PVOID)
	GetStdHandle(u32) -> HANDLE
	GetSystemDirectoryW(PSTR, u32) -> u32
	GetSystemFileCacheSize(*mut usize, *mut usize, *mut u32) -> BOOL
	GetSystemInfo(PVOID)
	GetSystemTime(PVOID)
	GetSystemTimeAsFileTime(PVOID)
	GetSystemTimePreciseAsFileTime(PVOID)
	GetSystemTimes(PVOID, PVOID, PVOID) -> BOOL
	GetTempPathW(u32, PSTR) -> u32
	GetThreadId(HANDLE) -> u32
	GetThreadTimes(HANDLE, PVOID, PVOID, PVOID, PVOID) -> BOOL
	GetTickCount64() -> u64
	GetVolumeInformationW(PCSTR, PSTR, u32, *mut u32, *mut u32, *mut u32, PSTR, u32) -> BOOL
	GetVolumePathNameW(PCSTR, PSTR, u32) -> BOOL
	GlobalAlloc(u32, usize) -> HANDLE
	GlobalFlags(HANDLE) -> u32
	GlobalFree(HANDLE) -> HANDLE
	GlobalLock(HANDLE) -> PVOID
	GlobalMemoryStatusEx(PVOID) -> BOOL
	GlobalReAlloc(HANDLE, usize, u32) -> HANDLE
	GlobalSize(HANDLE) -> usize
	GlobalUnlock(HANDLE) -> BOOL
	Heap32ListFirst(HANDLE, PVOID) -> BOOL
	Heap32ListNext(HANDLE, PVOID) -> BOOL
	HeapAlloc(HANDLE, u32, usize) -> PVOID
	HeapCompact(HANDLE, u32) -> usize
	HeapCreate(u32, usize, usize) -> HANDLE
	HeapDestroy(HANDLE) -> BOOL
	HeapFree(HANDLE, u32, PVOID) -> BOOL
	HeapLock(HANDLE) -> BOOL
	HeapReAlloc(HANDLE, u32, PVOID, usize) -> PVOID
	HeapSize(HANDLE, u32, PVOID) -> usize
	HeapUnlock(HANDLE) -> BOOL
	HeapWalk(HANDLE, PVOID) -> BOOL
	IsDebuggerPresent() -> BOOL
	IsNativeVhdBoot(*mut BOOL) -> BOOL
	IsProcessCritical(HANDLE, *mut BOOL) -> BOOL
	IsTokenRestricted(HANDLE) -> BOOL
	IsWow64Process(HANDLE, *mut BOOL) -> BOOL
	LoadLibraryW(PCSTR) -> HANDLE
	LoadResource(HANDLE, HANDLE) -> HANDLE
	LocalAlloc(u32, usize) -> HANDLE
	LocalFlags(HANDLE) -> u32
	LocalFree(HANDLE) -> HANDLE
	LocalReAlloc(HANDLE, usize, u32) -> HANDLE
	LocalSize(HANDLE) -> usize
	LockFile(HANDLE, u32, u32, u32, u32) -> BOOL
	LockResource(HANDLE) -> PVOID
	lstrlenW(PCSTR) -> i32
	MapViewOfFileFromApp(HANDLE, u32, u64, usize) -> PVOID
	Module32FirstW(HANDLE, PVOID) -> BOOL
	Module32NextW(HANDLE, PVOID) -> BOOL
	MoveFileW(PCSTR, PCSTR) -> BOOL
	MulDiv(i32, i32, i32) -> i32
	MultiByteToWideChar(u32, u32, *const u8, i32, PSTR, i32) -> i32
	OpenProcess(u32, BOOL, u32) -> HANDLE
	OpenProcessToken(HANDLE, u32, *mut HANDLE) -> BOOL
	OpenThreadToken(HANDLE, u32, BOOL, *mut HANDLE) -> BOOL
	OutputDebugStringW(PCSTR)
	Process32FirstW(HANDLE, PVOID) -> BOOL
	Process32NextW(HANDLE, PVOID) -> BOOL
	QueryFullProcessImageNameW(HANDLE, u32, PSTR, *mut u32) -> BOOL
	QueryPerformanceCounter(*mut i64) -> BOOL
	QueryPerformanceFrequency(*mut i64) -> BOOL
	QueryProcessAffinityUpdateMode(HANDLE, *mut u32) -> BOOL
	ReadConsoleW(HANDLE, PVOID, u32, *mut u32, PVOID) -> BOOL
	ReadFile(HANDLE, PVOID, u32, *mut u32, PVOID) -> BOOL
	ReplaceFileW(PCSTR, PCSTR, PCSTR, u32, PVOID, PVOID) -> BOOL
	ResumeThread(HANDLE) -> u32
	SetConsoleMode(HANDLE, u32) -> BOOL
	SetCurrentDirectoryW(PCSTR) -> BOOL
	SetEndOfFile(HANDLE) -> BOOL
	SetFilePointerEx(HANDLE, i64, *mut i64, u32) -> BOOL
	SetLastError(u32)
	SetPriorityClass(HANDLE, u32) -> BOOL
	SetProcessAffinityUpdateMode(HANDLE, u32) -> BOOL
	SetProcessPriorityBoost(HANDLE, BOOL) -> BOOL
	SetThreadIdealProcessor(HANDLE, u32) -> u32
	SetThreadIdealProcessorEx(HANDLE, PCVOID, PVOID) -> BOOL
	SetThreadPriorityBoost(HANDLE, BOOL) -> BOOL
	SetThreadStackGuarantee(*mut u32) -> BOOL
	SizeofResource(HANDLE, HANDLE) -> u32
	Sleep(u32)
	SuspendThread(HANDLE) -> u32
	SwitchToThread() -> BOOL
	SystemTimeToFileTime(PCVOID, PVOID) -> BOOL
	SystemTimeToTzSpecificLocalTime(PCVOID, PCVOID, PVOID) -> BOOL
	TerminateProcess(HANDLE, u32) -> BOOL
	TerminateThread(HANDLE, u32) -> BOOL
	Thread32First(HANDLE, PVOID) -> BOOL
	Thread32Next(HANDLE, PVOID) -> BOOL
	UnlockFile(HANDLE, u32, u32, u32, u32) -> BOOL
	UnmapViewOfFile(PCVOID) -> BOOL
	UpdateResourceW(HANDLE, PCSTR, PCSTR, u16, PVOID, u32) -> BOOL
	VerifyVersionInfoW(PVOID, u32, u64) -> BOOL
	VerSetConditionMask(u64, u32, u8) -> u64
	WaitForSingleObject(HANDLE, u32) -> u32
	WideCharToMultiByte(u32, u32, PCSTR, i32, PSTR, i32, *const u8, *mut BOOL) -> i32
	WriteConsoleW(HANDLE, PCVOID, u32, *mut u32, PVOID) -> BOOL
	WriteFile(HANDLE, PCVOID, u32, *mut u32, PVOID) -> BOOL
}

extern_sys! { "ktmw32";
	CommitTransaction(HANDLE) -> BOOL
	CreateTransaction(PVOID, PVOID, u32, u32, u32, u32, PSTR) -> HANDLE
	GetTransactionId(HANDLE, PVOID) -> BOOL
	OpenTransaction(u32, PVOID) -> HANDLE
	RollbackTransaction(HANDLE) -> BOOL
}
