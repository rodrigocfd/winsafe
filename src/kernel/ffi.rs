use crate::kernel::ffi_types::*;

extern_sys! { "kernel32";
	BeginUpdateResourceW(PCSTR, BOOL) -> HANDLE
	CheckRemoteDebuggerPresent(HANDLE, *mut BOOL) -> BOOL
	CloseHandle(HANDLE) -> BOOL
	CopyFileW(PCSTR, PCSTR, BOOL) -> BOOL
	CreateDirectoryW(PCSTR, PVOID) -> BOOL
	CreateEventExW(PCVOID, PCSTR, u32, u32) -> HANDLE
	CreateEventW(PCVOID, BOOL, BOOL, PCSTR) -> HANDLE
	CreateFileMappingFromApp(HANDLE, PVOID, u32, u64, PCSTR) -> HANDLE
	CreateFileW(PCSTR, u32, u32, PVOID, u32, u32, HANDLE) -> HANDLE
	CreatePipe(*mut HANDLE, *mut HANDLE, PVOID, u32) -> BOOL
	CreateProcessW(PCSTR, PSTR, PVOID, PVOID, BOOL, u32, PVOID, PCSTR, PVOID, PVOID) -> BOOL
	CreateThread(PVOID, usize, PVOID, PVOID, u32, *mut u32) -> HANDLE
	CreateToolhelp32Snapshot(u32, u32) -> HANDLE
	DeleteFileW(PCSTR) -> BOOL
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
	FlushViewOfFile(PVOID, usize) -> BOOL
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
	GetCurrentThread() -> HANDLE
	GetCurrentThreadId() -> u32
	GetDiskFreeSpaceExW(PCSTR, *mut u64, *mut u64, *mut u64) -> BOOL
	GetDiskSpaceInformationW(PCSTR, PVOID) -> u32
	GetDriveTypeW(PCSTR) -> u32
	GetEnvironmentStringsW() -> *mut u16
	GetExitCodeProcess(HANDLE, *mut u32) -> BOOL
	GetExitCodeThread(HANDLE, *mut u32) -> BOOL
	GetFileAttributesW(PCSTR) -> u32
	GetFileInformationByHandle(HANDLE, PVOID) -> BOOL
	GetFileSizeEx(HANDLE, *mut i64) -> BOOL
	GetFileTime(HANDLE, PVOID, PVOID, PVOID) -> BOOL
	GetFileType(HANDLE) -> u32
	GetFirmwareType(*mut u32) -> BOOL
	GetGuiResources(HANDLE, u32) -> u32
	GetLargePageMinimum() -> usize
	GetLastError() -> u32
	GetLocalTime(PVOID)
	GetLogicalDrives() -> u32
	GetLogicalDriveStringsW(u32, PSTR) -> u32
	GetModuleFileNameW(HANDLE, PSTR, u32) -> u32
	GetModuleHandleExW(u32, PCSTR, *mut HANDLE) -> BOOL
	GetModuleHandleW(PCSTR) -> HANDLE
	GetNativeSystemInfo(PVOID)
	GetPriorityClass(HANDLE) -> u32
	GetPrivateProfileSectionNamesW(PSTR, u32, PCSTR) -> u32
	GetPrivateProfileSectionW(PCSTR, PSTR, u32, PCSTR) -> u32
	GetPrivateProfileStringW(PCSTR, PCSTR, PCSTR, PSTR, u32, PCSTR) -> u32
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
	GetTempFileNameW(PCSTR, PCSTR, u32, PSTR) -> u32
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
	HeapSetInformation(HANDLE, u32, PCVOID, usize) -> BOOL
	HeapSize(HANDLE, u32, PVOID) -> usize
	HeapUnlock(HANDLE) -> BOOL
	HeapValidate(HANDLE, u32, PVOID) -> BOOL
	HeapWalk(HANDLE, PVOID) -> BOOL
	IsDebuggerPresent() -> BOOL
	IsNativeVhdBoot(*mut BOOL) -> BOOL
	IsProcessCritical(HANDLE, *mut BOOL) -> BOOL
	IsWow64Process(HANDLE, *mut BOOL) -> BOOL
	LoadLibraryW(PCSTR) -> HANDLE
	LoadResource(HANDLE, HANDLE) -> HANDLE
	LocalAlloc(u32, usize) -> HANDLE
	LocalFlags(HANDLE) -> u32
	LocalFree(HANDLE) -> HANDLE
	LocalLock(HANDLE) -> PVOID
	LocalReAlloc(HANDLE, usize, u32) -> HANDLE
	LocalSize(HANDLE) -> usize
	LocalUnlock(HANDLE) -> BOOL
	LockFile(HANDLE, u32, u32, u32, u32) -> BOOL
	LockResource(HANDLE) -> PVOID
	lstrcmpW(PCSTR, PCSTR) -> i32
	lstrlenW(PCSTR) -> i32
	MapViewOfFileFromApp(HANDLE, u32, u64, usize) -> PVOID
	Module32FirstW(HANDLE, PVOID) -> BOOL
	Module32NextW(HANDLE, PVOID) -> BOOL
	MoveFileW(PCSTR, PCSTR) -> BOOL
	MoveFileExW(PCSTR, PCSTR, u32) -> BOOL
	MulDiv(i32, i32, i32) -> i32
	MultiByteToWideChar(u32, u32, *const u8, i32, PSTR, i32) -> i32
	OpenEventW(u32, BOOL, PCSTR) -> HANDLE
	OpenProcess(u32, BOOL, u32) -> HANDLE
	OutputDebugStringW(PCSTR)
	Process32FirstW(HANDLE, PVOID) -> BOOL
	Process32NextW(HANDLE, PVOID) -> BOOL
	PulseEvent(HANDLE) -> BOOL
	QueryFullProcessImageNameW(HANDLE, u32, PSTR, *mut u32) -> BOOL
	QueryPerformanceCounter(*mut i64) -> BOOL
	QueryPerformanceFrequency(*mut i64) -> BOOL
	QueryProcessAffinityUpdateMode(HANDLE, *mut u32) -> BOOL
	QueryProcessCycleTime(HANDLE, &mut u64) -> BOOL
	QueryThreadCycleTime(HANDLE, &mut u64) -> BOOL
	QueryUnbiasedInterruptTime(&mut u64) -> BOOL
	ReadConsoleW(HANDLE, PVOID, u32, *mut u32, PVOID) -> BOOL
	ReadFile(HANDLE, PVOID, u32, *mut u32, PVOID) -> BOOL
	ReplaceFileW(PCSTR, PCSTR, PCSTR, u32, PVOID, PVOID) -> BOOL
	ResetEvent(HANDLE) -> BOOL
	ResumeThread(HANDLE) -> u32
	SetConsoleMode(HANDLE, u32) -> BOOL
	SetCurrentDirectoryW(PCSTR) -> BOOL
	SetEndOfFile(HANDLE) -> BOOL
	SetEvent(HANDLE) -> BOOL
	SetFileAttributesW(PCSTR, u32) -> BOOL
	SetFilePointerEx(HANDLE, i64, *mut i64, u32) -> BOOL
	SetFileTime(HANDLE, PCVOID, PCVOID, PCVOID) -> BOOL
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
	WritePrivateProfileStringW(PCSTR, PCSTR, PCSTR, PCSTR) -> BOOL
}

extern_sys! { "user32"; // these functions should belong to kernel
	CharLowerW(PSTR) -> PSTR
	CharUpperW(PSTR) -> PSTR
}
