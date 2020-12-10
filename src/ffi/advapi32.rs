//! Raw bindings to advapi32.lib functions.

use crate::ffi::HANDLE;

#[link(name = "advapi32")]
extern "system" {
	pub fn RegCloseKey(hKey: HANDLE) -> u32;
	pub fn RegOpenKeyExW(hKey: HANDLE, lpSubKey: *const u16, ulOptions: u32, samDesired: u32, phkResult: *mut HANDLE) -> u32;
	pub fn RegQueryValueExW(hKey: HANDLE, lpValueName: *const u16, lpReserved: *mut u32, lpType: *mut u32, lpData: *mut u8, lpcbData: *mut u32) -> u32;
}