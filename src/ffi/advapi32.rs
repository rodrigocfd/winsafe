//! Raw bindings to advapi32.lib functions.

use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};

#[link(name = "advapi32")]
extern "system" {
	pub fn DecryptFileW(_: PCSTR, _: u32) -> BOOL;
	pub fn EncryptFileW(_: PCSTR) -> BOOL;
	pub fn EncryptionDisable(_: PCSTR, _: BOOL) -> BOOL;
	pub fn RegCloseKey(_: HANDLE) -> BOOL;
	pub fn RegEnumKeyExW(_: HANDLE, _: u32, _: PSTR, _: *mut u32, _: *mut u32, _: PSTR, _: *mut u32, _: PVOID) -> BOOL;
	pub fn RegEnumValueW(_: HANDLE, _: u32, _: PSTR, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u8, _: *mut u32) -> BOOL;
	pub fn RegGetValueW(_: HANDLE, _: PCSTR, _: PCSTR, _: u32, _: *mut u32, _: PVOID, _: *mut u32) -> BOOL;
	pub fn RegOpenKeyExW(_: HANDLE, _: PCSTR, _: u32, _: u32, _: *mut HANDLE) -> BOOL;
	pub fn RegQueryInfoKeyW(_: HANDLE, _: PSTR, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: *mut u32, _: PVOID) -> BOOL;
	pub fn RegQueryValueExW(_: HANDLE, _: PCSTR, _: *mut u32, _: *mut u32, _: *mut u8, _: *mut u32) -> BOOL;
	pub fn RegSetKeyValueW(_: HANDLE, _: PCSTR, _: PCSTR, _: u32, _: PCVOID, _: u32) -> BOOL;
	pub fn RegSetValueExW(_: HANDLE, _: PCSTR, _: u32, _: u32, _: *const u8, _: u32) -> BOOL;
}
