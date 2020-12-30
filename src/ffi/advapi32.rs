//! Raw bindings to advapi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PVOID};

#[link(name = "advapi32")]
extern "system" {
	pub fn RegCloseKey(hKey: HANDLE) -> BOOL;
	pub fn RegGetValueW(hkey: HANDLE, lpSubKey: PCSTR, lpValue: PCSTR, dwFlags: u32, pdwType: *mut u32, pvData: PVOID, pcbData: *mut u32) -> BOOL;
	pub fn RegOpenKeyExW(hKey: HANDLE, lpSubKey: PCSTR, ulOptions: u32, samDesired: u32, phkResult: *mut HANDLE) -> BOOL;
	pub fn RegQueryValueExW(hKey: HANDLE, lpValueName: PCSTR, lpReserved: *mut u32, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> BOOL;
	pub fn RegSetKeyValueW(hKey: HANDLE, lpSubKey: PCSTR, lpValueName: PCSTR, dwType: u32, lpData: PCVOID, cbData: u32) -> BOOL;
	pub fn RegSetValueExW(hKey: HANDLE, lpValueName: PCSTR, Reserved: u32, dwType: u32, lpData: *const u8, cbData: u32) -> BOOL;
}
