//! Raw bindings to kernel32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "kernel32")]
extern "system" {
	pub fn CloseHandle(hObject: HANDLE) -> BOOL;
	pub fn CopyFileW(lpExistingFileName: PCSTR, lpNewFileName: PCSTR, bFailIfExists: BOOL) -> BOOL;
	pub fn CreateFileMappingW(hFile: HANDLE, lpFileMappingAttributes: PVOID, flProtect: u32, dwMaximumSizeHigh: u32, dwMaximumSizeLow: u32, lpName: PCSTR) -> HANDLE;
	pub fn CreateFileW(lpFileName: PCSTR, dwDesiredAccess: u32, dwShareMode: u32, lpSecurityAttributes: PVOID, dwCreationDisposition: u32, dwFlagsAndAttributes: u32, hTemplateFile: HANDLE) -> HANDLE;
	pub fn DeleteFileW(lpFileName: PCSTR) -> BOOL;
	pub fn ExpandEnvironmentStringsW(lpSrc: PCSTR, lpDst: PSTR, nSize: u32) -> u32;
	pub fn FileTimeToSystemTime(lpFileTime: PCVOID, lpSystemTime: PVOID) -> BOOL;
	pub fn FormatMessageW(dwFlags: u32, lpSource: PCVOID, dwMessageId: u32, dwLanguageId: u32, lpBuffer: PSTR, nSize: u32, Arguments: PVOID) -> u32;
	pub fn FreeEnvironmentStringsW(penv: HANDLE) -> BOOL;
	pub fn GetEnvironmentStringsW() -> HANDLE;
	pub fn GetFileAttributesW(lpFileName: PCSTR) -> u32;
	pub fn GetFileInformationByHandle(hFile: HANDLE, lpFileInformation: PVOID) -> BOOL;
	pub fn GetFileSizeEx(hFile: HANDLE, lpFileSize: *mut i64) -> BOOL;
	pub fn GetFileType(hFile: HANDLE) -> u32;
	pub fn GetLastError() -> u32;
	pub fn GetLogicalDriveStringsW(nBufferLength: u32, lpBuffer: PSTR) -> u32;
	pub fn GetModuleHandleW(lpModuleName: PCSTR) -> HANDLE;
	pub fn GetSystemTime(lpSystemTime: PVOID);
	pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: PVOID);
	pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: PVOID);
	pub fn GetTempPathW(nBufferLength: u32, lpBuffer: PSTR) -> u32;
	pub fn GetTickCount64() -> u64;
	pub fn LocalFree(hMem: HANDLE) -> HANDLE;
	pub fn LockFile(hFile: HANDLE, dwFileOffsetLow: u32, dwFileOffsetHigh: u32, nNumberOfBytesToLockLow: u32, nNumberOfBytesToLockHigh: u32) -> BOOL;
	pub fn lstrlenW(lpString: PCSTR) -> i32;
	pub fn MapViewOfFile(hFileMappingObject: HANDLE, dwDesiredAccess: u32, dwFileOffsetHigh: u32, dwFileOffsetLow: u32, dwNumberOfBytesToMap: i64) -> PVOID;
	pub fn MoveFileW(lpExistingFileName: PCSTR, lpNewFileName: PCSTR) -> BOOL;
	pub fn MulDiv(nNumber: i32, nNumerator: i32, nDenominator: i32) -> i32;
	pub fn MultiByteToWideChar(CodePage: u32, dwFlags: u32, lpMultiByteStr: *const u8, cbMultiByte: i32, lpWideCharStr: PSTR, cchWideChar: i32) -> i32;
	pub fn OutputDebugStringW(lpOutputString: PCSTR);
	pub fn ReadFile(hFile: HANDLE, lpBuffer: PVOID, nNumberOfBytesToRead: u32, lpNumberOfBytesRead: *mut u32, lpOverlapped: PVOID) -> BOOL;
	pub fn SetEndOfFile(hFile: HANDLE) -> BOOL;
	pub fn SetFilePointerEx(hFile: HANDLE, liDistanceToMove: i64, lpNewFilePointer: *mut i64, dwMoveMethod: u32) -> BOOL;
	pub fn SetLastError(dwErrorCode: u32);
	pub fn Sleep(dwMilliseconds: u32);
	pub fn SystemTimeToFileTime(lpSystemTime: PCVOID, lpFileTime: PVOID) -> BOOL;
	pub fn SystemTimeToTzSpecificLocalTime(lpTimeZoneInformation: PCVOID, lpUniversalTime: PCVOID, lpLocalTime: PVOID) -> BOOL;
	pub fn UnlockFile(hFile: HANDLE, dwFileOffsetLow: u32, dwFileOffsetHigh: u32, nNumberOfBytesToLockLow: u32, nNumberOfBytesToLockHigh: u32) -> BOOL;
	pub fn UnmapViewOfFile(lpBaseAddress: PCVOID) -> BOOL;
	pub fn VerifyVersionInfoW(lpVersionInformation: PVOID, dwTypeMask: u32, dwlConditionMask: u64) -> BOOL;
	pub fn VerSetConditionMask(ConditionMask: u64, TypeMask: u32, Condition: u8) -> u64;
	pub fn WideCharToMultiByte(CodePage: u32, dwFlags: u32, lpWideCharStr: PCSTR, cchWideChar: i32, lpMultiByteStr: PSTR, cbMultiByte: i32, lpDefaultChar: *const u8, lpUsedDefaultChar: *mut BOOL) -> i32;
	pub fn WriteFile(hFile: HANDLE, lpBuffer: PCVOID, nNumberOfBytesToWrite: u32, lpNumberOfBytesWritten: *mut u32, lpOverlapped: PVOID) -> BOOL;
}
