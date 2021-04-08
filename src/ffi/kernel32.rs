//! Raw bindings to kernel32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "kernel32")]
extern "system" {
	pub fn CloseHandle(hObject: HANDLE) -> BOOL;
	pub fn ExpandEnvironmentStringsW(lpSrc: PCSTR, lpDst: PSTR, nSize: u32) -> u32;
	pub fn FileTimeToSystemTime(lpFileTime: PCVOID, lpSystemTime: PVOID) -> BOOL;
	pub fn FormatMessageW(dwFlags: u32, lpSource: PCVOID, dwMessageId: u32, dwLanguageId: u32, lpBuffer: PSTR, nSize: u32, Arguments: PVOID) -> u32;
	pub fn FreeEnvironmentStringsW(penv: HANDLE) -> BOOL;
	pub fn GetEnvironmentStringsW() -> HANDLE;
	pub fn GetLastError() -> u32;
	pub fn GetModuleHandleW(lpModuleName: PCSTR) -> HANDLE;
	pub fn GetSystemTime(lpSystemTime: PVOID);
	pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: PVOID);
	pub fn GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime: PVOID);
	pub fn GetTickCount64() -> u64;
	pub fn LocalFree(hMem: HANDLE) -> HANDLE;
	pub fn lstrlenW(lpString: PCSTR) -> i32;
	pub fn MulDiv(nNumber: i32, nNumerator: i32, nDenominator: i32) -> i32;
	pub fn OutputDebugStringW(lpOutputString: PCSTR);
	pub fn SetLastError(dwErrorCode: u32);
	pub fn Sleep(dwMilliseconds: u32);
	pub fn SystemTimeToFileTime(lpSystemTime: PCVOID, lpFileTime: PVOID) -> BOOL;
	pub fn SystemTimeToTzSpecificLocalTime(lpTimeZoneInformation: PCVOID, lpUniversalTime: PCVOID, lpLocalTime: PVOID) -> BOOL;
	pub fn VerifyVersionInfoW(lpVersionInformation: PVOID, dwTypeMask: u32, dwlConditionMask: u64) -> BOOL;
	pub fn VerSetConditionMask(ConditionMask: u64, TypeMask: u32, Condition: u8) -> u64;
}
