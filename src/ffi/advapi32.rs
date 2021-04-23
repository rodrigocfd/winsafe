//! Raw bindings to advapi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "advapi32")]
extern "system" {
	pub fn DecryptFileW(lpFileName: PCSTR, dwReserved: u32) -> BOOL;
	pub fn EncryptFileW(lpFileName: PCSTR) -> BOOL;
	pub fn EncryptionDisable(DirPath: PCSTR, Disable: BOOL) -> BOOL;
	pub fn RegCloseKey(hKey: HANDLE) -> BOOL;
	pub fn RegEnumKeyExW(hKey: HANDLE, dwIndex: u32, lpName: PSTR, lpcchName: *mut u32, lpReserved: *mut u32, lpClass: PSTR, lpcchClass: *mut u32, lpftLastWriteTime: PVOID) -> BOOL;
	pub fn RegEnumValueW(hKey: HANDLE, dwIndex: u32, lpValueName: PSTR, lpcchValueName: *mut u32, lpReserved: *mut u32, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> BOOL;
	pub fn RegGetValueW(hkey: HANDLE, lpSubKey: PCSTR, lpValue: PCSTR, dwFlags: u32, pdwType: *mut u32, pvData: PVOID, pcbData: *mut u32) -> BOOL;
	pub fn RegOpenKeyExW(hKey: HANDLE, lpSubKey: PCSTR, ulOptions: u32, samDesired: u32, phkResult: *mut HANDLE) -> BOOL;
	pub fn RegQueryInfoKeyW(hKey: HANDLE, lpClass: PSTR, lpcchClass: *mut u32, lpReserved: *mut u32, lpcSubKeys: *mut u32, lpcbMaxSubKeyLen: *mut u32, lpcbMaxClassLen: *mut u32, lpcValues: *mut u32, lpcbMaxValueNameLen: *mut u32, lpcbMaxValueLen: *mut u32, lpcbSecurityDescriptor: *mut u32, lpftLastWriteTime: PVOID) -> BOOL;
	pub fn RegQueryValueExW(hKey: HANDLE, lpValueName: PCSTR, lpReserved: *mut u32, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> BOOL;
	pub fn RegSetKeyValueW(hKey: HANDLE, lpSubKey: PCSTR, lpValueName: PCSTR, dwType: u32, lpData: PCVOID, cbData: u32) -> BOOL;
	pub fn RegSetValueExW(hKey: HANDLE, lpValueName: PCSTR, Reserved: u32, dwType: u32, lpData: *const u8, cbData: u32) -> BOOL;
}
