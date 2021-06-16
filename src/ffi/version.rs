//! Raw bindings to version.lib functions.

use crate::ffi::{BOOL, PCSTR, PCVOID, PVOID};

#[link(name = "version")]
extern "system" {
	pub fn GetFileVersionInfoSizeW(_: PCSTR, _: *mut u32) -> u32;
	pub fn GetFileVersionInfoW(_: PCSTR, _: u32, _: u32, _: PVOID) -> BOOL;
	pub fn VerQueryValueW(_: PCVOID, _: PCSTR, _: PVOID, _: *mut u32) -> BOOL;
}
